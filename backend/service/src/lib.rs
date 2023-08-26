#[macro_use]
extern crate tracing;

pub mod action;
pub mod config;
pub mod crypto;
pub mod entity;
pub mod error;
pub mod util;

pub use sea_orm;

pub mod prelude {
    pub use sea_orm::*;

    pub use crate::error::*;

    pub mod user {
        pub use crate::{action::user::*, entity::user::*};
    }

    pub mod session {
        pub use crate::{action::session::*, entity::session::*};
    }

    pub mod password_auth {
        pub use crate::entity::password_auth::*;
    }
}
