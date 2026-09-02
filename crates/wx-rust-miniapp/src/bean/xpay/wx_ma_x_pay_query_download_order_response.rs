//! 查询下载订单任务响应。
//!
//! 对应官方文档 `developers.weixin.qq.com` 虚拟支付
//! `query_download_order`（2026-09 更新，超出 WxJava 4.8.6 覆盖范围的新增接口）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaXPayQueryDownloadOrderResponse {
    /// 错误码，0 表示成功
    #[serde(rename = "errcode", default)]
    pub errcode: i32,

    /// 错误信息
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,

    /// 下载任务 ID，与请求参数对应
    #[serde(rename = "task_id", default)]
    pub task_id: String,

    /// 任务状态：0=初始化 1=运行中 2=成功 3=失败
    #[serde(rename = "status", default)]
    pub status: i32,

    /// 下载文件 URL，仅 status=2 时有值
    #[serde(rename = "download_url", default)]
    pub download_url: String,

    /// URL 过期时间（Unix 秒级时间戳）
    #[serde(rename = "expire_at", default)]
    pub expire_at: i64,
}

impl WxMaXPayQueryDownloadOrderResponse {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxMaXPayQueryDownloadOrderResponse 序列化失败: {e}"))
    }
}
