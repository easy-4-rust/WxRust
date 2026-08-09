//! 对应 Java `me.chanjar.weixin.cp.bean.msgaudit.WxCpChatModel.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpChatModel {
    #[serde(rename = "msgid", default)]
    pub msg_id: String,
    #[serde(rename = "action", default)]
    pub action: String,
    #[serde(rename = "from", default)]
    pub from: String,
    #[serde(rename = "tolist", default)]
    pub tolist: Vec<String>,
    #[serde(rename = "roomid", default)]
    pub room_id: String,
    #[serde(rename = "msgtime", default)]
    pub msg_time: i64,
    #[serde(rename = "msgtype", default)]
    pub msg_type: String,
    #[serde(rename = "text", default)]
    pub text: Text,
    #[serde(rename = "image", default)]
    pub image: Image,
    #[serde(rename = "revoke", default)]
    pub revoke: Revoke,
    #[serde(rename = "agree", default)]
    pub agree: Agree,
    #[serde(rename = "disagree", default)]
    pub disagree: Agree,
    #[serde(rename = "voice", default)]
    pub voice: Voice,
    #[serde(rename = "video", default)]
    pub video: Video,
    #[serde(rename = "card", default)]
    pub card: Card,
    #[serde(rename = "location", default)]
    pub location: Location,
    #[serde(rename = "emotion", default)]
    pub emotion: Emotion,
    #[serde(rename = "file", default)]
    pub file: File,
    #[serde(rename = "link", default)]
    pub link: Link,
    #[serde(rename = "weapp", default)]
    pub weapp: Weapp,
    #[serde(rename = "chatrecord", default)]
    pub chat_record: ChatRecord,
    #[serde(rename = "collect", default)]
    pub collect: Collect,
    #[serde(rename = "redpacket", default)]
    pub red_packet: Redpacket,
    #[serde(rename = "meeting", default)]
    pub meeting: Meeting,
    #[serde(rename = "time", default)]
    pub time: i64,
    #[serde(rename = "user", default)]
    pub user: String,
    #[serde(rename = "doc", default)]
    pub doc: Doc,
    #[serde(rename = "info", default)]
    pub info: Info,
    #[serde(rename = "calendar", default)]
    pub calendar: Calendar,
    #[serde(rename = "mixed", default)]
    pub mixed: Mixed,
    #[serde(rename = "voiceid", default)]
    pub voice_id: String,
    #[serde(rename = "meeting_voice_call", default)]
    pub meeting_voice_call: MeetingVoiceCall,
    #[serde(rename = "voipid", default)]
    pub voip_id: String,
    #[serde(rename = "voip_doc_share", default)]
    pub voip_doc_share: crate::bean::msgaudit::wx_cp_file_item::WxCpFileItem,
    #[serde(rename = "sphfeed", default)]
    pub sph_feed: SphFeed,
    #[serde(rename = "voiptext", default)]
    pub voip_text: VoipText,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Text {
    #[serde(rename = "content", default)]
    pub content: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Image {
    #[serde(rename = "md5sum", default)]
    pub md5_sum: String,
    #[serde(rename = "sdkfileid", default)]
    pub sdk_file_id: String,
    #[serde(rename = "filesize", default)]
    pub file_size: i64,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Revoke {
    #[serde(rename = "pre_msgid", default)]
    pub pre_msg_id: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Agree {
    #[serde(rename = "userid", default)]
    pub user_id: String,
    #[serde(rename = "agree_time", default)]
    pub agree_time: i64,
    #[serde(rename = "disagree_time", default)]
    pub disagree_time: i64,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Voice {
    #[serde(rename = "md5sum", default)]
    pub md5_sum: String,
    #[serde(rename = "sdkfileid", default)]
    pub sdk_file_id: String,
    #[serde(rename = "voice_size", default)]
    pub voice_size: i64,
    #[serde(rename = "play_length", default)]
    pub play_length: i64,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Video {
    #[serde(rename = "md5sum", default)]
    pub md5_sum: String,
    #[serde(rename = "sdkfileid", default)]
    pub sdk_file_id: String,
    #[serde(rename = "filesize", default)]
    pub file_size: i64,
    #[serde(rename = "play_length", default)]
    pub play_length: i64,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Card {
    #[serde(rename = "corpname", default)]
    pub corp_name: String,
    #[serde(rename = "userid", default)]
    pub user_id: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Location {
    #[serde(rename = "longitude", default)]
    pub longitude: f64,
    #[serde(rename = "latitude", default)]
    pub latitude: f64,
    #[serde(rename = "address", default)]
    pub address: String,
    #[serde(rename = "title", default)]
    pub title: String,
    #[serde(rename = "zoom", default)]
    pub zoom: i32,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Emotion {
    #[serde(rename = "type", default)]
    pub r#type: i32,
    #[serde(rename = "width", default)]
    pub width: i32,
    #[serde(rename = "height", default)]
    pub height: i32,
    #[serde(rename = "title", default)]
    pub title: String,
    #[serde(rename = "imagesize", default)]
    pub image_size: i32,
    #[serde(rename = "md5sum", default)]
    pub md5_sum: String,
    #[serde(rename = "sdkfileid", default)]
    pub sdk_file_id: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct File {
    #[serde(rename = "md5sum", default)]
    pub md5_sum: String,
    #[serde(rename = "filename", default)]
    pub file_name: String,
    #[serde(rename = "fileext", default)]
    pub file_ext: String,
    #[serde(rename = "sdkfileid", default)]
    pub sdk_file_id: String,
    #[serde(rename = "filesize", default)]
    pub file_size: i64,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Link {
    #[serde(rename = "title", default)]
    pub title: String,
    #[serde(rename = "description", default)]
    pub description: String,
    #[serde(rename = "link_url", default)]
    pub link_url: String,
    #[serde(rename = "image_url", default)]
    pub image_url: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Weapp {
    #[serde(rename = "title", default)]
    pub title: String,
    #[serde(rename = "description", default)]
    pub description: String,
    #[serde(rename = "username", default)]
    pub user_name: String,
    #[serde(rename = "displayname", default)]
    pub display_name: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ChatRecord {
    #[serde(rename = "item", default)]
    pub item: Vec<crate::bean::msgaudit::wx_cp_chat_model::ChatRecordItem>,
    #[serde(rename = "title", default)]
    pub title: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ChatRecordItem {
    #[serde(rename = "type", default)]
    pub r#type: String,
    #[serde(rename = "msgtime", default)]
    pub msg_time: i64,
    #[serde(rename = "content", default)]
    pub content: String,
    #[serde(rename = "from_chatroom", default)]
    pub from_chat_room: bool,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Collect {
    #[serde(rename = "room_name", default)]
    pub room_name: String,
    #[serde(rename = "creator", default)]
    pub creator: String,
    #[serde(rename = "create_time", default)]
    pub create_time: String,
    #[serde(rename = "title", default)]
    pub title: String,
    #[serde(rename = "details", default)]
    pub details: Vec<crate::bean::msgaudit::wx_cp_chat_model::Details>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Details {
    #[serde(rename = "id", default)]
    pub id: String,
    #[serde(rename = "ques", default)]
    pub ques: String,
    #[serde(rename = "type", default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Redpacket {
    #[serde(rename = "type", default)]
    pub r#type: i32,
    #[serde(rename = "totalcnt", default)]
    pub total_cnt: i32,
    #[serde(rename = "totalamount", default)]
    pub total_amount: i32,
    #[serde(rename = "wish", default)]
    pub wish: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Meeting {
    #[serde(rename = "topic", default)]
    pub topic: String,
    #[serde(rename = "starttime", default)]
    pub start_time: i64,
    #[serde(rename = "endtime", default)]
    pub end_time: i64,
    #[serde(rename = "address", default)]
    pub address: String,
    #[serde(rename = "remarks", default)]
    pub remarks: String,
    #[serde(rename = "meetingtype", default)]
    pub meeting_type: i32,
    #[serde(rename = "meetingid", default)]
    pub meeting_id: String,
    #[serde(rename = "status", default)]
    pub status: i32,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Doc {
    #[serde(rename = "title", default)]
    pub title: String,
    #[serde(rename = "doc_creator", default)]
    pub doc_creator: String,
    #[serde(rename = "link_url", default)]
    pub link_url: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Info {
    #[serde(rename = "content", default)]
    pub content: String,
    #[serde(rename = "item", default)]
    pub news_item: Vec<crate::bean::msgaudit::wx_cp_chat_model::NewsItem>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct NewsItem {
    #[serde(rename = "title", default)]
    pub title: String,
    #[serde(rename = "description", default)]
    pub description: String,
    #[serde(rename = "url", default)]
    pub url: String,
    #[serde(rename = "picurl", default)]
    pub pic_url: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Calendar {
    #[serde(rename = "title", default)]
    pub title: String,
    #[serde(rename = "creatorname", default)]
    pub creator_name: String,
    #[serde(rename = "attendeename", default)]
    pub attendee_name: Vec<String>,
    #[serde(rename = "starttime", default)]
    pub start_time: i64,
    #[serde(rename = "endtime", default)]
    pub end_time: i64,
    #[serde(rename = "place", default)]
    pub place: String,
    #[serde(rename = "remarks", default)]
    pub remarks: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Mixed {
    #[serde(rename = "item", default)]
    pub item: Vec<Item>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Item {
    #[serde(rename = "type", default)]
    pub r#type: String,
    #[serde(rename = "content", default)]
    pub content: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MeetingVoiceCall {
    #[serde(rename = "endtime", default)]
    pub end_time: i64,
    #[serde(rename = "sdkfileid", default)]
    pub sdk_file_id: String,
    #[serde(rename = "demofiledata", default)]
    pub demo_file_data: Vec<DemoFileData>,
    #[serde(rename = "sharescreendata", default)]
    pub share_screen_data: Vec<ShareScreenData>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DemoFileData {
    #[serde(rename = "filename", default)]
    pub file_name: String,
    #[serde(rename = "demooperator", default)]
    pub demo_operator: String,
    #[serde(rename = "starttime", default)]
    pub start_time: i64,
    #[serde(rename = "endtime", default)]
    pub end_time: i64,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ShareScreenData {
    #[serde(rename = "share", default)]
    pub share: String,
    #[serde(rename = "starttime", default)]
    pub start_time: i64,
    #[serde(rename = "endtime", default)]
    pub end_time: i64,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SphFeed {
    #[serde(rename = "feed_type", default)]
    pub feed_type: i32,
    #[serde(rename = "sph_name", default)]
    pub sph_name: String,
    #[serde(rename = "feed_desc", default)]
    pub feed_desc: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct VoipText {
    #[serde(rename = "callduration", default)]
    pub call_duration: i32,
    #[serde(rename = "invitetype", default)]
    pub invite_type: i32,
}

impl WxCpChatModel {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpChatModel 解析失败: {e}"))
    }
}

impl WxCpChatModel {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxCpChatModel 序列化失败: {e}"))
    }
}
