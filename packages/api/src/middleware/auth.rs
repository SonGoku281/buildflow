use axum::{
    extract::Request,
    http::{HeaderValue, StatusCode},
    middleware::Next,
    response::Response,
};

// Supabase JWT verification middleware
pub async fn AuthMiddleware(
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .ok_or(StatusCode::UNAUTHORIZED)?
        .to_str()
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    if !auth_header.starts_with("Bearer ") {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // TODO: Verify JWT with Supabase JWT secret
    // let token = &auth_header[7..];
    // let decoded = jsonwebtoken::decode::<Claims>(token, &key, options)?;
    // req.extensions_mut().insert(decoded.claims);

    Ok(next.run(req).await)
}
