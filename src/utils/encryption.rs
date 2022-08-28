use bcrypt::DEFAULT_COST;
use error_stack::{IntoReport, ResultExt};

use crate::error;

pub async fn hash_password(password: String) -> error_stack::Result<String, error::Error> {
    let (send, recv) = tokio::sync::oneshot::channel();
    rayon::spawn(move || {
        let result = bcrypt::hash(password, DEFAULT_COST)
            .report()
            .change_context(error::Error::BcryptError);
        let _ = send.send(result);
    });
    recv.await
        .report()
        .change_context(error::Error::TokioRecvError)?
}

pub async fn verify_password(
    password: String,
    hash: String,
) -> error_stack::Result<bool, error::Error> {
    let (send, recv) = tokio::sync::oneshot::channel();
    rayon::spawn(move || {
        let result = bcrypt::verify(password, &hash)
            .report()
            .change_context(error::Error::BcryptError);
        let _ = send.send(result);
    });
    recv.await
        .report()
        .change_context(error::Error::TokioRecvError)?
}
