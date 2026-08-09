//! 对应 Java `cn.binarywang.wx.miniapp.bean.Watermark.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。
//! `timestamp` 反序列化宽容数字/字符串：Java 字段为 `String` 但微信接口可能返回
//! JSON 数字（UNIX 秒），Gson 数字→String 强转可解析，serde 需自定义适配
//! （见 [`super::serde_util::de_num_or_str`]）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Watermark {
    #[serde(
        rename = "timestamp",
        default,
        deserialize_with = "super::serde_util::de_num_or_str"
    )]
    pub timestamp: String,
    #[serde(rename = "appid", default)]
    pub appid: String,
}
