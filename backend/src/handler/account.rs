use crate::error::AppError;
use crate::handler::DepotExt;
use crate::model::account::{AccountResponse, Claims, LoginRequest, LoginResponse};
use crate::response::{ApiResponse, ApiResult};
use argon2::password_hash::Error;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use jsonwebtoken::{EncodingKey, Header};
use salvo::writing::Json;
use salvo::{handler, Depot, Request};

#[handler]
pub async fn login(req: &mut Request, depot: &mut Depot) -> ApiResult<LoginResponse> {
    // 1. 解析请求
    let input: LoginRequest = req
        .parse_json()
        .await
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    let ctx = depot.app_context()?;
    let account = ctx.services.account.get(input.username).await?;

    let hashed_pwd = account.password_hash.ok_or(AppError::Internal)?;

    // 3. 验证密码（Argon2 verify）
    let hasher = PasswordHash::new(&hashed_pwd).map_err(|e| {
        tracing::error!(err = ?e, "密码哈希解析失败");
        AppError::Internal
    })?;
    Argon2::default()
        .verify_password(input.password.as_bytes(), &hasher)
        .map_err(|e| match e {
            Error::Password => AppError::PasswordError,
            _ => AppError::Internal,
        })?;

    // 4. 生成 JWT token
    let add_date = chrono::Utc::now() + chrono::Duration::days(7);
    let claims = Claims {
        sub: account.id.to_string(),
        exp: add_date.timestamp() as usize,
    };

    let app_config = depot.app_config()?;
    let token = jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(&app_config.jwt_secret.as_bytes()),
    )
    .map_err(|_| AppError::Internal)?;

    // 5. 返回响应
    let login_resp = LoginResponse {
        access_token: token,
        account: AccountResponse {
            id: account.id,
            username: account.username,
            nickname: account.nickname,
        },
    };

    Ok(Json(ApiResponse::success(login_resp)))
}
