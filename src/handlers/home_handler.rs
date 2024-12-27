use axum::{
    response::Html,
};
use crate::templates;

pub async fn index() -> Html<String> {
    let rendered = templates::render("home.html", None);
    Html(rendered)
}