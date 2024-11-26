use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

use crate::{Document, Render};

impl IntoResponse for Document {
    fn into_response(self) -> axum::response::Response {
        match self.render_to_string() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
        }
    }
}
