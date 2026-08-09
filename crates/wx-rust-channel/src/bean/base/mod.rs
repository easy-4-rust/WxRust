//! 对应 Java `me.chanjar.weixin.channel.bean.base` 包（生成）。

pub mod address_info;
pub mod attr_info;
pub mod offset_param;
pub mod page_param;
pub mod stream_page_param;
pub mod time_range;
pub mod wx_channel_base_response;

pub use address_info::AddressInfo;
pub use attr_info::AttrInfo;
pub use offset_param::OffsetParam;
pub use page_param::PageParam;
pub use stream_page_param::StreamPageParam;
pub use time_range::TimeRange;
pub use wx_channel_base_response::WxChannelBaseResponse;
