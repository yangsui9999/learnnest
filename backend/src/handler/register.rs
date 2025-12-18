use crate::error::AppError;
use crate::model::account::{AccountResponse, RegisterRequest};
use crate::response::ApiResponse;
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use salvo::writing::Json;
use salvo::{handler, Depot, Request};
use sqlx::PgPool;

#[handler]
pub async fn register(
    req: &mut Request,
    depot: &mut Depot,
) -> Result<Json<ApiResponse<AccountResponse>>, AppError> {
    // 1. 解析请求体
    let input: RegisterRequest = req
        .parse_json()
        .await
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    // 2. 加密
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::default()
        .hash_password(input.password.as_bytes(), &salt)
        .map_err(|_| AppError::PasswordHash)?
        .to_string();

    // 先写死
    let role = "parent".to_string();
    let uid = uuid::Uuid::new_v4();
    let now = chrono::Utc::now();

    let pool = depot.obtain::<PgPool>().unwrap();

    // 3. 写入数据
    sqlx::query!(
        r#"INSERT INTO account(id, username, password_hash, nickname, role, created_at, created_by, updated_at, updated_by, is_deleted)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)"#,
        uid,
        input.username,
        password_hash,
        input.nickname,
        role,
        now,
        uid,
       now,
        uid,
        false,
    ).execute(pool).await?;

    // 4. 返回响应
    let response = AccountResponse {
        id: uid,
        username: Some(input.username),
        nickname: input.nickname,
    };

    Ok(Json(ApiResponse::success(response)))
}
