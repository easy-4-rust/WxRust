//! 发送的位置信息。
//!
//! 对应 Java `me.chanjar.weixin.mp.bean.message.SendLocationInfo`。

/// 发送的位置信息（用户发送位置时附带）。
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SendLocationInfo {
    /// 地理位置纬度（字符串）。
    pub location_x: Option<String>,
    /// 地理位置经度（字符串）。
    pub location_y: Option<String>,
    /// 地图缩放大小。
    pub scale: Option<String>,
    /// 地理位置信息。
    pub label: Option<String>,
    /// 朋友圈 POI 名字。
    pub poi_name: Option<String>,
}

impl SendLocationInfo {
    /// 地理位置纬度。
    pub fn get_location_x(&self) -> Option<&str> {
        self.location_x.as_deref()
    }

    /// 地理位置经度。
    pub fn get_location_y(&self) -> Option<&str> {
        self.location_y.as_deref()
    }

    /// 地图缩放大小。
    pub fn get_scale(&self) -> Option<&str> {
        self.scale.as_deref()
    }

    /// 地理位置信息。
    pub fn get_label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    /// 朋友圈 POI 名字。
    pub fn get_poi_name(&self) -> Option<&str> {
        self.poi_name.as_deref()
    }
}
