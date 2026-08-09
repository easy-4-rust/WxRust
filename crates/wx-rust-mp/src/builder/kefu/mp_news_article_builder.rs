//! MpNewsArticleBuilder（对应 Java `me.chanjar.weixin.mp.builder.kefu.MpNewsArticleBuilder`）。
//!
//! 线格式语义由 `WxMpKefuMessage::to_json`（msgType 分支）承载；
//! builder 包装 `KefuMessageBuilder` 提供链式 API。

use crate::bean::kefu::{KefuMessageBuilder, MsgMenu, WxArticle, WxMpKefuMessage};

crate::kefu_builder!(MpNewsArticleBuilder, "mpnewsarticle");
