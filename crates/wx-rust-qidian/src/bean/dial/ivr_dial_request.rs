//! 对应 Java `me.chanjar.weixin.qidian.bean.dial.IVRDialRequest.java`。

use serde::{Deserialize, Serialize};

/// IVR 外呼请求。
///
/// 对应 Java `IVRDialRequest`：字段名即 Java 字段名（下划线风格，无
/// `@SerializedName`，Gson 原样序列化）；`loc_pref_on` 默认 1、
/// `skip_restrict` 默认 false（对应 Java 字段初始值）。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IVRDialRequest {
    /// 被叫号码
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    /// IVR 流程 id
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ivr_id: Option<String>,
    /// 企业总机号列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub corp_phone_list: Option<Vec<String>>,
    /// 是否优先本机（默认 1）
    #[serde(
        default = "default_loc_pref_on",
        skip_serializing_if = "Option::is_none"
    )]
    pub loc_pref_on: Option<i32>,
    /// 备用企业总机号列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backup_corp_phone_list: Option<Vec<String>>,
    /// 是否跳过限制（默认 false）
    #[serde(
        default = "default_skip_restrict",
        skip_serializing_if = "Option::is_none"
    )]
    pub skip_restrict: Option<bool>,
}

impl Default for IVRDialRequest {
    /// 对应 Java 字段初始值：`loc_pref_on = 1`、`skip_restrict = false`。
    fn default() -> Self {
        Self {
            phone_number: None,
            ivr_id: None,
            corp_phone_list: None,
            loc_pref_on: Some(1),
            backup_corp_phone_list: None,
            skip_restrict: Some(false),
        }
    }
}

/// `loc_pref_on` 默认值 1（对应 Java 字段初始值）。
fn default_loc_pref_on() -> Option<i32> {
    Some(1)
}

/// `skip_restrict` 默认值 false（对应 Java 字段初始值）。
fn default_skip_restrict() -> Option<bool> {
    Some(false)
}

impl IVRDialRequest {
    /// 序列化为 JSON（对应 Java `toJson()`，Gson 默认省略 null 字段）。
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).expect("IVRDialRequest 序列化不应失败")
    }
}
