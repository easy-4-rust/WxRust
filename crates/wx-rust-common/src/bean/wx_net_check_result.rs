//! 对应 Java `me.chanjar.weixin.common.bean.WxNetCheckResult`（由 gen_bean_structs.py 生成）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxNetCheckResult {
    /// dnsInfos（微信接口实际返回 `dns`，对应 Java WxNetCheckResultGsonAdapter 映射）
    #[serde(rename = "dnsInfos", alias = "dns", default)]
    pub dns_infos: Vec<WxNetCheckDnsInfo>,
    /// pingInfos（微信接口实际返回 `ping`）
    #[serde(rename = "pingInfos", alias = "ping", default)]
    pub ping_infos: Vec<WxNetCheckPingInfo>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxNetCheckDnsInfo {
    /// ip
    #[serde(rename = "ip", default)]
    pub ip: String,
    /// realOperator（微信实际返回 `real_operator`，对应 Gson 适配器映射）
    #[serde(rename = "realOperator", alias = "real_operator", default)]
    pub real_operator: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxNetCheckPingInfo {
    /// ip
    #[serde(rename = "ip", default)]
    pub ip: String,
    /// fromOperator（微信实际返回 `from_operator`）
    #[serde(rename = "fromOperator", alias = "from_operator", default)]
    pub from_operator: String,
    /// packageLoss（微信实际返回 `package_loss`）
    #[serde(rename = "packageLoss", alias = "package_loss", default)]
    pub package_loss: String,
    /// time
    #[serde(rename = "time", default)]
    pub time: String,
}
