//! 群发/欢迎语附件构建器。
//!
//! 对应 Java `me.chanjar.weixin.cp.bean.external.msg.AttachmentBuilder`：
//! Lombok `@Builder` 为 `image`/`video`/`file`/`link`/`miniProgram` 五个
//! 私有静态方法生成的链式 builder（`ImageBuilder`/`VideoBuilder`/
//! `FileBuilder`/`LinkBuilder`/`MiniProgramBuilder`），以
//! `AttachmentBuilder.imageBuilder()` 等工厂方法进入，`build()` 产出
//! `Attachment`（组件字段 + `msgtype` 由 `Attachment::set_*` 写入）。
//!
//! 使用方法（对应 Java）：
//! ```ignore
//! let attachment = AttachmentBuilder::image_builder()
//!     .media_id("MEDIA_ID").pic_url("URL").build();
//! ```

use crate::bean::external::msg::{Attachment, File, Image, Link, MiniProgram, Video};

/// 附件构建器门面（对应 Java `AttachmentBuilder`，仅有静态工厂方法）。
pub struct AttachmentBuilder;

impl AttachmentBuilder {
    /// 图片附件 builder（对应 Java `imageBuilder()`）。
    pub fn image_builder() -> ImageBuilder {
        ImageBuilder::default()
    }

    /// 视频附件 builder（对应 Java `videoBuilder()`）。
    pub fn video_builder() -> VideoBuilder {
        VideoBuilder::default()
    }

    /// 文件附件 builder（对应 Java `fileBuilder()`）。
    pub fn file_builder() -> FileBuilder {
        FileBuilder::default()
    }

    /// 图文附件 builder（对应 Java `linkBuilder()`）。
    pub fn link_builder() -> LinkBuilder {
        LinkBuilder::default()
    }

    /// 小程序附件 builder（对应 Java `miniProgramBuilder()`）。
    pub fn mini_program_builder() -> MiniProgramBuilder {
        MiniProgramBuilder::default()
    }
}

/// 图片附件 builder（对应 Java `AttachmentBuilder.ImageBuilder`）。
#[derive(Debug, Clone, Default)]
pub struct ImageBuilder {
    media_id: Option<String>,
    pic_url: Option<String>,
}

impl ImageBuilder {
    /// 媒体文件 id（对应 Java `mediaId(String)`）。
    pub fn media_id(mut self, media_id: impl Into<String>) -> Self {
        self.media_id = Some(media_id.into());
        self
    }

    /// 图片链接（对应 Java `picUrl(String)`）。
    pub fn pic_url(mut self, pic_url: impl Into<String>) -> Self {
        self.pic_url = Some(pic_url.into());
        self
    }

    /// 构建 `Attachment`（对应 Java `build()`）。
    pub fn build(self) -> Attachment {
        let image = Image {
            media_id: self.media_id.unwrap_or_default(),
            pic_url: self.pic_url.unwrap_or_default(),
        };
        Attachment::default().set_image(image)
    }
}

/// 视频附件 builder（对应 Java `AttachmentBuilder.VideoBuilder`）。
#[derive(Debug, Clone, Default)]
pub struct VideoBuilder {
    media_id: Option<String>,
}

impl VideoBuilder {
    /// 媒体文件 id（对应 Java `mediaId(String)`）。
    pub fn media_id(mut self, media_id: impl Into<String>) -> Self {
        self.media_id = Some(media_id.into());
        self
    }

    /// 构建 `Attachment`（对应 Java `build()`）。
    pub fn build(self) -> Attachment {
        let video = Video {
            media_id: self.media_id.unwrap_or_default(),
            thumb_media_id: String::new(),
        };
        Attachment::default().set_video(video)
    }
}

/// 文件附件 builder（对应 Java `AttachmentBuilder.FileBuilder`）。
#[derive(Debug, Clone, Default)]
pub struct FileBuilder {
    media_id: Option<String>,
}

impl FileBuilder {
    /// 媒体文件 id（对应 Java `mediaId(String)`）。
    pub fn media_id(mut self, media_id: impl Into<String>) -> Self {
        self.media_id = Some(media_id.into());
        self
    }

    /// 构建 `Attachment`（对应 Java `build()`）。
    pub fn build(self) -> Attachment {
        let file = File {
            media_id: self.media_id.unwrap_or_default(),
        };
        Attachment::default().set_file(file)
    }
}

/// 图文附件 builder（对应 Java `AttachmentBuilder.LinkBuilder`）。
#[derive(Debug, Clone, Default)]
pub struct LinkBuilder {
    title: Option<String>,
    url: Option<String>,
    pic_url: Option<String>,
    desc: Option<String>,
}

impl LinkBuilder {
    /// 图文标题（对应 Java `title(String)`）。
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// 图文链接（对应 Java `url(String)`）。
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    /// 封面图片链接（对应 Java `picUrl(String)`）。
    pub fn pic_url(mut self, pic_url: impl Into<String>) -> Self {
        self.pic_url = Some(pic_url.into());
        self
    }

