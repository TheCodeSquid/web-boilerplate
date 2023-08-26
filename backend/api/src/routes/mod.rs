mod login;
mod signup;

use crate::state::ApiRouter;

pub fn routes() -> ApiRouter {
    ApiRouter::new()
        .nest("/signup", signup::routes())
        .nest("/login", login::routes())
}
