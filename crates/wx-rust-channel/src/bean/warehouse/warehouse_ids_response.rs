//! 对应 Java `me.chanjar.weixin.channel.bean.warehouse.WarehouseIdsResponse.java`。
//!
//! 手写（HAND_WRITTEN 保护）：Java 以 `@JsonProperty("data")` setter
//! `unpackNameFromNestedObject` 展开嵌套对象（响应形如
//! `{"errcode":0,"errmsg":"ok","data":{"out_warehouse_ids":[...],"next_key":"..."}}`），
//! serde 以影子结构体镜像该反序列化语义；序列化仅输出扁平字段（与 Java toJson 一致，
//! Java 的 data setter 无 getter 不入序列化）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct WarehouseIdsResponse {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    /// 外部仓库ID列表（对应 Java `out_warehouse_ids`，经 `data` 展开）
    #[serde(rename = "out_warehouse_ids", default)]
    pub ids: Vec<String>,
    /// 本次翻页的上下文，用于请求下一页，如果是空，则当前是最后一页
    #[serde(rename = "next_key", default)]
    pub next_key: String,
}

impl<'de> serde::Deserialize<'de> for WarehouseIdsResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        // 影子结构体：对应 Java `@JsonProperty("data")` setter 展开前的外层 JSON
        #[derive(serde::Deserialize, Default)]
        struct DataInner {
            #[serde(rename = "out_warehouse_ids", default)]
            ids: Vec<String>,
            #[serde(rename = "next_key", default)]
            next_key: String,
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
        Ok(WarehouseIdsResponse {
            err_code: shadow.err_code,
            err_msg: shadow.err_msg,
            ids: data.ids,
            next_key: data.next_key,
        })
    }
}
