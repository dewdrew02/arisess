use axum::{
    extract::Request,
    http::{StatusCode, header},
    middleware::Next,
    response::Response,
};

use crate::config::config_loader::get_user_secret;

pub async fn auth(
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let header: &str = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let token: &str = header
        .strip_prefix("Bearer ")
        .ok_or(StatusCode::UNAUTHORIZED)?
        .to_string();

    let secret: String =
        get_user_secret().map_err(|_| StatusCode::UNAUTHORIZED)?;

    // TODO: ตรวจสอบ token กับ secret
    let Claims = verify_token(&secret, token)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let _user_id  = Claims.sub.parse::<i32>()
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    req.extensions_mut().insert(val : user_id);

    Ok(next.run(req).await)
}
