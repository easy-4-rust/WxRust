//! 对应 Java `me.chanjar.weixin.mp.bean.material` 包（生成）。

pub mod wx_media_img_upload_result;
pub mod wx_mp_material;
pub mod wx_mp_material_article_update;
pub mod wx_mp_material_count_result;
pub mod wx_mp_material_file_batch_get_result;
pub mod wx_mp_material_news;
pub mod wx_mp_material_news_batch_get_result;
pub mod wx_mp_material_upload_result;
pub mod wx_mp_material_video_info_result;
pub mod wx_mp_news_article;

pub use wx_media_img_upload_result::WxMediaImgUploadResult;
pub use wx_mp_material::WxMpMaterial;
pub use wx_mp_material_article_update::WxMpMaterialArticleUpdate;
pub use wx_mp_material_count_result::WxMpMaterialCountResult;
pub use wx_mp_material_file_batch_get_result::WxMaterialFileBatchGetNewsItem;
pub use wx_mp_material_file_batch_get_result::WxMpMaterialFileBatchGetResult;
pub use wx_mp_material_news::WxMpMaterialNews;
pub use wx_mp_material_news_batch_get_result::WxMaterialNewsBatchGetNewsItem;
pub use wx_mp_material_news_batch_get_result::WxMpMaterialNewsBatchGetResult;
pub use wx_mp_material_upload_result::WxMpMaterialUploadResult;
pub use wx_mp_material_video_info_result::WxMpMaterialVideoInfoResult;
pub use wx_mp_news_article::WxMpNewsArticle;
