#[macro_use]
extern crate log;
extern crate bcrypt;

pub mod authorization;
pub mod config;
pub mod data;
pub mod email;
pub mod general;
pub mod input_validation;
pub mod login;
pub mod models;
pub mod redis_data;
pub mod registration;
pub mod schema;
pub mod token;
use crate::authorization::AuthCode;
use crate::data::{
    establish_connection,
    session::{get_session, update_last_access},
};
use crate::email::{send_mail, EmailAccount};
use crate::registration::{confirmed_email, register_user};
use crate::token::{extract_token, generate_jwt_token, invalidate_token, verify_jwt_token};
use config::get_env_var;
use log::{error, info};
use tonic::Code;
use tonic::{transport::Server, Request, Response, Status};

pub mod identity {
    tonic::include_proto!("identity"); // The string specified here must match the proto package name
}

use identity::authentication_service_server::{AuthenticationService, AuthenticationServiceServer};
use identity::{
    AuthorizationCode, Email, LoginRequest, LoginResponse, LogoutRequest, MfaCode, Profile,
    ProfilePublic, ResetPasswordReq, SessionStatus, SessionStatusRequest, Token, TokenRequest,
    TokenValidation, TransactionalToken,
};

use jsonwebtoken::errors::Error as JwtError;
use jsonwebtoken::{decode, DecodingKey, Validation};

extern crate serde;
#[macro_use]
extern crate serde_derive;

const TTL_ACCESS: i64 = 60; //1 Hour
const TTL_REFRESH: i64 = 2880; //2 Days

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
}

