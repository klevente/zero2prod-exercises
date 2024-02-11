use actix_session::{Session, SessionExt, SessionGetError, SessionInsertError};
use actix_web::{dev::Payload, FromRequest, HttpRequest};
use std::future::{ready, Ready};
use uuid::Uuid;

pub struct TypedSession(Session);

impl TypedSession {
    const USER_ID_KEY: &'static str = "user_id";

    pub fn renew(&self) {
        self.0.renew();
    }

    pub fn insert_user_id(&self, user_id: Uuid) -> Result<(), SessionInsertError> {
        self.0.insert(Self::USER_ID_KEY, user_id)
    }

    pub fn get_user_id(&self) -> Result<Option<Uuid>, SessionGetError> {
        self.0.get(Self::USER_ID_KEY)
    }

    pub fn log_out(self) {
        self.0.purge()
    }
}

impl FromRequest for TypedSession {
    // `TypedSession`'s `FromRequest` impl returns the same error as
    // `Session`'s `FromRequest` impl
    type Error = <Session as FromRequest>::Error;

    // Rust does not fully support `async fn` in traits, but `FromRequest` expects
    // a `Future` to be returned from `from_request`, to allow extractors to perform
    // async operations.
    // Here we don't need to, so we use `Ready`, a type that implements `Future`,
    // and is immediately resolved the first time it's polled.
    type Future = Ready<Result<TypedSession, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        // build the `Ready` `Future` using `ready`
        ready(Ok(TypedSession(req.get_session())))
    }
}
