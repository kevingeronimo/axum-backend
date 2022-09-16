use bcrypt::DEFAULT_COST;

pub async fn hash_password(password: String) -> anyhow::Result<String> {
    let (send, recv) = tokio::sync::oneshot::channel();
    rayon::spawn(move || {
        let result = bcrypt::hash(password, DEFAULT_COST);
        let _ = send.send(result);
    });
    recv.await
        .map_err(|e| e.into())
        .and_then(|result| result.map_err(|e| e.into()))
}

pub async fn verify_password(password: String, hash: String) -> anyhow::Result<bool> {
    let (send, recv) = tokio::sync::oneshot::channel();
    rayon::spawn(move || {
        let result = bcrypt::verify(password, &hash);
        let _ = send.send(result);
    });
    recv.await
        .map_err(|e| e.into())
        .and_then(|result| result.map_err(|e| e.into()))
}
