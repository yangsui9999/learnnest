use crate::error::AppError;
use crate::handler::DepotExt;
use crate::model::account::{AccountCreate, AccountResponse, RegisterRequest};
use crate::response::{ApiResponse, ApiResult};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use salvo::writing::Json;
use salvo::{handler, Depot, Request};

#[handler]
pub async fn register(req: &mut Request, depot: &mut Depot) -> ApiResult<AccountResponse> {
    // 1. 解析请求体
    let input: RegisterRequest = req
        .parse_json()
        .await
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    // 2. 加密
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::default()
        .hash_password(input.password.as_bytes(), &salt)
        .map_err(|_| AppError::Internal)?
        .to_string();

    // 先写死
    // todo 写死的 role
    let role = "parent".to_string();
    let uid = uuid::Uuid::new_v4();
    let now = chrono::Utc::now();

    // 3. 写入数据
    let account_create = AccountCreate {
        id: uid,
        username: Some(input.username.clone()),
        nickname: input.nickname.clone(),
        password_hash: Some(password_hash),
        role,
        created_at: now,
        updated_at: now,
        created_by: uid,
        updated_by: uid,
        is_deleted: false,
    };
    let ctx = depot.app_context()?;
    ctx.services.account.create(account_create).await?;

    // 4. 返回响应
    let response = AccountResponse {
        id: uid,
        username: Some(input.username.clone()),
        nickname: input.nickname.clone(),
    };

    Ok(Json(ApiResponse::success(response)))
}
