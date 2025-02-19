use axum::{async_trait, extract::FromRequestParts, http::request::Parts, RequestPartsExt};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};

pub const AUTHORIZATION_HEADER: &str = "Authorization";

use crate::domain::jwt::decode_jwt;
pub use crate::domain::jwt::JwtClaims;

#[async_trait]
impl<S> FromRequestParts<S> for JwtClaims
where
    S: Send + Sync,
{
    type Rejection = ();

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        match parts.extract::<TypedHeader<Authorization<Bearer>>>().await {
            Err(err) => {
                println!("parts.extract.Error: {:?}", err);
                Err(())
            }
            Ok(TypedHeader(Authorization(bearer))) => {
                match decode_jwt(bearer.token(), "secret11") {
                    Err(err) => {
                        println!("decode_jwt.Error: {:?}", err);
                        Err(())
                    }
                    Ok(data) => Ok(data.claims),
                }
            }
        }
    }
}