// Placeholder function to validate a JWT
fn validate_jwt(token: &str) -> Result<bool, JwtError> {
    let validation = Validation::default();
    match decode::<Claims>(
        &token,
        &DecodingKey::from_secret("secret".as_ref()),
        &validation,
    ) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

const CONFIRM_ENDPOINT: &str = "http://localhost:5173/confirm_email";

#[derive(Debug, Default)]
pub struct MyAuthenticationService;

#[tonic::async_trait]
impl AuthenticationService for MyAuthenticationService {
    async fn register(
        &self,
        request: Request<Profile>,
    ) -> Result<Response<TransactionalToken>, Status> {
        //INSERT USER IF TOKEN IS CREATED
        let agen_req = request.into_inner();
        let private_key_base64 = get_env_var("ACCESS_TOKEN_PRIVATE_KEY");
        let result = generate_jwt_token(agen_req.email.to_string(), TTL_ACCESS, private_key_base64);
        match result {
            Ok(token_detail) => {
                match register_user(
                    &agen_req.username,
                    &agen_req.password,
                    &agen_req.email,
                    &agen_req.fullname,
                    &agen_req.phone_number,
                    &agen_req.profile_picture_url,
                ) {
                    Ok(new_user) => {
                        info!(
                            "\nSaved user {} with id {}",
                            new_user.username, new_user.user_id
                        );
                        let token_str = token_detail.token.unwrap_or(String::from(""));
                        send_mail(
                            EmailAccount {
                                first_name: String::from(&agen_req.username),
                                email_address: String::from(&agen_req.email),
                            },
                            "Email Confirmation",
                            "verification_email",
                            format!("{}/{}", CONFIRM_ENDPOINT, &token_str).as_str(),
                        );
                        return Ok(Response::new(TransactionalToken { token: token_str }));
                    }
                    Err(err) => {
                        error!("Failed register user: {:?}", err);
                        return Err(Status::new(Code::InvalidArgument, format!("{:?}", err)));
                    }
                };
            }
            Err(err) => {
                error!("Failed creating token for user: {:?}", &agen_req.username);
                return Err(Status::new(Code::InvalidArgument, format!("{:?}", err)));
            }
        };
    }
    async fn validate_account(
        &self,
        request: Request<TransactionalToken>,
    ) -> Result<Response<TokenValidation>, Status> {
        // Validate JWT
        let transaction = request.into_inner();
        let pk64 = get_env_var("ACCESS_TOKEN_PUBLIC_KEY");
        let public_key_base64 = pk64.clone();

        let verify_result = verify_jwt_token(public_key_base64, &transaction.token);
        match verify_result {
            Ok(token_detail) => {
                let user_email = &token_detail.user_id;
                if confirmed_email(user_email) {
                    info!("User {} confirmed his/her email!", user_email);
                    if let Ok(extracted_token) = extract_token(pk64, &transaction.token) {
                        invalidate_token(extracted_token);
                    }

                    return Ok(Response::new(TokenValidation { valid: true }));
                }
                return Err(Status::internal(
                    format!("Database failed to acivate user",),
                ));
            }
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal(format!(
                    "Failed to validate JWT:{:?}",
                    err
                )));
            }
        };
    }
    async fn get_profile(
        &self,
        request: Request<Token>,
    ) -> Result<Response<ProfilePublic>, Status> {
        // TODO: implement
        let request = request.into_inner();
        let reply = ProfilePublic {
            username: "access123".into(),
            email: "refresh123".into(),
            fullname: "".into(),
            phone_number: "".into(),
            profile_picture_url: "".into(),
        };
        Ok(Response::new(reply))
    }
    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        let agen_req = request.into_inner();
        let user_email = agen_req.email.as_str();
        let user_password = agen_req.password.as_str();
        let user_ipv4 = agen_req.ipv4.as_str();
        let user_agen = agen_req.user_agen.as_str();
        let mut db_conn = establish_connection();
        //Authenticate credentials
        let auth_result = crate::login::authenticate(&mut db_conn, user_email, user_password);
        match auth_result {
            Ok(user) => {
                //Create session
                let session_result =
                    crate::login::create_session(&mut db_conn, user.user_id, user_ipv4, user_agen);
                match session_result {
                    Ok(sid) => {
                        //Generate and save authorization code
                        let auth_code = AuthCode::new(user_email).get_code();

                        //Serve response with session id and authorization code
                        let reply = LoginResponse {
                            session_id: sid,
                            auth_code: Some(AuthorizationCode {
                                code: String::from(auth_code),
                            }),
                            url: String::from(""),
                        };
                        Ok(Response::new(reply))
                    }
                    Err(err) => {
                        return Err(Status::internal(format!(
                            "Failed create session: {:?}",
                            err
                        )));
                    }
                }
            }
            Err(err) => return Err(Status::invalid_argument(format!("{:?}", err))),
        }
    }
    async fn get_session_status(
        &self,
        request: tonic::Request<SessionStatusRequest>,
    ) -> std::result::Result<tonic::Response<SessionStatus>, tonic::Status> {
        let request = request.into_inner();
        let sid = request.session_id;
        let mut db_conn = establish_connection();

        if let Some(_) = get_session(&mut db_conn, sid.as_str()) {
            if update_last_access(&mut db_conn, sid.as_str()) {
                return Ok(Response::new(SessionStatus { active: false }));
            }
            return Err(Status::new(
                Code::Internal,
                format!("Server failed to update 'last access' of the session!"),
            ));
        }

        return Err(Status::new(
            Code::InvalidArgument,
            format!("No session for provided session id!"),
        ));
    }
    async fn get_token(&self, request: Request<TokenRequest>) -> Result<Response<Token>, Status> {
        // TODO: implement
        let user_req = request.into_inner();

        if let Some(auth_code) = user_req.auth_code {
            let auth_code = AuthCode::from(auth_code.code.as_str());
            if auth_code.is_valid() {
                let acc_private_key_base64 = crate::config::get_env_var("ACCESS_TOKEN_PRIVATE_KEY");
                let ref_private_key_base64 = acc_private_key_base64.clone();

                let mut access_token = String::from("");
                if let Ok(token_detail) =
                    generate_jwt_token(auth_code.get_email(), TTL_ACCESS, acc_private_key_base64)
                {
                    access_token = token_detail.token.unwrap();
                }
                let mut refresh_token = String::from("");
                if let Ok(token_detail) =
                    generate_jwt_token(auth_code.get_email(), TTL_REFRESH, ref_private_key_base64)
                {
                    refresh_token = token_detail.token.unwrap();
                }
                let token = Token {
                    access_token,
                    refresh_token,
                };
                return Ok(Response::new(token));
            }
        }
        return Err(Status::new(
            Code::InvalidArgument,
            format!("Invalid token request"),
        ));
    }
    async fn refresh_token(&self, request: Request<Token>) -> Result<Response<Token>, Status> {
        // TODO: implement actual logic
        println!("Received RefreshToken request {:?}", request);

        let reply = Token {
            access_token: "access123".into(),
            refresh_token: "refresh123".into(),
        };
        Ok(Response::new(reply))
    }
    async fn forgot_password(&self, request: Request<Email>) -> Result<Response<()>, Status> {
        // TODO: implement actual logic
        println!("Received ForgotPassword request {:?}", request);

        let reply = ();
        Ok(Response::new(reply))
    }
    async fn reset_password(
        &self,
        request: Request<ResetPasswordReq>,
    ) -> Result<Response<()>, Status> {
        // TODO: implement actual logic
        println!("Received ResetPassword request {:?}", request);

        let reply = ();
        Ok(Response::new(reply))
    }
    async fn logout(&self, request: Request<LogoutRequest>) -> Result<Response<()>, Status> {
        // TODO: implement actual logic
        println!("Received Logout request {:?}", request);

        let reply = ();
        Ok(Response::new(reply))
    }
    async fn send_mfa_code(&self, request: Request<Token>) -> Result<Response<()>, Status> {
        // TODO: implement actual logic
        println!("Received SendMfaCode request {:?}", request);

        let reply = ();
        Ok(Response::new(reply))
    }
    async fn verify_mfa_code(&self, request: Request<MfaCode>) -> Result<Response<()>, Status> {
        // TODO: implement actual logic
        println!("Received VerifyMfaCode request {:?}", request);

        let reply = ();
        Ok(Response::new(reply))
    }
    async fn validate_token(
        &self,
        request: Request<Token>,
    ) -> Result<Response<TokenValidation>, Status> {
        // TODO: implement
        let request = request.into_inner();

        // Validate JWT
        match validate_jwt(&request.access_token) {
            Ok(valid) => {
                // Construct response
                let reply = TokenValidation { valid };
                Ok(Response::new(reply))
            }
            Err(_) => Err(Status::internal("Failed to validate JWT")),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let addr = "127.0.0.1:50051".parse()?;
    let authentication_service = MyAuthenticationService::default();

    Server::builder()
        .accept_http1(true)
        .add_service(tonic_web::enable(AuthenticationServiceServer::new(
            authentication_service,
        )))
        .serve(addr)
        .await?;

    Ok(())
}
