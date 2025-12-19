use sqlx::PgPool;

pub trait FromPool {
    fn from_pool(pool: PgPool) -> Self;
}
