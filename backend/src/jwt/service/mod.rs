pub mod errors;

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::utils::env::get_var;

use self::errors::JwtServiceError;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub struct JwtService {
    pub secret_key: String,
}

impl JwtService {
    pub fn new() -> Self {
        let secret_key = get_var("SECRET_KEY").unwrap_or_else(|| "tcc_secret_key".to_string());
        Self { secret_key }
    }

    pub fn create_token<T>(&self, t: &T, exp: usize) -> Result<String, JwtServiceError>
    where
        T: Serialize,
    {
        let stringified_t = serde_json::to_string(t).map_err(|err| {
            JwtServiceError::Unknown(anyhow::anyhow!(
                "Failed to stringify token data err: {}",
                err
            ))
        })?;

        let claims = Claims {
            sub: stringified_t,
            exp,
        };
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret_key.as_ref()),
        )
        .map_err(|err| JwtServiceError::from(err.kind()))?;
        Ok(token)
    }

    pub fn decode_token<T>(&self, token: &str) -> Result<T, JwtServiceError>
    where
        T: DeserializeOwned,
    {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret_key.as_ref()),
            &Validation::default(),
        )
        .map_err(|err| JwtServiceError::from(err.kind()))?;

        let data = serde_json::from_str::<T>(&token_data.claims.sub).map_err(|err| {
            JwtServiceError::Unknown(anyhow::anyhow!(
                "Failed to deserialize token data err: {}",
                err
            ))
        })?;

        Ok(data)
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Duration, Utc};

    use super::*;

    #[derive(Debug, Serialize, Deserialize)]
    struct TestToken {
        test: String,
    }

    #[test]
    fn should_create_and_decode_token() {
        let jwt_service = JwtService::new();
        let test_token = TestToken {
            test: "test".to_string(),
        };
        let token = jwt_service
            .create_token(
                &test_token,
                (Utc::now() + Duration::hours(1)).timestamp() as usize,
            )
            .unwrap();
        let decoded_token = jwt_service.decode_token::<TestToken>(&token).unwrap();

        assert_eq!(decoded_token.test, test_token.test);
    }

    #[test]
    fn should_return_correct_errors() {
        let jwt_service = JwtService::new();
        let test_token = TestToken {
            test: "test".to_string(),
        };

        let token = jwt_service.create_token(&test_token, 1 as usize).unwrap();

        assert!(matches!(
            jwt_service.decode_token::<TestToken>(&token).err().unwrap(),
            JwtServiceError::ExpiredToken
        ));

        let invalid_token = "invalid_token".to_owned();

        assert!(matches!(
            jwt_service
                .decode_token::<TestToken>(&invalid_token)
                .err()
                .unwrap(),
            JwtServiceError::InvalidToken
        ));
    }
}
