//! 对应 Java `me.chanjar.weixin.mp.bean.draft` 包（生成）。

pub mod wx_mp_add_draft;
pub mod wx_mp_draft_articles;
pub mod wx_mp_draft_cover_info;
pub mod wx_mp_draft_image_info;
pub mod wx_mp_draft_info;
pub mod wx_mp_draft_item;
pub mod wx_mp_draft_list;
pub mod wx_mp_draft_product_info;
pub mod wx_mp_update_draft;

pub use wx_mp_add_draft::WxMpAddDraft;
pub use wx_mp_draft_articles::WxMpDraftArticles;
pub use wx_mp_draft_cover_info::CropPercent;
pub use wx_mp_draft_cover_info::WxMpDraftCoverInfo;
pub use wx_mp_draft_image_info::ImageItem;
pub use wx_mp_draft_image_info::WxMpDraftImageInfo;
pub use wx_mp_draft_info::WxMpDraftInfo;
pub use wx_mp_draft_item::WxMpDraftItem;
pub use wx_mp_draft_list::WxMpDraftList;
pub use wx_mp_draft_product_info::FooterProductInfo;
pub use wx_mp_draft_product_info::WxMpDraftProductInfo;
pub use wx_mp_update_draft::WxMpUpdateDraft;
