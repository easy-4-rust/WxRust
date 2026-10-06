/// 微信小店接口数据。对应 Java: com.binarywang.wxjava.store.bean.after.AfterSaleCreateResponse
/// 微信小店 AfterSaleCreateResponse 数据类型；对应 Java: com.binarywang.wxjava.store.bean.after.AfterSaleCreateResponse。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AfterSaleCreateResponse {
    #[serde(rename = "errcode", skip_serializing_if = "Option::is_none")]
    pub err_code: Option<i32>,
    #[serde(rename = "errmsg", skip_serializing_if = "Option::is_none")]
    pub err_msg: Option<String>,
    #[serde(
        rename = "after_sale_order_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub after_sale_order_id: Option<String>,
}
