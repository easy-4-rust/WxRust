//! 会话内容存档服务。
//!
//! 对应 Java `me.chanjar.weixin.cp.api.WxCpMsgAuditService`。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::{
    WxCpAgreeInfo, WxCpChatData, WxCpChatDatas, WxCpChatModel, WxCpCheckAgreeRequest, WxCpGroupChat,
};

/// 会话内容存档服务。
///
/// 说明：Java 中部分方法将原生 `Finance` SDK 句柄（`long sdk`）暴露给
/// 调用方（已标注 `@Deprecated`），Rust 侧同样保留对应签名以严格镜像
/// 接口面；`Consumer<byte[]>` 以 `&mut dyn FnMut(&[u8])` 表达，ADAPTED。
#[async_trait]
pub trait WxCpMsgAuditService: Send + Sync {
    /// 拉取聊天记录（SDK 句柄版，对应 Java
    /// `WxCpMsgAuditService.getChatDatas(long, long, String, String, long)`，
    /// Java 中已 `@Deprecated`，推荐使用 `get_chat_records`）。
    async fn get_chat_datas(
        &self,
        seq: i64,
        limit: i64,
        proxy: Option<&str>,
        passwd: Option<&str>,
        timeout: i64,
    ) -> Result<WxCpChatDatas, WxErrorException>;

    /// 拉取聊天记录（推荐使用，对应 Java
    /// `WxCpMsgAuditService.getChatRecords(long, long, String, String, long)`；
    /// 不包含 SDK 信息）。
    async fn get_chat_records(
        &self,
        seq: i64,
        limit: i64,
        proxy: Option<&str>,
        passwd: Option<&str>,
        timeout: i64,
    ) -> Result<Vec<WxCpChatData>, WxErrorException>;

    /// 获取解密的聊天数据 Model（SDK 句柄版，对应 Java
    /// `WxCpMsgAuditService.getDecryptData(long, WxCpChatData, Integer)`，
    /// Java 中已 `@Deprecated`，推荐使用 `get_decrypt_chat_data`）。
    async fn get_decrypt_data(
        &self,
        sdk: i64,
        chat_data: &WxCpChatData,
        pkcs1: i32,
    ) -> Result<WxCpChatModel, WxErrorException>;

    /// 获取解密的聊天数据 Model（推荐使用，对应 Java
    /// `WxCpMsgAuditService.getDecryptChatData(WxCpChatData, Integer)`；
    /// `pkcs1`：1 使用 PKCS1 解密，2 使用 PKCS8 解密）。
    async fn get_decrypt_chat_data(
        &self,
        chat_data: &WxCpChatData,
        pkcs1: i32,
    ) -> Result<WxCpChatModel, WxErrorException>;

    /// 获取解密的聊天数据明文（SDK 句柄版，对应 Java
    /// `WxCpMsgAuditService.getChatPlainText(long, WxCpChatData, Integer)`，
    /// Java 中已 `@Deprecated`，推荐使用 `get_chat_record_plain_text`）。
    async fn get_chat_plain_text(
        &self,
        sdk: i64,
        chat_data: &WxCpChatData,
        pkcs1: i32,
    ) -> Result<String, WxErrorException>;

    /// 获取解密的聊天数据明文（推荐使用，对应 Java
    /// `WxCpMsgAuditService.getChatRecordPlainText(WxCpChatData, Integer)`）。
    async fn get_chat_record_plain_text(
        &self,
        chat_data: &WxCpChatData,
        pkcs1: i32,
    ) -> Result<String, WxErrorException>;

    /// 获取媒体文件（SDK 句柄版，写入目标文件，对应 Java
    /// `WxCpMsgAuditService.getMediaFile(long, String, String, String, long,
    /// String)`，Java 中已 `@Deprecated`，推荐使用
    /// `download_media_file`）。
    async fn get_media_file(
        &self,
        sdk: i64,
        sdkfileid: &str,
        proxy: Option<&str>,
        passwd: Option<&str>,
        timeout: i64,
        target_file_path: &str,
    ) -> Result<(), WxErrorException>;

    /// 获取媒体文件（推荐使用，写入目标文件，对应 Java
    /// `WxCpMsgAuditService.downloadMediaFile(String, String, String, long,
    /// String)`）。
    async fn download_media_file(
        &self,
        sdkfileid: &str,
        proxy: Option<&str>,
        passwd: Option<&str>,
        timeout: i64,
        target_file_path: &str,
    ) -> Result<(), WxErrorException>;

    /// 获取媒体文件（SDK 句柄版 + 分片回调，对应 Java
    /// `WxCpMsgAuditService.getMediaFile(long, String, String, String, long,
    /// Consumer<byte[]>)`，Java 中已 `@Deprecated`，推荐使用
    /// `download_media_file_with_callback`；Java `Consumer<byte[]>` 以
    /// `&mut dyn FnMut(&[u8])` 表达，ADAPTED）。
    async fn get_media_file_with_callback(
        &self,
        sdk: i64,
        sdkfileid: &str,
        proxy: Option<&str>,
        passwd: Option<&str>,
        timeout: i64,
        action: &mut dyn FnMut(&[u8]),
    ) -> Result<(), WxErrorException>;

    /// 获取媒体文件（推荐使用 + 分片回调，对应 Java
    /// `WxCpMsgAuditService.downloadMediaFile(String, String, String, long,
    /// Consumer<byte[]>)`；Java `Consumer<byte[]>` 以
    /// `&mut dyn FnMut(&[u8])` 表达，ADAPTED）。
    async fn download_media_file_with_callback(
        &self,
        sdkfileid: &str,
        proxy: Option<&str>,
        passwd: Option<&str>,
        timeout: i64,
        action: &mut dyn FnMut(&[u8]),
    ) -> Result<(), WxErrorException>;

    /// 获取会话内容存档开启成员列表（对应 Java
    /// `WxCpMsgAuditService.getPermitUserList(Integer)`；`type`：1-办公版，
    /// 2-服务版，3-企业版；不填返回全量）。
    async fn get_permit_user_list(
        &self,
        r#type: Option<i32>,
    ) -> Result<Vec<String>, WxErrorException>;

    /// 获取会话内容存档内部群信息（对应 Java
    /// `WxCpMsgAuditService.getGroupChat(String)`）。
    async fn get_group_chat(&self, roomid: &str) -> Result<WxCpGroupChat, WxErrorException>;

    /// 获取会话同意情况（单聊，对应 Java
    /// `WxCpMsgAuditService.checkSingleAgree(WxCpCheckAgreeRequest)`）。
    async fn check_single_agree(
        &self,
        check_agree_request: &WxCpCheckAgreeRequest,
    ) -> Result<WxCpAgreeInfo, WxErrorException>;

    /// 关闭当前线程持有的 SDK，释放本地资源（对应 Java
    /// `WxCpMsgAuditService.closeThreadLocalSdk()`；Java 无异常，同步方法）。
    fn close_thread_local_sdk(&self);

    /// 关闭所有会话存档 SDK 实例，释放全部原生资源（对应 Java
    /// `WxCpMsgAuditService.closeAllSdks()`；Java 无异常，同步方法）。
    fn close_all_sdks(&self);
}
