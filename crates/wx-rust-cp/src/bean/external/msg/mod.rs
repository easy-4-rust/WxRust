//! 对应 Java `me.chanjar.weixin.cp.bean.external/msg` 包（生成）。

pub mod attachment;
pub mod attachment_builder;
pub mod file;
pub mod image;
pub mod link;
pub mod location;
pub mod mini_program;
pub mod tag_filter;
pub mod tag_list;
pub mod text;
pub mod video;

pub use attachment::Attachment;
pub use attachment_builder::AttachmentBuilder;
pub use file::File;
pub use image::Image;
pub use link::Link;
pub use location::Location;
pub use mini_program::MiniProgram;
pub use tag_filter::TagFilter;
pub use tag_list::TagList;
pub use text::Text;
pub use video::Video;
