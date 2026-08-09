//! 卡券枚举：CardSceneType。
//!
//! 对应 Java `me.chanjar.weixin.mp.bean.card.enums.CardSceneType`。

/// CardSceneType。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CardSceneType {
    /// 附近
    #[default]
    SceneNearBy,
    /// 自定义菜单
    SceneMenu,
    /// 二维码
    SceneQrcode,
    /// 公众号文章
    SceneArticle,
    /// H5
    SceneH5,
    /// 自动回复
    SceneIvr,
    /// 卡券自定义cell
    SceneCardCustomCell,
}

impl CardSceneType {
    /// 全部枚举项（保持 Java 声明顺序）。
    pub const ALL: [Self; 7] = [
        Self::SceneNearBy,
        Self::SceneMenu,
        Self::SceneQrcode,
        Self::SceneArticle,
        Self::SceneH5,
        Self::SceneIvr,
        Self::SceneCardCustomCell,
    ];

    /// 枚举值（对应 Java 构造参数，如描述/颜色）。
    pub fn value(self) -> &'static str {
        match self {
            Self::SceneNearBy => "附近",
            Self::SceneMenu => "自定义菜单",
            Self::SceneQrcode => "二维码",
            Self::SceneArticle => "公众号文章",
            Self::SceneH5 => "H5",
            Self::SceneIvr => "自动回复",
            Self::SceneCardCustomCell => "卡券自定义cell",
        }
    }

    /// 按枚举值查找。
    pub fn find_by_value(value: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|e| e.value() == value)
    }
}
