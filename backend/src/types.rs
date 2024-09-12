// This is only for global types that have no association with e.g. exclusively patient endpoints

use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct ApiResponse<T> {
    pub data: T,
}
