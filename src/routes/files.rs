use actix_web::{HttpRequest, HttpResponse, Result as ActixResult, middleware, web};
use once_cell::sync::Lazy;

use crate::middlewares;
use crate::services::FileService;

// 懒加载的全局 FileService 实例
static FILE_SERVICE: Lazy<FileService> = Lazy::new(FileService::new_lazy);

pub async fn handle_upload(
    request: HttpRequest,
    payload: actix_multipart::Multipart,
) -> ActixResult<HttpResponse> {
    FILE_SERVICE.handle_upload(&request, payload).await
}

pub async fn handle_download(
    request: HttpRequest,
    file_id: web::Path<String>,
) -> ActixResult<HttpResponse> {
    FILE_SERVICE
        .handle_download(&request, file_id.into_inner())
        .await
}
// 配置路由
pub fn configure_file_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/files")
            .wrap(middlewares::RequireJWT)
            .wrap(middleware::Compress::default())
            .route("/upload", web::post().to(handle_upload))
            .route("/download/{file_id}", web::get().to(handle_download)),
    );
}
