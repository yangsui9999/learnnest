use crate::model::account::Claims;
use salvo::jwt_auth::{ConstDecoder, HeaderFinder};
use salvo::prelude::JwtAuth;

pub fn create_jwt_auth(secret: &str) -> JwtAuth<Claims, ConstDecoder> {
    JwtAuth::new(ConstDecoder::from_secret(secret.as_bytes()))
        .finders(vec![Box::new(HeaderFinder::new())])
        .force_passed(false)
}