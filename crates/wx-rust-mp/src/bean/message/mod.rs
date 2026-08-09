//! 微信推送消息模型。
//!
//! 对应 Java `me.chanjar.weixin.mp.bean.message` 包。

pub mod article_url_result;
pub mod hard_ware;
pub mod scan_code_info;
pub mod send_location_info;
pub mod send_pics_info;
pub mod wx_mp_subscribe_msg_event;
pub mod wx_mp_xml_message;
pub mod wx_mp_xml_out_device_message;
pub mod wx_mp_xml_out_image_message;
pub mod wx_mp_xml_out_message;
pub mod wx_mp_xml_out_music_message;
pub mod wx_mp_xml_out_news_message;
pub mod wx_mp_xml_out_text_message;
pub mod wx_mp_xml_out_transfer_biz_ai_ivr_message;
pub mod wx_mp_xml_out_transfer_kefu_message;
pub mod wx_mp_xml_out_video_message;
pub mod wx_mp_xml_out_voice_message;

pub use article_url_result::{ArticleUrlResult, ArticleUrlResultItem};
pub use hard_ware::HardWare;
pub use scan_code_info::ScanCodeInfo;
pub use send_location_info::SendLocationInfo;
pub use send_pics_info::{PicItem, SendPicsInfo};
pub use wx_mp_subscribe_msg_event::{
    ChangeEvent, PopupEvent, SentEvent, WxMpSubscribeMsgChangeEvent, WxMpSubscribeMsgEvent,
    WxMpSubscribeMsgPopupEvent, WxMpSubscribeMsgSentEvent,
};
pub use wx_mp_xml_message::WxMpXmlMessage;
pub use wx_mp_xml_out_image_message::WxMpXmlOutImageMessage;
pub use wx_mp_xml_out_message::WxMpXmlOutMessage;
pub use wx_mp_xml_out_music_message::{WxMpXmlOutMusic, WxMpXmlOutMusicMessage};
pub use wx_mp_xml_out_news_message::{WxMpXmlOutNewsMessage, WxMpXmlOutNewsMessageItem};
pub use wx_mp_xml_out_text_message::WxMpXmlOutTextMessage;
pub use wx_mp_xml_out_transfer_biz_ai_ivr_message::WxMpXmlOutTransferBizAiIvrMessage;
pub use wx_mp_xml_out_transfer_kefu_message::WxMpXmlOutTransferKefuMessage;
pub use wx_mp_xml_out_video_message::{WxMpXmlOutVideo, WxMpXmlOutVideoMessage};
pub use wx_mp_xml_out_voice_message::WxMpXmlOutVoiceMessage;
