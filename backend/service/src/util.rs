use sea_orm::error::{DbErr, SqlErr};

pub fn unique_key_violation(err: &DbErr) -> bool {
    matches!(err.sql_err(), Some(SqlErr::UniqueConstraintViolation(_)))
}
