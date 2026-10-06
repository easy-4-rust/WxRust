//! 对应 Java `com.binarywang.wxjava.store.bean.favorite.FavoriteCountResponse.java`。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 FavoriteCountResponse；对应 Java com.binarywang.wxjava.store.bean.favorite.FavoriteCountResponse.java。
pub struct FavoriteCountResponse {
    /// 错误码
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    /// 错误信息
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    /// 收藏人数
    #[serde(rename = "favorite_count", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favorite_count: Option<i64>,
    /// 上游字段 favor_uv_acc_shop_homepage。
    #[serde(
        rename = "favor_uv_acc_shop_homepage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub favor_uv_acc_shop_homepage: Option<i64>,
    /// 上游字段 favor_uv_acc_order_detail。
    #[serde(
        rename = "favor_uv_acc_order_detail",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub favor_uv_acc_order_detail: Option<i64>,
    /// 上游字段 favor_uv_acc_product_detail。
    #[serde(
        rename = "favor_uv_acc_product_detail",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub favor_uv_acc_product_detail: Option<i64>,
    /// 上游字段 favor_uv_acc_other_scene。
    #[serde(
        rename = "favor_uv_acc_other_scene",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub favor_uv_acc_other_scene: Option<i64>,
    /// 上游字段 favor_uv_acc_all。
    #[serde(
        rename = "favor_uv_acc_all",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub favor_uv_acc_all: Option<i64>,
}
