use crate::middleware::AccessLogMiddleware;
use anyhow::*;
use async_std::net::ToSocketAddrs;

pub async fn serve(addr: impl ToSocketAddrs) -> Result<()> {
    let mut app = tide::new();
    app.middleware(AccessLogMiddleware::new());
    app.listen(addr).await?;
    Ok(())
}
