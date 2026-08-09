//! 小程序代码提审（commit）请求。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.bean.code.WxMaCodeCommitRequest`。
//! 线格式由 `WxMaCodeCommitRequestGsonAdapter` 决定：`template_id`/
//! `user_version`/`user_desc` + `ext_json`（extConfig 的 JSON 字符串化）。
//! serde 派生仅为平铺字段解析；对外序列化以 `to_json`（adapter 线格式）为准。

use serde::{Deserialize, Serialize};

use crate::bean::code::WxMaCodeExtConfig;

/// 小程序代码提审请求（对应 Java `WxMaCodeCommitRequest`）。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct WxMaCodeCommitRequest {
    /// 代码库中的代码模版 ID。
    #[serde(rename = "template_id", default)]
    pub template_id: i64,
    /// 代码版本号，开发者可自定义。
    #[serde(rename = "user_version", default)]
    pub user_version: String,
    /// 代码描述，开发者可自定义。
    #[serde(rename = "user_desc", default)]
    pub user_desc: String,
    /// 第三方自定义的配置（序列化为 `ext_json` 字符串）。
    pub ext_config: Option<WxMaCodeExtConfig>,
}

impl WxMaCodeCommitRequest {
    /// 序列化为 JSON（对应 Java `WxMaCodeCommitRequestGsonAdapter` 线格式）。
    pub fn to_json(&self) -> Result<String, String> {
        let mut map = serde_json::Map::new();
        map.insert("template_id".into(), serde_json::json!(self.template_id));
        map.insert("user_version".into(), serde_json::json!(self.user_version));
        map.insert("user_desc".into(), serde_json::json!(self.user_desc));
        if let Some(ext_config) = &self.ext_config {
            let ext_json = serde_json::to_string(ext_config)
                .map_err(|e| format!("extConfig 序列化失败: {e}"))?;
            map.insert("ext_json".into(), serde_json::json!(ext_json));
        }
        serde_json::to_string(&serde_json::Value::Object(map))
            .map_err(|e| format!("代码提审请求序列化失败: {e}"))
    }
}
