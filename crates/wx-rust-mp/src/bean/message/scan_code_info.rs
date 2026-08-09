//! 扫码信息。
//!
//! 对应 Java `me.chanjar.weixin.mp.bean.message.ScanCodeInfo`。

/// 扫码事件信息。
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ScanCodeInfo {
    /// 扫描类型，一般是 qrcode。
    pub scan_type: Option<String>,
    /// 扫描结果，即二维码对应的字符串信息。
    pub scan_result: Option<String>,
}

impl ScanCodeInfo {
    /// 扫描类型。
    pub fn get_scan_type(&self) -> Option<&str> {
        self.scan_type.as_deref()
    }

    /// 扫描结果。
    pub fn get_scan_result(&self) -> Option<&str> {
        self.scan_result.as_deref()
    }
}
