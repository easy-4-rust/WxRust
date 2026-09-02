//! 查询下载订单任务请求。
//!
//! 对应官方文档 `developers.weixin.qq.com` 虚拟支付
//! `query_download_order`（2026-09 更新，超出 WxJava 4.8.6 覆盖范围的新增接口）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaXPayQueryDownloadOrderRequest {
    /// 下载任务 ID，由 StartDownloadOrder 接口返回
    #[serde(rename = "task_id", default)]
    pub task_id: String,

    /// 环境标识：0=现网 1=沙箱
    #[serde(rename = "env", default)]
    pub env: i32,
}

impl WxMaXPayQueryDownloadOrderRequest {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxMaXPayQueryDownloadOrderRequest 序列化失败: {e}"))
    }
}
