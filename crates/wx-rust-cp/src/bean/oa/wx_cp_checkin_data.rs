//! 对应 Java `me.chanjar.weixin.cp.bean.oa.WxCpCheckinData.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpCheckinData {
    #[serde(rename = "userid", default)]
    pub user_id: String,
    #[serde(rename = "groupname", default)]
    pub group_name: String,
    #[serde(rename = "checkin_type", default)]
    pub checkin_type: String,
    #[serde(rename = "exception_type", default)]
    pub exception_type: String,
    #[serde(rename = "checkin_time", default)]
    pub checkin_time: i64,
    #[serde(rename = "location_title", default)]
    pub location_title: String,
    #[serde(rename = "location_detail", default)]
    pub location_detail: String,
    #[serde(rename = "wifiname", default)]
    pub wifi_name: String,
    #[serde(rename = "wifimac", default)]
    pub wifi_mac: String,
    #[serde(rename = "notes", default)]
    pub notes: String,
    #[serde(rename = "mediaids", default)]
    pub media_ids: Vec<String>,
    #[serde(rename = "lat", default)]
    pub lat: i32,
    #[serde(rename = "lng", default)]
    pub lng: i32,
    #[serde(rename = "deviceid", default)]
    pub device_id: String,
    #[serde(rename = "sch_checkin_time", default)]
    pub sch_checkin_time: i64,
    #[serde(rename = "groupid", default)]
    pub group_id: i32,
    #[serde(rename = "schedule_id", default)]
    pub schedule_id: i32,
    #[serde(rename = "timeline_id", default)]
    pub timeline_id: i32,
}
