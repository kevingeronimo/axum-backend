use crate::error::{Error, Result};
use bcrypt::DEFAULT_COST;
use error_stack::{IntoReport, ResultExt};

pub async fn hash_password(password: String) -> Result<String> {
    let (send, recv) = tokio::sync::oneshot::channel();
    rayon::spawn(move || {
        let result = bcrypt::hash(password, DEFAULT_COST)
            .report()
            .change_context(Error::BcryptError);
        let _ = send.send(result);
    });
    recv.await.report().change_context(Error::TokioRecvError)?
}

pub async fn verify_password(password: String, hash: String) -> Result<bool> {
    let (send, recv) = tokio::sync::oneshot::channel();
    rayon::spawn(move || {
        let result = bcrypt::verify(password, &hash)
            .report()
            .change_context(Error::BcryptError);
        let _ = send.send(result);
    });
    recv.await.report().change_context(Error::TokioRecvError)?
}
