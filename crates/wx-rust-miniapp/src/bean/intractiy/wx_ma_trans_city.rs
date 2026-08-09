//! 对应 Java `cn.binarywang.wx.miniapp.bean.intractiy.WxMaTransCity.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaTransCity {
    #[serde(rename = "serviceTransId", default)]
    pub service_trans_id: String,
    #[serde(rename = "cityList", default)]
    pub city_list: Vec<City>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct City {
    #[serde(rename = "cityName", default)]
    pub city_name: String,
    #[serde(rename = "cityCode", default)]
    pub city_code: String,
}
