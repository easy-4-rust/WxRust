//! 用户画像分布数据。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.bean.analysis.WxMaUserPortrait`。
//! 线格式由 `WxMaUserPortraitGsonAdapter` 决定：`ref_date` + `visit_uv_new`/
//! `visit_uv`（对象，各维度 `[{name, value}]` 数组映射为 `{name -> value}` Map）。

use std::collections::HashMap;

use serde::{Deserialize, Deserializer, Serialize};

/// 用户画像某维度分布项（对应 Java `WxMaUserPortrait.Item`）。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Item {
    /// 省份分布。
    #[serde(default)]
    pub province: HashMap<String, i64>,
    /// 城市分布。
    #[serde(default)]
    pub city: HashMap<String, i64>,
    /// 性别分布。
    #[serde(default)]
    pub genders: HashMap<String, i64>,
    /// 平台分布。
    #[serde(default)]
    pub platforms: HashMap<String, i64>,
    /// 设备分布。
    #[serde(default)]
    pub devices: HashMap<String, i64>,
    /// 年龄分布。
    #[serde(default)]
    pub ages: HashMap<String, i64>,
}

/// 用户画像分布数据（对应 Java `WxMaUserPortrait`）。
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct WxMaUserPortrait {
    /// 日期。
    pub ref_date: String,
    /// 新增用户画像。
    pub visit_uv_new: Option<Item>,
    /// 活跃用户画像。
    pub visit_uv: Option<Item>,
}

/// 解析某维度分布（对应 adapter `getPortraitItem`：`{province: [{name, value}]}`）。
fn parse_item(v: &serde_json::Value) -> Option<Item> {
    let obj = v.as_object()?;
    Some(Item {
        province: name_value_map(obj.get("province")),
        city: name_value_map(obj.get("city")),
        genders: name_value_map(obj.get("genders")),
        platforms: name_value_map(obj.get("platforms")),
        devices: name_value_map(obj.get("devices")),
        ages: name_value_map(obj.get("ages")),
    })
}

/// 将 `[{name, value}]` 数组解析为 Map（对应 adapter `getAsMap`）。
fn name_value_map(v: Option<&serde_json::Value>) -> HashMap<String, i64> {
    let mut map = HashMap::new();
    if let Some(serde_json::Value::Array(arr)) = v {
        for e in arr {
            if let Some(obj) = e.as_object() {
                let name = obj.get("name").and_then(|n| n.as_str());
                let value = obj.get("value").and_then(|n| n.as_i64());
                if let Some(name) = name {
                    if !name.trim().is_empty() {
                        map.insert(name.to_string(), value.unwrap_or(0));
                    }
                }
            }
        }
    }
    map
}

impl<'de> Deserialize<'de> for WxMaUserPortrait {
    /// 对应 Java `WxMaUserPortraitGsonAdapter.deserialize`。
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let v = serde_json::Value::deserialize(d)?;
        let obj = v
            .as_object()
            .ok_or_else(|| serde::de::Error::custom("WxMaUserPortrait 应为 JSON 对象"))?;
        Ok(WxMaUserPortrait {
            ref_date: obj
                .get("ref_date")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string(),
            visit_uv_new: obj.get("visit_uv_new").and_then(parse_item),
            visit_uv: obj.get("visit_uv").and_then(parse_item),
        })
    }
}

impl WxMaUserPortrait {
    /// 从 JSON 构建（对应 Java `fromJson`，adapter 线格式）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("用户画像解析失败: {e}"))
    }
}
