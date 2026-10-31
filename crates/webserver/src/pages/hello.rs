use axum::response::Html;

pub(crate) async fn hello() -> Html<&'static str> {
    Html("<h1>Hello from Orin Nano!!!</h1>")
}
