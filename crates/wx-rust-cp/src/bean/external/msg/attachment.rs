//! 对应 Java `me.chanjar.weixin.cp.bean.external.msg.Attachment.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName
//! 覆盖保留）。Wave 5 C5 修正：组件字段类型对齐 Java 语义——`image`/`link`/
//! `miniprogram`/`video`/`file` 均为本包（`external::msg`）的组件类型
//! （生成脚本原先误引 intelligentrobot/oa 下的同名类型）；并补齐 Java
//! `setImage`/`setLink`/`setMiniProgram`/`setVideo`/`setFile` 链式 setter
//! 语义（设置组件的同时写入 `msgtype`，对应 Java 各 setter 内
//! `this.msgType = WxCpConsts.WelcomeMsgType.XXX`）。

use crate::bean::external::msg::{File, Image, Link, MiniProgram, Video};

/// 群发/欢迎语附件（对应 Java `Attachment`）。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Attachment {
    #[serde(rename = "msgtype", default)]
    pub msg_type: String,
    #[serde(rename = "image", default, skip_serializing_if = "Option::is_none")]
    pub image: Option<Image>,
    #[serde(rename = "link", default, skip_serializing_if = "Option::is_none")]
    pub link: Option<Link>,
    #[serde(
        rename = "miniprogram",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub mini_program: Option<MiniProgram>,
    #[serde(rename = "video", default, skip_serializing_if = "Option::is_none")]
    pub video: Option<Video>,
    #[serde(rename = "file", default, skip_serializing_if = "Option::is_none")]
    pub file: Option<File>,
}

impl Attachment {
    /// 设置图片（对应 Java `setImage(Image)`：同时写入
    /// `msgType = WelcomeMsgType.IMAGE`）。
    pub fn set_image(mut self, image: Image) -> Self {
        self.image = Some(image);
        self.msg_type = crate::constant::wx_cp_constants::welcome_msg_type::IMAGE.to_string();
        self
    }

    /// 设置图文（对应 Java `setLink(Link)`：同时写入
    /// `msgType = WelcomeMsgType.LINK`）。
    pub fn set_link(mut self, link: Link) -> Self {
        self.link = Some(link);
        self.msg_type = crate::constant::wx_cp_constants::welcome_msg_type::LINK.to_string();
        self
    }

    /// 设置小程序（对应 Java `setMiniProgram(MiniProgram)`：同时写入
    /// `msgType = WelcomeMsgType.MINIPROGRAM`）。
    pub fn set_mini_program(mut self, mini_program: MiniProgram) -> Self {
        self.mini_program = Some(mini_program);
        self.msg_type = crate::constant::wx_cp_constants::welcome_msg_type::MINIPROGRAM.to_string();
        self
    }

    /// 设置视频（对应 Java `setVideo(Video)`：同时写入
    /// `msgType = WelcomeMsgType.VIDEO`）。
    pub fn set_video(mut self, video: Video) -> Self {
        self.video = Some(video);
        self.msg_type = crate::constant::wx_cp_constants::welcome_msg_type::VIDEO.to_string();
        self
    }

    /// 设置文件（对应 Java `setFile(File)`：同时写入
    /// `msgType = WelcomeMsgType.FILE`）。
    pub fn set_file(mut self, file: File) -> Self {
        self.file = Some(file);
        self.msg_type = crate::constant::wx_cp_constants::welcome_msg_type::FILE.to_string();
        self
    }

    /// 序列化为 JSON（对应 Java Gson `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("Attachment 序列化失败: {e}"))
    }
}
