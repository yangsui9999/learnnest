use crate::config::AppConfig;
use crate::error::AppError;
use crate::model::account::{Account, AccountResponse, Claims, LoginRequest, LoginResponse};
use crate::response::ApiResponse;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use jsonwebtoken::{EncodingKey, Header};
use salvo::writing::Json;
use salvo::{handler, Depot, Request};
use sqlx::PgPool;

#[handler]
pub async fn login(
    req: &mut Request,
    depot: &mut Depot,
) -> Result<Json<ApiResponse<LoginResponse>>, AppError> {
    // 1. 解析请求
    let input: LoginRequest = req
        .parse_json()
        .await
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    let pgpool = depot.obtain::<PgPool>().unwrap();

    // 2. 查询用户（根据 username）
    let account = sqlx::query_as!(
        Account,
        r#"select * from account where username = $1"#,
        input.username,
    )
    .fetch_one(pgpool)
    .await?;

    let hashed_pwd = account.password_hash.ok_or(AppError::PasswordHash)?;

    // 3. 验证密码（Argon2 verify）
    let hasher =
        PasswordHash::new(&hashed_pwd).map_err(|e| AppError::PasswordVerify(e.to_string()))?;
    Argon2::default()
        .verify_password(input.password.as_bytes(), &hasher)
        .map_err(|_| AppError::PasswordError)?;

    // 4. 生成 JWT token
    let add_date = chrono::Utc::now() + chrono::Duration::days(7);
    let claims = Claims {
        sub: account.id.to_string(),
        exp: add_date.timestamp() as usize,
    };

    let app_config = depot.obtain::<AppConfig>().unwrap();
    let jwt_secret = &app_config.jwt_secret;
    let token = jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )
    .map_err(|_| AppError::TokenError)?;

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
