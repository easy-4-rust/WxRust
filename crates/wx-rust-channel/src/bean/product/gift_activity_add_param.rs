//! 对应 Java `me.chanjar.weixin.channel.bean.product.GiftActivityAddParam`。

use super::GiftActivityInfo;

/// 创建买赠活动参数（对应 Java `GiftActivityAddParam`）。
///
/// 将 `GiftActivityInfo` 包装在 `gift_activity` 字段中，
/// 序列化为 `{"gift_activity": {...}}`。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GiftActivityAddParam {
    /// 赠品活动信息
    #[serde(rename = "gift_activity")]
    pub gift_activity: GiftActivityInfo,
}

impl GiftActivityAddParam {
    /// 从活动信息构建参数。
    pub fn new(gift_activity: GiftActivityInfo) -> Self {
        Self { gift_activity }
    }
}
