//! 对应 Java `com.binarywang.wxjava.store.bean.home.banner.BannerItemOfficialAccount.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 BannerItemOfficialAccount；对应 Java com.binarywang.wxjava.store.bean.home.banner.BannerItemOfficialAccount.java。
pub struct BannerItemOfficialAccount {
    #[serde(rename = "url", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
