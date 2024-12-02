use axum_core::response::IntoResponse;
use http::{header, HeaderValue, StatusCode};

use crate::{Document, Render};

// https://github.com/hyperium/mime/blob/ce5062d216bf757a0ed3fc70f0fe255d1c8d74ae/src/lib.rs#L753
const TEXT_HTML_UTF_8: &str = "text/html; charset=utf-8";

impl IntoResponse for Document {
    fn into_response(self) -> axum_core::response::Response {
        match self.render_to_string() {
            // Keeping dependency churn low by manually reimplementing
            // https://github.com/tokio-rs/axum/blob/b5a01092216d0fa5ab950cbd7030ebcc925ceb33/axum/src/response/mod.rs#L40-L54
            Ok(html) => (
                [(
                    header::CONTENT_TYPE,
                    HeaderValue::from_static(TEXT_HTML_UTF_8),
                )],
                html,
            )
                .into_response(),

            Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
        }
    }
}
