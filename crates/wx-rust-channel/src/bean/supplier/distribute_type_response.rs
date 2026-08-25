//! 对应 Java `me.chanjar.weixin.channel.bean.supplier.DistributeTypeResponse.java`。

#[allow(unused_imports)]
use super::*;

#[allow(unused_imports)]
use crate::bean::base::WxChannelBaseResponse;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DistributeTypeResponse {
    /// 错误码
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    /// 错误信息
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    /// 分配方式：1-手动分配，2-全店自动分配，3-按商品自动分配
    #[serde(rename = "distribute_type", default)]
    pub distribute_type: i32,
    /// 供货商 ID（全店自动分配时有值）
    #[serde(rename = "supplier_id", default)]
    pub supplier_id: String,
}
