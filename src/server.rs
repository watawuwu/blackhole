use actix_web::{App, HttpServer};
use anyhow::*;
use std::net::SocketAddr;

use crate::middleware::StructuredLogging;

pub async fn serve(addr: SocketAddr) -> Result<()> {
    HttpServer::new(|| App::new().wrap(StructuredLogging))
        .bind(addr)?
        .run()
        .await?;
    Ok(())
}
