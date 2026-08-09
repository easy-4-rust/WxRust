//! 视频号小店 枚举（对应 Java `PackageAuditItemType`）。

/// PackageAuditItemType（对应 Java `me.chanjar.weixin.channel.enums.PackageAuditItemType`）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PackageAuditItemType {
    /// 商品快递单图片url
    ExpressPic,
    /// 商品包装箱图片url
    BoxPic,
    /// 商品开箱图片url
    UnboxingPic,
    /// 商品单个细节图片url
    DetailPic,
}

impl PackageAuditItemType {
    /// 枚举 key（对应 Java `getKey()`）。
    pub fn key(&self) -> &'static str {
        match self {
            PackageAuditItemType::ExpressPic => "product_express_pic_url",
            PackageAuditItemType::BoxPic => "product_packaging_box_pic_url",
            PackageAuditItemType::UnboxingPic => "product_unboxing_pic_url",
            PackageAuditItemType::DetailPic => "single_product_detail_pic_url",
        }
    }

    /// 枚举中文说明（对应 Java `getValue()`）。
    pub fn value(&self) -> &'static str {
        match self {
            PackageAuditItemType::ExpressPic => "商品快递单图片url",
            PackageAuditItemType::BoxPic => "商品包装箱图片url",
            PackageAuditItemType::UnboxingPic => "商品开箱图片url",
            PackageAuditItemType::DetailPic => "商品单个细节图片url",
        }
    }
}
