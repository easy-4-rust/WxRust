//! 对应 Java `me.chanjar.weixin.channel.bean.product.ProductTimingSaleParam.java`。

#[allow(unused_imports)]
use super::*;

/// 商品立即开售/定时开售请求参数。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BeginTimingSaleParam {
    /// 商品 ID。
    #[serde(rename = "product_id", default)]
    pub product_id: String,
    /// 定时任务 ID。
    #[serde(rename = "task_id", default)]
    pub task_id: i64,
}
