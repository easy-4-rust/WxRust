//! 对应 Java `me.chanjar.weixin.channel.bean.warehouse.WarehouseStockResponse.java`。
//!
//! 手写（HAND_WRITTEN 保护）：Java 以 `@JsonProperty("data")` setter
//! `unpackNameFromNestedObject` 展开嵌套对象（响应形如
//! `{"errcode":0,"errmsg":"ok","data":{"num":1}}`），serde 以影子结构体
//! 镜像该反序列化语义；序列化仅输出扁平字段（与 Java toJson 一致）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct WarehouseStockResponse {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    /// 仓库库存（对应 Java `num`，经 `data` 展开）
    #[serde(rename = "num", default)]
    pub num: i32,
}

impl<'de> serde::Deserialize<'de> for WarehouseStockResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        // 影子结构体：对应 Java `@JsonProperty("data")` setter 展开前的外层 JSON
        #[derive(serde::Deserialize, Default)]
        struct DataInner {
            #[serde(rename = "num", default)]
            num: i32,
        }
        #[derive(serde::Deserialize)]
        struct Shadow {
            #[serde(rename = "errcode", default)]
            err_code: i32,
            #[serde(rename = "errmsg", default)]
            err_msg: String,
            #[serde(rename = "data", default)]
            data: Option<DataInner>,
        }
        let shadow = Shadow::deserialize(d)?;
        let data = shadow.data.unwrap_or_default();
        Ok(WarehouseStockResponse {
            err_code: shadow.err_code,
            err_msg: shadow.err_msg,
            num: data.num,
        })
    }
}
