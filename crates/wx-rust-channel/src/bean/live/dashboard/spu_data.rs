//! 对应 Java `me.chanjar.weixin.channel.bean.live.dashboard.SpuData.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::live::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SpuData {
    #[serde(rename = "base_data", default)]
    pub base_data: SpuBaseData,
    #[serde(rename = "exp_uv", default)]
    pub exp_uv: i64,
    #[serde(rename = "exp_pv", default)]
    pub exp_pv: i64,
    #[serde(rename = "fans_exp_uv", default)]
    pub fans_exp_uv: i64,
    #[serde(rename = "fans_exp_pv", default)]
    pub fans_exp_pv: i64,
    #[serde(rename = "non_fans_exp_uv", default)]
    pub non_fans_exp_uv: i64,
    #[serde(rename = "non_fans_exp_pv", default)]
    pub non_fans_exp_pv: i64,
    #[serde(rename = "new_customer_exp_uv", default)]
    pub new_customer_exp_uv: i64,
    #[serde(rename = "new_customer_exp_pv", default)]
    pub new_customer_exp_pv: i64,
    #[serde(rename = "repeated_customer_exp_uv", default)]
    pub repeated_customer_exp_uv: i64,
    #[serde(rename = "repeated_customer_exp_pv", default)]
    pub repeated_customer_exp_pv: i64,
    #[serde(rename = "clk_uv", default)]
    pub clk_uv: i64,
    #[serde(rename = "clk_pv", default)]
    pub clk_pv: i64,
    #[serde(rename = "new_customer_clk_uv", default)]
    pub new_customer_clk_uv: i64,
    #[serde(rename = "new_customer_clk_pv", default)]
    pub new_customer_clk_pv: i64,
    #[serde(rename = "repeated_customer_clk_uv", default)]
    pub repeated_customer_clk_uv: i64,
    #[serde(rename = "repeated_customer_clk_pv", default)]
    pub repeated_customer_clk_pv: i64,
    #[serde(rename = "fans_clk_uv", default)]
    pub fans_clk_uv: i64,
    #[serde(rename = "fans_clk_pv", default)]
    pub fans_clk_pv: i64,
    #[serde(rename = "non_fans_clk_uv", default)]
    pub non_fans_clk_uv: i64,
    #[serde(rename = "non_fans_clk_pv", default)]
    pub non_fans_clk_pv: i64,
    #[serde(rename = "share_uv", default)]
    pub share_uv: i64,
    #[serde(rename = "share_pv", default)]
    pub share_pv: i64,
    #[serde(rename = "exp_clk_ratio", default)]
    pub exp_clk_ratio: f64,
    #[serde(rename = "clk_pay_ratio", default)]
    pub clk_pay_ratio: f64,
    #[serde(rename = "gmv", default)]
    pub gmv: i64,
    #[serde(rename = "pay_pv", default)]
    pub pay_pv: i64,
    #[serde(rename = "pay_uv", default)]
    pub pay_uv: i64,
    #[serde(rename = "fans_pay_pv", default)]
    pub fans_pay_pv: i64,
    #[serde(rename = "fans_pay_uv", default)]
    pub fans_pay_uv: i64,
    #[serde(rename = "non_fans_pay_pv", default)]
    pub non_fans_pay_pv: i64,
    #[serde(rename = "non_fans_pay_uv", default)]
    pub non_fans_pay_uv: i64,
    #[serde(rename = "new_customer_pay_pv", default)]
    pub new_customer_pay_pv: i64,
    #[serde(rename = "new_customer_pay_uv", default)]
    pub new_customer_pay_uv: i64,
    #[serde(rename = "repeated_customer_pay_pv", default)]
    pub repeated_customer_pay_pv: i64,
    #[serde(rename = "repeated_customer_pay_uv", default)]
    pub repeated_customer_pay_uv: i64,
    #[serde(rename = "refund_uv", default)]
    pub refund_uv: i64,
    #[serde(rename = "refund_pv", default)]
    pub refund_pv: i64,
    #[serde(rename = "refund_amount", default)]
    pub refund_amount: i64,
    #[serde(rename = "create_uv", default)]
    pub create_uv: i64,
    #[serde(rename = "create_pv", default)]
    pub create_pv: i64,
    #[serde(rename = "fans_create_uv", default)]
    pub fans_create_uv: i64,
    #[serde(rename = "fans_create_pv", default)]
    pub fans_create_pv: i64,
    #[serde(rename = "non_fans_create_uv", default)]
    pub non_fans_create_uv: i64,
    #[serde(rename = "non_fans_create_pv", default)]
    pub non_fans_create_pv: i64,
    #[serde(rename = "new_customer_create_uv", default)]
    pub new_customer_create_uv: i64,
    #[serde(rename = "new_customer_create_pv", default)]
    pub new_customer_create_pv: i64,
    #[serde(rename = "repeated_customer_create_uv", default)]
    pub repeated_customer_create_uv: i64,
    #[serde(rename = "repeated_customer_create_pv", default)]
    pub repeated_customer_create_pv: i64,
    #[serde(rename = "stock", default)]
    pub stock: i64,
    #[serde(rename = "refund_rate", default)]
    pub refund_rate: f64,
    #[serde(rename = "finish_pv", default)]
    pub finish_pv: i64,
    #[serde(rename = "no_finish_pv", default)]
    pub no_finish_pv: i64,
    #[serde(rename = "new_customer_conversion_rate", default)]
    pub new_customer_conversion_rate: f64,
    #[serde(rename = "explanation_count", default)]
    pub explanation_count: i64,
}
