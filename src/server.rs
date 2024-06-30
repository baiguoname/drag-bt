use axum::response::{IntoResponse, Response};
use axum::{
    extract::Path, 
    response::Json, 
    routing::get, 
    Router
};
// use reqwest::StatusCode;
use axum::http::StatusCode;
use super::types::FileResp;

pub async fn server() {
    let app = Router::new().route("/files/:name", get(get_file));

    let listsener = tokio::net::TcpListener::bind("localhost:8081")
        .await
        .unwrap();

    axum::serve(listsener, app).await.unwrap();
}

// async fn get_file(Path(name): Path<String>) -> Response {
//     let file_name = format!("./files/{}", name);
//     let content = std::fs::read_to_string(file_name).unwrap_or_default();
//     FileResp { content }.into_response()
// }

async fn get_file(Path(name): Path<String>) -> Json<FileResp> {
    let file_name = format!("./files/{}", name);
    let content = std::fs::read_to_string(file_name).unwrap_or_default();
    // FileResp { content }.into_response()
    Json(FileResp { content })
}

impl IntoResponse for FileResp {
    fn into_response(self) -> Response {
        let body = serde_json::to_string(&self).unwrap();
        Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "application/json")
            .body(body.into())
            .unwrap()
    }
}