//! 获取授权方的选项设置信息的结果。
//!
//! 对应 Java `me.chanjar.weixin.open.bean.result.WxOpenAuthorizerOptionResult`。
//! 由 `WxOpenAuthorizerOptionResultGsonAdapter` 驱动解析
//! （`authorizer_appid`/`option_name`/`option_value` 键），与字段名直映
//! 不同，故人工迁移。

/// 获取授权方的选项设置信息的结果。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxOpenAuthorizerOptionResult {
    /// 授权方 appid。
    #[serde(rename = "authorizer_appid", default)]
    pub authorizer_appid: Option<String>,
    /// 选项名称。
    #[serde(rename = "option_name", default)]
    pub option_name: Option<String>,
    /// 选项值。
    #[serde(rename = "option_value", default)]
    pub option_value: Option<String>,
}
