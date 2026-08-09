//! 对应 Java `cn.binarywang.wx.miniapp.bean.security.WxMaMsgSecCheckCheckResponse.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。
//! `label` 反序列化宽容数字/字符串：Java 字段为 `String`，但 msg_sec_check v2
//! 真实响应中 `label` 是 JSON 数字（如 20002），Gson 数字→String 强转可解析，
//! serde 需自定义适配（见 [`crate::bean::serde_util::de_num_or_str`]）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaMsgSecCheckCheckResponse {
    #[serde(rename = "errcode", default)]
    pub errcode: i32,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "result", default)]
    pub result: ResultBean,
    #[serde(rename = "trace_id", default)]
    pub trace_id: String,
    #[serde(rename = "detail", default)]
    pub detail: Vec<DetailBean>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ResultBean {
    #[serde(rename = "suggest", default)]
    pub suggest: String,
    #[serde(
        rename = "label",
        default,
        deserialize_with = "crate::bean::serde_util::de_num_or_str"
    )]
    pub label: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DetailBean {
    #[serde(rename = "strategy", default)]
    pub strategy: String,
    #[serde(rename = "errcode", default)]
    pub errcode: i32,
    #[serde(rename = "suggest", default)]
    pub suggest: String,
    #[serde(
        rename = "label",
        default,
        deserialize_with = "crate::bean::serde_util::de_num_or_str"
    )]
    pub label: String,
    #[serde(rename = "prob", default)]
    pub prob: i32,
    #[serde(rename = "level", default)]
    pub level: String,
    #[serde(rename = "keyword", default)]
    pub keyword: String,
}
