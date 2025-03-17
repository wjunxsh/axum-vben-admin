use axum::{
    body::{to_bytes, Body}, extract::{MatchedPath, State}, http::{Request, StatusCode}, middleware::Next, response::Response
};
use bytes::{Buf, Bytes};
use chrono::Utc;
use serde_json::Value;
use std::sync::Arc;
use crate::{app_state:: AppState, entity::system_admin_log::SystemAdminLog, service::{admin_log::AdminLogService, api::ApiService, claims::Claims, ip::get_remote_ip, jwt::ClaimUser}};
pub async fn log_middleware(
    State(app_state): State<Arc<AppState>>,
    req: Request<Body>,
    next: Next,) -> Result<Response, StatusCode> {
    // 获取路由 path
    let path = req.uri().path().to_string();
    let matched_path = req.extensions()
       .get::<MatchedPath>()
       .map(|p| p.as_str().to_string())
       .unwrap_or_else(|| path.clone());
    
    //记录操作开始时间
    let start = Utc::now().timestamp_millis();
    //获取请求的完整url
    let url = req.uri().to_string();
    // 获取 user
    let user = if let Some(claims) = req.extensions().get::<Claims<Arc<ClaimUser>>>() {
        let user = claims.0.clone();
        user
    } else {
        // 如果没有找到用户信息，返回 401 Unauthorized
        return Err(StatusCode::UNAUTHORIZED);
    };
    let ip = get_remote_ip(&req.headers());
    let mut system_admin_log = SystemAdminLog {
        system_user_id: user.user_id,
        system_real_name: user.real_name.clone(),
        path: matched_path.clone(),
        method: req.method().to_string(),
        user_agent: req.headers().get("User-Agent").unwrap().to_str().unwrap().to_string(),
        request_url: url,
        request_body: None,
        response_data: None,
        response_status: 0,
        response_time_ms: 0,
        operate_name: "".to_string(),
        error_message: "".to_string(),
        ip:  ip,
        created_at: Utc::now().timestamp() as u64,
        id: 0,
    };
    let api_info = ApiService::get_api_by_path(&app_state, &matched_path).await;
    match api_info {
        Ok(api) => {
            system_admin_log.operate_name = api.description;
        }
        Err(_) => {
            system_admin_log.operate_name = "未知操作".to_string();
        }
    }
    let (parts, body) = req.into_parts();

   
    // 读取 body 为 bytes
    let bytes = match to_bytes(body, usize::MAX).await {
        Ok(v) => v,
        Err(_) => Bytes::new(),
    };
    // 处理 JSON 或 TEXT 请求体
    if let Some(v) = parts.headers.get("content-type") {
        if let Ok(content_type) = v.to_str() {
            if content_type.contains("json") {
                system_admin_log.request_body = Some(
                    serde_json::from_reader(bytes.clone().reader()).unwrap_or_default(),
                );
            } else if content_type.contains("text") {
                system_admin_log.request_body = Some(Value::String(
                    String::from_utf8_lossy(&bytes).to_string(),
                ));
            }
        }
    }

    // 重新组装请求
    let req = Request::from_parts(parts, bytes.into());
    // 调用下一个中间件
    let mut resp = next.run(req).await;
    let (parts, body) = resp.into_parts();
    let end = Utc::now().timestamp_millis();
    let headers = parts.headers.clone();
    let status = parts.status.as_u16();
    let bytes = match to_bytes(body, usize::MAX).await {
        Ok(v) => v,
        Err(_) => Bytes::new(),
    };
     //如果是get请求则不记录返回数据
     if system_admin_log.method != "GET" {
        // 处理 JSON 或 TEXT 响应体
        if let Some(v) = headers.get("content-type") {
            if let Ok(content_type) = v.to_str() {
                if content_type.contains("json") {
                    system_admin_log.response_data = Some(
                        serde_json::from_reader(bytes.clone().reader()).unwrap_or_default(),
                    );
                } else if content_type.contains("text") {
                    system_admin_log.response_data = Some(Value::String(
                        String::from_utf8_lossy(&bytes).to_string(),
                    ));
                }
            }
        }
    }
    
    system_admin_log.response_status = status as u32;
    system_admin_log.response_time_ms = end - start;
    resp = Response::from_parts(parts, bytes.into());
    let _ = AdminLogService::add_log(&app_state, system_admin_log).await;
    Ok(resp)
}