    /// 图文描述（对应 Java `desc(String)`）。
    pub fn desc(mut self, desc: impl Into<String>) -> Self {
        self.desc = Some(desc.into());
        self
    }

    /// 构建 `Attachment`（对应 Java `build()`）。
    pub fn build(self) -> Attachment {
        let link = Link {
            title: self.title.unwrap_or_default(),
            pic_url: self.pic_url.unwrap_or_default(),
            desc: self.desc.unwrap_or_default(),
            url: self.url.unwrap_or_default(),
            media_id: String::new(),
        };
        Attachment::default().set_link(link)
    }
}

/// 小程序附件 builder（对应 Java `AttachmentBuilder.MiniProgramBuilder`）。
#[derive(Debug, Clone, Default)]
pub struct MiniProgramBuilder {
    title: Option<String>,
    pic_media_id: Option<String>,
    app_id: Option<String>,
    page: Option<String>,
}

impl MiniProgramBuilder {
    /// 小程序消息标题（对应 Java `title(String)`）。
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// 小程序消息封面的 mediaid（对应 Java `picMediaId(String)`）。
    pub fn pic_media_id(mut self, pic_media_id: impl Into<String>) -> Self {
        self.pic_media_id = Some(pic_media_id.into());
        self
    }

    /// 小程序 appid（对应 Java `appId(String)`）。
    pub fn app_id(mut self, app_id: impl Into<String>) -> Self {
        self.app_id = Some(app_id.into());
        self
    }

    /// 小程序 page 路径（对应 Java `page(String)`）。
    pub fn page(mut self, page: impl Into<String>) -> Self {
        self.page = Some(page.into());
        self
    }

    /// 构建 `Attachment`（对应 Java `build()`）。
    pub fn build(self) -> Attachment {
        let mini_program = MiniProgram {
            title: self.title.unwrap_or_default(),
            pic_media_id: self.pic_media_id.unwrap_or_default(),
            appid: self.app_id.unwrap_or_default(),
            page: self.page.unwrap_or_default(),
        };
        Attachment::default().set_mini_program(mini_program)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 镜像 Java `AttachmentBuilder` 各 builder 的 `build()` 语义：
    /// 组件字段写入 + `msgtype` 写入（WxCpConsts.WelcomeMsgType 取值）。
    #[test]
    fn attachment_builder_sets_msgtype_and_component() {
        let image_attachment = AttachmentBuilder::image_builder()
            .media_id("media-1")
            .pic_url("https://example.com/a.png")
            .build();
        assert_eq!(image_attachment.msg_type, "image");
        assert_eq!(
            image_attachment.image,
            Some(Image {
                media_id: "media-1".into(),
                pic_url: "https://example.com/a.png".into(),
            })
        );

        let video_attachment = AttachmentBuilder::video_builder()
            .media_id("media-2")
            .build();
        assert_eq!(video_attachment.msg_type, "video");
        assert_eq!(
            video_attachment.video,
            Some(Video {
                media_id: "media-2".into(),
                thumb_media_id: String::new(),
            })
        );

        let file_attachment = AttachmentBuilder::file_builder()
            .media_id("media-3")
            .build();
        assert_eq!(file_attachment.msg_type, "file");
        assert_eq!(
            file_attachment.file,
            Some(File {
                media_id: "media-3".into(),
            })
        );

        let link_attachment = AttachmentBuilder::link_builder()
            .title("标题")
            .url("https://example.com")
            .pic_url("https://example.com/b.png")
            .desc("描述")
            .build();
        assert_eq!(link_attachment.msg_type, "link");
        assert_eq!(
            link_attachment.link,
            Some(Link {
                title: "标题".into(),
                pic_url: "https://example.com/b.png".into(),
                desc: "描述".into(),
                url: "https://example.com".into(),
                media_id: String::new(),
            })
        );

        let mini_program_attachment = AttachmentBuilder::mini_program_builder()
            .title("小程序")
            .pic_media_id("pic-1")
            .app_id("wx123")
            .page("pages/index")
            .build();
        assert_eq!(mini_program_attachment.msg_type, "miniprogram");
        assert_eq!(
            mini_program_attachment.mini_program,
            Some(MiniProgram {
                title: "小程序".into(),
                pic_media_id: "pic-1".into(),
                appid: "wx123".into(),
                page: "pages/index".into(),
            })
        );
    }

    /// 序列化线格式（Java Gson：组件字段名含 @SerializedName 覆盖）。
    #[test]
    fn attachment_json_line_format() {
        let attachment = AttachmentBuilder::image_builder()
            .media_id("media-1")
            .pic_url("https://example.com/a.png")
            .build();
        let json = attachment.to_json().unwrap();
        assert!(json.contains("\"msgtype\":\"image\""));
        assert!(json.contains("\"media_id\":\"media-1\""));
        assert!(json.contains("\"pic_url\":\"https://example.com/a.png\""));
    }
}
