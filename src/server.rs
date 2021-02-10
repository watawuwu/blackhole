use crate::middleware::AccessLogMiddleware;
use anyhow::*;
use std::net::ToSocketAddrs;

pub async fn serve(addr: impl ToSocketAddrs) -> Result<()> {
    let mut app = tide::new();
    app.with(AccessLogMiddleware::new());
    app.listen(addr.to_socket_addrs()?.collect::<Vec<_>>())
        .await?;
    Ok(())
}
