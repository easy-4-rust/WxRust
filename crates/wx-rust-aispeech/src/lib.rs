//! WxRust 智能对话模块（对应 WxJava `weixin-java-aispeech`）。
//!
//! 覆盖 `me.chanjar.weixin.aispeech.*`：对话机器人（dialog，含 token 获取、
//! bot 导入/发布、异步任务、问答查询）与知识库助理（knowledge，文件/URL/
//! 手工创建、列表/检索、移动、标签）两大子域，以及独立的签名/AES 工具
//! （`WxAispeechSignUtil`）。认证采用非 access_token 的请求头签名体系：
//! dialog 走 MD5 请求签名（`sign`）+ `X-APPID`/`X-OPENAI-TOKEN` 头，
//! knowledge 走 HmacSHA256 签名（`X-Signature`）+ `X-Request-ID` 头。

#![forbid(unsafe_code)]

pub mod api;
pub mod bean;
pub mod config;
pub mod util;
