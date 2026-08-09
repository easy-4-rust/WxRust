//! 访问来源分布数据。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.bean.analysis.WxMaVisitDistribution`。
//! 线格式由 `WxMaVisitDistributionGsonAdapter` 决定：`ref_date` + `list`
//! （`[{index, item_list: [{key, value}]}]` 映射为 `{index -> {key -> value}}`）。

use std::collections::HashMap;

use serde::{Deserialize, Deserializer, Serialize};

/// 访问来源分布数据（对应 Java `WxMaVisitDistribution`）。
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct WxMaVisitDistribution {
    /// 日期。
    pub ref_date: String,
    /// 分布数据：来源 ->（维度 -> 数值）。
    pub list: HashMap<String, HashMap<i32, i32>>,
}

impl<'de> Deserialize<'de> for WxMaVisitDistribution {
    /// 对应 Java `WxMaVisitDistributionGsonAdapter.deserialize`。
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let v = serde_json::Value::deserialize(d)?;
        let obj = v
            .as_object()
            .ok_or_else(|| serde::de::Error::custom("WxMaVisitDistribution 应为 JSON 对象"))?;
        let mut list = HashMap::new();
        if let Some(serde_json::Value::Array(list_array)) = obj.get("list") {
            for index_element in list_array {
                let index_obj = match index_element.as_object() {
                    Some(o) => o,
                    None => continue,
                };
                let index = index_obj.get("index").and_then(|i| i.as_str());
                if index.is_none() {
                    continue;
                }
                let mut item_list = HashMap::new();
                if let Some(serde_json::Value::Array(item_array)) = index_obj.get("item_list") {
                    for item_element in item_array {
                        if let Some(item_obj) = item_element.as_object() {
                            let key = item_obj.get("key").and_then(|k| k.as_i64());
                            let value = item_obj.get("value").and_then(|k| k.as_i64());
                            if let Some(k) = key {
                                item_list.insert(k as i32, value.unwrap_or(0) as i32);
                            }
                        }
                    }
                }
                list.insert(index.unwrap().to_string(), item_list);
            }
        }
        Ok(WxMaVisitDistribution {
            ref_date: obj
                .get("ref_date")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string(),
            list,
        })
    }
}

impl WxMaVisitDistribution {
    /// 从 JSON 构建（对应 Java `fromJson`，adapter 线格式）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("访问分布解析失败: {e}"))
    }
}
