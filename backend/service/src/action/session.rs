use std::time::Duration;

use sea_orm::*;
use time::OffsetDateTime;

use crate::{crypto::token, entity::*, error::*};

pub async fn find_by_id(
    id: i32,
    db: &impl ConnectionTrait,
) -> Result<Option<session::Model>, DbErr> {
    session::Entity::find_by_id(id).one(db).await
}

pub async fn create(
    user: &user::Model,
    session_secret: &str,
    db: &impl ConnectionTrait,
) -> Result<String, DbErr> {
    let session = session::ActiveModel {
        id: NotSet,
        user_id: Set(user.id),
        start: Set(OffsetDateTime::now_utc()),
    }
    .insert(db)
    .await?;

    let bytes = session.id.to_le_bytes();
    Ok(token::sign(&bytes, session_secret))
}

pub async fn verify(
    token: &str,
    session_secret: &str,
    db: &impl ConnectionTrait,
) -> Result<user::Model, SvcErr> {
    let bytes = token::verify(token, session_secret).ok_or(SvcErr::NoSession)?;
    let bytes = bytes.try_into().map_err(|_| SvcErr::NoSession)?;
    let id = i32::from_le_bytes(bytes);

    let session = find_by_id(id, db).await?.ok_or(SvcErr::NoSession)?;
    let user = session
        .find_related(user::Entity)
        .one(db)
        .await?
        .ok_or_else(|| {
            error!(
                "session (id {}) without associated user (id {})",
                session.id, session.user_id
            );
            SvcErr::NoSession
        })?;
    Ok(user)
}

pub async fn prune(lifetime: Duration, db: &impl ConnectionTrait) -> Result<(), DbErr> {
    let oldest = OffsetDateTime::now_utc() - lifetime;

    let result = session::Entity::delete_many()
        .filter(session::Column::Start.lte(oldest))
        .exec(db)
        .await?;

    if result.rows_affected > 0 {
        debug!("pruned {} expired sessions", result.rows_affected);
    }

    Ok(())
}
