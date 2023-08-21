pub mod data_bridges;
pub mod translator;
pub mod type_converters;
use actix_cors::Cors;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use data_bridges::{LoginRequestBridge, ProfileBridge};
pub mod identity {
    tonic::include_proto!("identity"); // The string specified here must match the proto package name
}
use identity::authentication_service_client::AuthenticationServiceClient;
use identity::{
    AuthorizationCode, LoginRequest, Profile, SessionStatusRequest, TokenRequest,
    TransactionalToken,
};

static SERVER_URI: &str = "http://127.0.0.1:50051";

#[post("/newuser")]
async fn new_user(profile_json: web::Json<ProfileBridge>) -> impl Responder {
    println!("Receive request: {:?}", profile_json);
    let profile = Profile {
        username: profile_json.username.to_string(),
        password: profile_json.password.to_string(),
        email: profile_json.email.to_string(),
        fullname: profile_json.fullname.to_string(),
        phone_number: profile_json.fullname.to_string(),
        profile_picture_url: profile_json.profile_picture_url.to_string(),
    };

    let mut client = AuthenticationServiceClient::connect(SERVER_URI)
        .await
        .expect("Unable to connect to server");
    let request = tonic::Request::new(profile);
    let result = client.register(request).await;
    match result {
        Ok(response) => {
            let trans_token = response;
            let reply = format!("{:?}", trans_token);
            return HttpResponse::Ok().body(reply);
        }
        Err(err) => {
            let reply = format!("{:?}", err);
            return HttpResponse::PreconditionFailed().body(reply);
        }
    }
}

#[get("/useractivation/{token}")]
async fn user_activation(path: web::Path<(String,)>) -> impl Responder {
    let mut client = AuthenticationServiceClient::connect(SERVER_URI)
        .await
        .expect("Unable to connect to server");
    let str_token = path.into_inner().0;
    let request = tonic::Request::new(TransactionalToken { token: str_token });
    let result = client.validate_account(request).await;
    match result {
        Ok(response) => {
            let reply = format!("{:?}", response);
            return HttpResponse::Ok().body(reply);
        }
        Err(err) => {
            let reply = format!("{:?}", err);
            return HttpResponse::Unauthorized().body(reply);
        }
    }
}
#[post("/login")]
async fn login(cred_json: web::Json<LoginRequestBridge>) -> impl Responder {
    println!("Receive request: {:?}", cred_json);
    let mut client = AuthenticationServiceClient::connect(SERVER_URI)
        .await
        .expect("Unable to connect to server");
    let login_request = tonic::Request::new(LoginRequest {
        email: cred_json.email.clone(),
        password: cred_json.password.clone(),
        url: cred_json.url.clone(),
        ipv4: cred_json.ipv4.clone(),
        user_agen: cred_json.user_agen.clone(),
    });
    let result = client.login(login_request).await;
    match result {
        Ok(response) => {
            let reply = format!("{:?}", response);
            return HttpResponse::Ok().body(reply);
        }
        Err(err) => {
            let reply = format!("{:?}", err.message());
            return HttpResponse::Unauthorized().body(reply);
        }
    }
}

#[get("/session_status/{session_id}")]
async fn session_status(path: web::Path<(String,)>) -> impl Responder {
    let sid = path.into_inner().0;
    let mut client = AuthenticationServiceClient::connect(SERVER_URI)
        .await
        .expect("Unable to connect to server");
    let request = tonic::Request::new(SessionStatusRequest { session_id: sid });
    let result = client.get_session_status(request).await;
    match result {
        Ok(response) => {
            let reply = format!("{:?}", response);
            return HttpResponse::Ok().body(reply);
        }
        Err(err) => {
            return type_converters::from_tonic(err);
        }
    }
}
#[get("/token/{auth_code}")]
async fn token(path: web::Path<(String,)>) -> impl Responder {
    //let path_str = path.into_inner().0;
    let auth_code = path.into_inner().0;
    let request = tonic::Request::new(TokenRequest {
        auth_code: Some(AuthorizationCode { code: auth_code }),
    });
    //TODO
    let mut client = AuthenticationServiceClient::connect(SERVER_URI)
        .await
        .expect("Unable to connect to server");
    let result = client.get_token(request).await;
    match result {
        Ok(response) => {
            let reply = format!("{:?}", response);
            return HttpResponse::Ok().body(reply);
        }
        Err(err) => {
            return type_converters::from_tonic(err);
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .service(new_user)
            .service(user_activation)
            .service(login)
            .service(session_status)
            .service(token)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
