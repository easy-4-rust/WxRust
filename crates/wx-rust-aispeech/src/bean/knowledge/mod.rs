//! 知识库助理数据对象。
//!
//! 对应 Java `me.chanjar.weixin.aispeech.bean.knowledge` 包。

pub mod knowledge_info;
pub mod knowledge_list_result;
pub mod knowledge_manual_create_request;
pub mod knowledge_move_progress;
pub mod knowledge_move_request;
pub mod knowledge_tag_request;
pub mod knowledge_update_request;
pub mod knowledge_url_create_request;

pub use knowledge_info::KnowledgeInfo;
pub use knowledge_list_result::KnowledgeListResult;
pub use knowledge_manual_create_request::KnowledgeManualCreateRequest;
pub use knowledge_move_progress::KnowledgeMoveProgress;
pub use knowledge_move_request::KnowledgeMoveRequest;
pub use knowledge_tag_request::KnowledgeTagRequest;
pub use knowledge_update_request::KnowledgeUpdateRequest;
pub use knowledge_url_create_request::KnowledgeUrlCreateRequest;
