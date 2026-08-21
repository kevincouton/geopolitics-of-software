use axum::async_trait;
use axum::extract::{FromRequest, FromRequestParts, Request};
use axum::http::request::Parts;
use chassis::error::ApiError;
use serde::de::DeserializeOwned;

/// JSON extractor that always renders rejection as an `ApiError::BadRequest` JSON response.
#[derive(Debug)]
pub struct Json<T>(pub T);

#[async_trait]
impl<T, S> FromRequest<S> for Json<T>
where
    T: DeserializeOwned + Send,
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        match axum::Json::<T>::from_request(req, state).await {
            Ok(axum::Json(inner)) => Ok(Json(inner)),
            Err(_) => Err(ApiError::BadRequest),
        }
    }
}

/// Query extractor that always renders rejection as an `ApiError::BadRequest` JSON response.
#[derive(Debug)]
pub struct Query<T>(pub T);

#[async_trait]
impl<T, S> FromRequestParts<S> for Query<T>
where
    T: DeserializeOwned + Send,
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match axum::extract::Query::<T>::from_request_parts(parts, state).await {
            Ok(axum::extract::Query(inner)) => Ok(Query(inner)),
            Err(_) => Err(ApiError::BadRequest),
        }
    }
}
