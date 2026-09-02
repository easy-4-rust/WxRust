//! 下载支付订单响应。
//!
//! 对应官方文档 `developers.weixin.qq.com` 虚拟支付
//! `start_download_order`（2026-09 更新，超出 WxJava 4.8.6 覆盖范围的新增接口）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaXPayStartDownloadOrderResponse {
    /// 错误码，0 表示成功
    #[serde(rename = "errcode", default)]
    pub errcode: i32,

    /// 错误信息
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,

    /// 下载任务 ID，用于后续查询下载结果
    #[serde(rename = "task_id", default)]
    pub task_id: String,
}

impl WxMaXPayStartDownloadOrderResponse {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxMaXPayStartDownloadOrderResponse 序列化失败: {e}"))
    }
}
