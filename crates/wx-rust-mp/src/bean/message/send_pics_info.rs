//! 发送的图片信息。
//!
//! 对应 Java `me.chanjar.weixin.mp.bean.message.SendPicsInfo`。

/// 发送图片列表中的单项。
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PicItem {
    /// 图片的 MD5 值。
    pub pic_md5_sum: Option<String>,
}

/// 发送的图片信息（用户发图片时附带）。
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SendPicsInfo {
    /// 发送的图片数量。
    pub count: Option<i64>,
    /// 图片列表。
    pub pic_list: Vec<PicItem>,
}

impl SendPicsInfo {
    /// 发送的图片数量。
    pub fn get_count(&self) -> Option<i64> {
        self.count
    }

    /// 图片列表。
    pub fn get_pic_list(&self) -> &[PicItem] {
        &self.pic_list
    }
}
