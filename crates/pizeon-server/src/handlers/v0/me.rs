// use axum::Json;
// use tracing::instrument;
// 
// use crate::handlers::ErrorResponseStatus;
// use crate::router::UserAuth;
// 
// use pizeon_common::api::*;
// 
// #[instrument(skip_all, fields(user.id = user.id))]
// pub async fn get(
//     UserAuth(user): UserAuth,
// ) -> Result<Json<MeResponse>, ErrorResponseStatus<'static>> {
//     Ok(Json(MeResponse {
//         username: user.username,
//     }))
// }
