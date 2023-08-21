extern crate base64;
use crate::redis_data;
use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenDetails {
    pub token: Option<String>,
    pub token_uuid: String,
    pub user_id: String,
    pub expires_in: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub token_uuid: String,
    pub exp: i64,
    pub iat: i64,
    pub nbf: i64,
}

pub fn invalidate_token(token_detail: TokenDetails) {
    let user_token = &token_detail.token.unwrap_or(String::from(""));
    let current_timestamp = chrono::Utc::now().timestamp();
    println!("expires_in = {:?}", token_detail.expires_in);
    let ttl_seconds = if let Some(expiration_timestamp) = token_detail.expires_in {
        let duration = expiration_timestamp - current_timestamp;
        duration as usize
    } else {
        0
    };
    if let Ok(_) = redis_data::fetch_str(&token_detail.token_uuid) {
        info!("Inactivated token added to redis!");
        println!(
            "Inactivated token added to redis: {}",
            &token_detail.token_uuid
        );
    } else {
        match redis_data::push_str(&token_detail.token_uuid, &user_token, ttl_seconds) {
            Ok(_) => {
                info!("Inactivated token added to redis!");
                println!("Inactivated token added to redis!");
                //return true;
            }
            Err(err) => {
                error!("Redis failed. Error: {}, ttl_seconds: {}", err, ttl_seconds);
                //return false;
            }
        }
    }
}
pub fn generate_jwt_token(
    user_id: String,
    ttl: i64,
    private_key: String,
) -> Result<TokenDetails, jsonwebtoken::errors::Error> {
    let bytes_private_key = general_purpose::STANDARD.decode(private_key).unwrap();
    let decoded_private_key = String::from_utf8(bytes_private_key).unwrap();

    let now = chrono::Utc::now();
    let mut token_details = TokenDetails {
        user_id,
        token_uuid: Uuid::new_v4().to_string(),
        expires_in: Some((now + chrono::Duration::minutes(ttl)).timestamp()),
        token: None,
    };

    let claims = TokenClaims {
        sub: token_details.user_id.to_string(),
        token_uuid: token_details.token_uuid.clone(),
        exp: token_details.expires_in.unwrap(),
        iat: now.timestamp(),
        nbf: now.timestamp(),
    };

    let header = jsonwebtoken::Header::new(jsonwebtoken::Algorithm::RS256);
    let token = jsonwebtoken::encode(
        &header,
        &claims,
        &jsonwebtoken::EncodingKey::from_rsa_pem(decoded_private_key.as_bytes())?,
    )?;
    token_details.token = Some(token);
    Ok(token_details)
}

pub fn verify_jwt_token(
    public_key: String,
    token: &str,
) -> Result<TokenDetails, jsonwebtoken::errors::Error> {
    println!("###TOKEN VERIVICATION CALLED###");
    let bytes_public_key = general_purpose::STANDARD.decode(public_key).unwrap();
    let decoded_public_key = String::from_utf8(bytes_public_key).unwrap();

    let validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::RS256);

    let decoded = jsonwebtoken::decode::<TokenClaims>(
        token,
        &jsonwebtoken::DecodingKey::from_rsa_pem(decoded_public_key.as_bytes())?,
        &validation,
    )?;
    //Check if token is stored in invalidated list in redis then revoked
    let token_uuid = decoded.claims.token_uuid.as_str().to_owned();
    if let Ok(_) = redis_data::fetch_str(token_uuid.as_str()) {
        println!("token exist:{}", token_uuid.as_str());
        let err = jsonwebtoken::errors::ErrorKind::ExpiredSignature;
        return Err(jsonwebtoken::errors::Error::from(err));
    }
    let user_id = match decoded.claims.sub.parse::<String>() {
        Ok(i) => i,
        Err(_e) => "".to_string(),
    };

    Ok(TokenDetails {
        token: None,
        token_uuid,
        user_id,
        expires_in: None,
    })
}
pub fn extract_token(
    public_key: String,
    token: &str,
) -> Result<TokenDetails, jsonwebtoken::errors::Error> {
    let bytes_public_key = general_purpose::STANDARD.decode(public_key).unwrap();
    let decoded_public_key = String::from_utf8(bytes_public_key).unwrap();

    let validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::RS256);

    let decoded = jsonwebtoken::decode::<TokenClaims>(
        token,
        &jsonwebtoken::DecodingKey::from_rsa_pem(decoded_public_key.as_bytes())?,
        &validation,
    )?;
    let user_id = match decoded.claims.sub.parse::<String>() {
        Ok(i) => i,
        Err(_e) => "".to_string(),
    };
    let exp = decoded.claims.exp;

    let token_uuid = decoded.claims.token_uuid.as_str().to_owned();

    Ok(TokenDetails {
        token: Some(token.to_string()),
        token_uuid,
        user_id,
        expires_in: Some(exp),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use dotenvy::dotenv;
    use std::env;

    static USER_ID: &str = "123";
    const TTL: i64 = 60; // 60 minutes

    #[test]
    fn test_generate_and_verify_jwt_token() {
        dotenv().ok(); // Load environment variables from .env file
        let private_key_base64 = env::var("ACCESS_TOKEN_PRIVATE_KEY")
            .expect("ACCESS_TOKEN_PRIVATE_KEY must be set in the .env file");
        let public_key_base64 = env::var("ACCESS_TOKEN_PUBLIC_KEY")
            .expect("ACCESS_TOKEN_PUBLIC_KEY must be set in the .env file");

        let result = generate_jwt_token(String::from(USER_ID), TTL, private_key_base64);
        assert!(result.is_ok(), "Failed to generate JWT token");

        let token_details = result.unwrap();
        let token = token_details.token.unwrap();

        let verify_result = verify_jwt_token(public_key_base64, &token);
        assert!(verify_result.is_ok(), "Failed to verify JWT token");

        let verified_token_details = verify_result.unwrap();
        assert_eq!(verified_token_details.user_id, USER_ID);
        assert_eq!(verified_token_details.token_uuid, token_details.token_uuid);
    }

    #[test]
    fn test_token_expiry() {
        dotenv().ok(); // Load environment variables from .env file
        let private_key_base64 = env::var("ACCESS_TOKEN_PRIVATE_KEY")
            .expect("ACCESS_TOKEN_PRIVATE_KEY must be set in the .env file");

        let result = generate_jwt_token(String::from(USER_ID), TTL, private_key_base64);
        assert!(result.is_ok(), "Failed to generate JWT token");

        let token_details = result.unwrap();
        assert!(token_details.expires_in.unwrap() > Utc::now().timestamp());
    }
}
