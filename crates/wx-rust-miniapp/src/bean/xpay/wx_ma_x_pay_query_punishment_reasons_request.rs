//! 商户被管控原因查询请求。
//!
//! 对应官方文档 `developers.weixin.qq.com` 虚拟支付
//! `query_punishment_reasons`（2026-09 更新，超出 WxJava 4.8.6 覆盖范围的新增接口）。
//!
//! 官方定义无请求体字段（示例请求为 `{}`）；保留空结构体以统一
//! `WxMaXPayService` 的（请求, 签名）调用形状。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaXPayQueryPunishmentReasonsRequest {}

impl WxMaXPayQueryPunishmentReasonsRequest {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxMaXPayQueryPunishmentReasonsRequest 序列化失败: {e}"))
    }
}
