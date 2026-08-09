//! 小程序代码版本分布。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.bean.code.WxMaCodeVersionDistribution`。
//! 线格式由 `WxMaCodeVersionDistributionGsonAdapter` 决定：
//! `now_version` + `uv_info`（`{items: [{version, percentage}]}` 数组映射为
//! `{version -> percentage}` 的 Map）。

use std::collections::HashMap;

use serde::{Deserialize, Deserializer, Serialize};

/// 小程序代码版本分布（对应 Java `WxMaCodeVersionDistribution`）。
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct WxMaCodeVersionDistribution {
    /// 当前版本。
    pub now_version: String,
    /// 版本 -> 用户占比（百分比）。
    pub uv_info: HashMap<String, f32>,
}

/// 解析 `items` 数组为 Map（对应 adapter `getAsMap`）。
fn items_map(v: Option<&serde_json::Value>) -> HashMap<String, f32> {
    let mut map = HashMap::new();
    if let Some(serde_json::Value::Object(obj)) = v {
        if let Some(serde_json::Value::Array(items)) = obj.get("items") {
            for e in items {
                if let Some(e_obj) = e.as_object() {
                    let version = e_obj.get("version").and_then(|v| v.as_str());
                    let percentage = e_obj.get("percentage").and_then(|p| p.as_f64());
                    if let Some(version) = version {
                        map.insert(version.to_string(), percentage.unwrap_or(0.0) as f32);
                    }
                }
            }
        }
    }
    map
}

impl<'de> Deserialize<'de> for WxMaCodeVersionDistribution {
    /// 对应 Java `WxMaCodeVersionDistributionGsonAdapter.deserialize`。
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let v = serde_json::Value::deserialize(d)?;
        let obj = v.as_object().ok_or_else(|| {
            serde::de::Error::custom("WxMaCodeVersionDistribution 应为 JSON 对象")
        })?;
        Ok(WxMaCodeVersionDistribution {
            now_version: obj
                .get("now_version")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string(),
            uv_info: items_map(obj.get("uv_info")),
        })
    }
}

impl WxMaCodeVersionDistribution {
    /// 从 JSON 构建（对应 Java `fromJson`，adapter 线格式）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("代码版本分布解析失败: {e}"))
    }
}
