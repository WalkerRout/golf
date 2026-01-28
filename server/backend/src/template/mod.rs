use askama::Template;

use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};

pub mod congeries;
pub mod error;
pub mod feed;
pub mod home;

/// Wrapper for us to return our templates in
pub struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
where
  T: Template,
{
  fn into_response(self) -> Response {
    match self.0.render() {
      Ok(html) => Html(html).into_response(),
      Err(err) => (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Failed to render template. Error: {err}"),
      )
        .into_response(),
    }
  }
}

impl<T> From<T> for HtmlTemplate<T>
where
  T: Template,
{
  fn from(template: T) -> Self {
    Self(template)
  }
}
