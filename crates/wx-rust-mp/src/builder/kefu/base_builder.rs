//! 客服消息 builder 基类与生成宏。
//!
//! 对应 Java `me.chanjar.weixin.mp.builder.kefu.BaseBuilder`。
//! Rust 中以 `KefuMessageBuilder` 统一承载（链式 to_user/content/media_id 等）。

pub use crate::bean::kefu::{KefuMessageBuilder, MsgMenu, WxArticle, WxMpKefuMessage};

/// 生成 kefu builder 的宏：包装 `KefuMessageBuilder` 并委托链式方法。
#[macro_export]
macro_rules! kefu_builder {
    ($name:ident, $msg_type:literal) => {
        /// 客服消息 builder（对应 Java `$name`）。
        pub struct $name(KefuMessageBuilder);

        impl $name {
            /// 按消息类型构建（对应 Java 静态工厂语义）。
            pub fn start() -> Self {
                Self(KefuMessageBuilder::new($msg_type))
            }

            /// 设置接收者 openid。
            pub fn to_user(mut self, v: impl Into<String>) -> Self {
                self.0 = self.0.to_user(v);
                self
            }

            /// 设置文本内容。
            pub fn content(mut self, v: impl Into<String>) -> Self {
                self.0 = self.0.content(v);
                self
            }

            /// 设置素材 media_id。
            pub fn media_id(mut self, v: impl Into<String>) -> Self {
                self.0 = self.0.media_id(v);
                self
            }

            /// 设置视频缩略图 media_id。
            pub fn thumb_media_id(mut self, v: impl Into<String>) -> Self {
                self.0 = self.0.thumb_media_id(v);
                self
            }

            /// 设置标题。
            pub fn title(mut self, v: impl Into<String>) -> Self {
                self.0 = self.0.title(v);
                self
            }

            /// 设置描述。
            pub fn description(mut self, v: impl Into<String>) -> Self {
                self.0 = self.0.description(v);
                self
            }

            /// 设置音乐链接。
            pub fn music_url(mut self, v: impl Into<String>) -> Self {
                self.0 = self.0.music_url(v);
                self
            }

            /// 设置高品质音乐链接。
            pub fn hq_music_url(mut self, v: impl Into<String>) -> Self {
                self.0 = self.0.hq_music_url(v);
                self
            }

            /// 添加图文文章。
            pub fn add_article(mut self, v: WxArticle) -> Self {
                self.0 = self.0.add_article(v);
                self
            }

            /// 设置小程序 appid。
            pub fn app_id(mut self, v: impl Into<String>) -> Self {
                self.0 = self.0.app_id(v);
                self
            }

            /// 设置小程序页面路径。
            pub fn page_path(mut self, v: impl Into<String>) -> Self {
                self.0 = self.0.page_path(v);
                self
            }

            /// 设置菜单消息菜单项。
            pub fn add_menus(mut self, v: Vec<MsgMenu>) -> Self {
                self.0 = self.0.add_menus(v);
                self
            }

            /// 设置菜单消息头部内容。
            pub fn head_content(mut self, v: impl Into<String>) -> Self {
                self.0 = self.0.head_content(v);
                self
            }

            /// 设置菜单消息尾部内容。
            pub fn tail_content(mut self, v: impl Into<String>) -> Self {
                self.0 = self.0.tail_content(v);
                self
            }

            /// 设置发布接口 article_id。
            pub fn article_id(mut self, v: impl Into<String>) -> Self {
                self.0 = self.0.article_id(v);
                self
            }

            /// 设置卡券 id。
            pub fn card_id(mut self, v: impl Into<String>) -> Self {
                self.0 = self.0.card_id(v);
                self
            }

            /// 构建消息。
            pub fn build(self) -> WxMpKefuMessage {
                self.0.build()
            }
        }
    };
}
