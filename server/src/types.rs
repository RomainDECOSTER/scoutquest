use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct OkResponse {
    pub status: &'static str,
}

impl OkResponse {
    pub fn new() -> Self {
        Self {
            status: "ok",
        }
    }
}