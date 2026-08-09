//! 视频号小店 枚举（对应 Java `AfterSalesReason`）。

/// AfterSalesReason（对应 Java `me.chanjar.weixin.channel.enums.AfterSalesReason`）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AfterSalesReason {
    /// 拍错/多拍
    IncorrectSelection,
    /// 不想要了
    NoLongerWant,
    /// 无快递信息
    NoExpressInfo,
    /// 包裹为空
    EmptyPackage,
    /// 已拒签包裹
    RejectReceivePackage,
    /// 快递长时间未送达
    NotDeliveredTooLong,
    /// 与商品描述不符
    NotMatchProductDesc,
    /// 质量问题
    QualityIssue,
    /// 卖家发错货
    SendWrongGoods,
    /// 三无产品
    ThreeNoProduct,
    /// 假冒产品
    FakeProduct,
    /// 其它
    Others,
}

impl AfterSalesReason {
    /// 枚举 key（对应 Java `getKey()`）。
    pub fn key(&self) -> &'static str {
        match self {
            AfterSalesReason::IncorrectSelection => "INCORRECT_SELECTION",
            AfterSalesReason::NoLongerWant => "NO_LONGER_WANT",
            AfterSalesReason::NoExpressInfo => "NO_EXPRESS_INFO",
            AfterSalesReason::EmptyPackage => "EMPTY_PACKAGE",
            AfterSalesReason::RejectReceivePackage => "REJECT_RECEIVE_PACKAGE",
            AfterSalesReason::NotDeliveredTooLong => "NOT_DELIVERED_TOO_LONG",
            AfterSalesReason::NotMatchProductDesc => "NOT_MATCH_PRODUCT_DESC",
            AfterSalesReason::QualityIssue => "QUALITY_ISSUE",
            AfterSalesReason::SendWrongGoods => "SEND_WRONG_GOODS",
            AfterSalesReason::ThreeNoProduct => "THREE_NO_PRODUCT",
            AfterSalesReason::FakeProduct => "FAKE_PRODUCT",
            AfterSalesReason::Others => "OTHERS",
        }
    }

    /// 枚举中文说明（对应 Java `getValue()`）。
    pub fn value(&self) -> &'static str {
        match self {
            AfterSalesReason::IncorrectSelection => "拍错/多拍",
            AfterSalesReason::NoLongerWant => "不想要了",
            AfterSalesReason::NoExpressInfo => "无快递信息",
            AfterSalesReason::EmptyPackage => "包裹为空",
            AfterSalesReason::RejectReceivePackage => "已拒签包裹",
            AfterSalesReason::NotDeliveredTooLong => "快递长时间未送达了",
            AfterSalesReason::NotMatchProductDesc => "与商品描述不符",
            AfterSalesReason::QualityIssue => "质量问题",
            AfterSalesReason::SendWrongGoods => "卖家发错货",
            AfterSalesReason::ThreeNoProduct => "三无产品",
            AfterSalesReason::FakeProduct => "假冒产品",
            AfterSalesReason::Others => "其它",
        }
    }

    /// 按 key 查找（对应 Java `getByKey(String)`；找不到返回 OTHERS。
    pub fn get_by_key(key: &str) -> Self {
        for v in [
            AfterSalesReason::IncorrectSelection,
            AfterSalesReason::NoLongerWant,
            AfterSalesReason::NoExpressInfo,
            AfterSalesReason::EmptyPackage,
            AfterSalesReason::RejectReceivePackage,
            AfterSalesReason::NotDeliveredTooLong,
            AfterSalesReason::NotMatchProductDesc,
            AfterSalesReason::QualityIssue,
            AfterSalesReason::SendWrongGoods,
            AfterSalesReason::ThreeNoProduct,
            AfterSalesReason::FakeProduct,
            AfterSalesReason::Others,
        ] {
            if v.key() == key {
                return v;
            }
        }
        AfterSalesReason::Others
    }
}
