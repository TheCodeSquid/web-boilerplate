use sea_orm::*;
use time::OffsetDateTime;

use crate::{crypto::password, entity::*, error::*, util::unique_key_violation};

// Query //

pub async fn find_by_id(id: i32, db: &impl ConnectionTrait) -> Result<Option<user::Model>, DbErr> {
    user::Entity::find_by_id(id).one(db).await
}

pub async fn find_by_username(
    username: &str,
    db: &impl ConnectionTrait,
) -> Result<Option<user::Model>, DbErr> {
    user::Entity::find()
        .filter(user::Column::Username.eq(username))
        .one(db)
        .await
}

// Creation //

pub async fn create_with_password(
    username: String,
    display_name: String,
    password: String,
    pepper: String,
    db: &impl TransactionTrait,
) -> Result<user::Model, SvcErr> {
    let tr = db.begin().await?;

    let now = OffsetDateTime::now_utc();

    let user = user::ActiveModel {
        id: NotSet,
        username: Set(username),
        display_name: Set(display_name),
        created: Set(now),
    }
    .insert(&tr)
    .await
    .map_err(|e| {
        if unique_key_violation(&e) {
            SvcErr::UsernameTaken
        } else {
            e.into()
        }
    })?;

    let hash = password::hash(password, pepper).await;
    password_auth::ActiveModel {
        user_id: Set(user.id),
        hash: Set(hash),
        queue_rehash: Set(false),
    }
    .insert(&tr)
    .await?;

    tr.commit().await?;
    info!(
        "created user \"{}\" (id {}) with password authentication",
        user.username, user.id
    );

    Ok(user)
}

// Login

pub async fn validate_password(
    username: String,
    password: String,
    pepper: String,
    prev_pepper: Option<String>,
    db: &impl TransactionTrait,
) -> Result<user::Model, SvcErr> {
    let tr = db.begin().await?;

    let user = find_by_username(&username, &tr)
        .await?
        .ok_or(SvcErr::InvalidLogin)?;
    let password_auth = user
        .find_related(password_auth::Entity)
        .one(&tr)
        .await?
        .ok_or(SvcErr::UnavailableLogin)?;

    let validation_pepper = if password_auth.queue_rehash {
        prev_pepper.as_ref().unwrap_or_else(|| {
            error!("previous pepper not provided for password rehash");
            &pepper
        })
    } else {
        &pepper
    };

    if password::verify(
        password.clone(),
        password_auth.hash.clone(),
        validation_pepper.clone(),
    )
    .await
    {
        if password_auth.queue_rehash {
            let mut model = password_auth.into_active_model();
            model.hash = Set(password::hash(password, pepper).await);
            model.queue_rehash = Set(false);
            model.save(&tr).await?;
        }

        Ok(user)
    } else {
        Err(SvcErr::InvalidLogin)
    }
}
