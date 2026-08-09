//! 对应 Java `me.chanjar.weixin.cp.bean.WxCpAgentJsapiSignature.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpAgentJsapiSignature {
    #[serde(rename = "url", default)]
    pub url: String,
    #[serde(rename = "corpid", default)]
    pub corpid: String,
    #[serde(rename = "agentid", default)]
    pub agentid: i32,
    #[serde(rename = "timestamp", default)]
    pub timestamp: i64,
    #[serde(rename = "nonceStr", default)]
    pub nonce_str: String,
    #[serde(rename = "signature", default)]
    pub signature: String,
}

impl WxCpAgentJsapiSignature {
    /// 构建应用 jsapi 签名结果（对应 Java `@Builder` 语义；参数顺序与
    /// `WxCpService::create_agent_jsapi_signature` 调用一致：corpid、
    /// agentid、nonce_str、timestamp、url、signature。agentid 为 None
    /// 时按 0 处理（Java Integer 缺省 0 的构造语义）。
    pub fn new(
        corpid: impl Into<String>,
        agentid: Option<i32>,
        nonce_str: impl Into<String>,
        timestamp: i64,
        url: impl Into<String>,
        signature: impl Into<String>,
    ) -> Self {
        Self {
            corpid: corpid.into(),
            agentid: agentid.unwrap_or(0),
            nonce_str: nonce_str.into(),
            timestamp,
            url: url.into(),
            signature: signature.into(),
        }
    }
}
