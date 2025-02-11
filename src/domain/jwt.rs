use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};

const ALGORYTHM: jsonwebtoken::Algorithm = jsonwebtoken::Algorithm::HS512;

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    pub exp: usize,
    pub uid: String,
}

pub fn encode_jwt(
    claims: JwtClaims,
    secret: &str,
) -> Result<std::string::String, jsonwebtoken::errors::Error> {
    encode(
        &Header::new(ALGORYTHM),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
}

pub fn decode_jwt(
    token: &str,
    secret: &str,
) -> Result<TokenData<JwtClaims>, jsonwebtoken::errors::Error> {
    decode::<JwtClaims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::new(ALGORYTHM),
    )
}
