//! 对话机器人服务接口。
//!
//! 对应 Java `me.chanjar.weixin.aispeech.api.WxAispeechDialogService`。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::dialog::{
    AsyncTaskResult, BotIntent, DialogQueryRequest, DialogResult, PublishProgress,
};

/// 对话机器人服务。
#[async_trait]
pub trait WxAispeechDialogService: Send + Sync {
    /// 获取 OpenAI token（对应 Java `getAccessToken(String appid, String
    /// account)`）。
    ///
    /// POST `/v2/token`（`X-APPID` 头，appid 为空时回落配置 appid）；
    /// 成功后写入配置存储 `open_ai_token`（对应 Java `setOpenAiToken`）。
    async fn get_access_token(
        &self,
        appid: Option<&str>,
        account: Option<&str>,
    ) -> Result<String, WxErrorException>;

    /// 导入 bot JSON（对应 Java `importBotJson(int mode, List<BotIntent>
    /// data)`）。
    ///
    /// POST `/v2/bot/import/json`（`X-OPENAI-TOKEN` 头），返回任务 id
    /// （`data.task_id`）。
    async fn import_bot_json(
        &self,
        mode: i32,
        data: &[BotIntent],
    ) -> Result<String, WxErrorException>;

    /// 发布 bot（对应 Java `publishBot()`）。
    ///
    /// POST `/v2/bot/publish`，返回 `request_id`。
    async fn publish_bot(&self) -> Result<String, WxErrorException>;

    /// 查询发布进度（对应 Java `getPublishProgress(String env)`）。
    async fn get_publish_progress(&self, env: &str) -> Result<PublishProgress, WxErrorException>;

    /// 查询异步任务（对应 Java `queryAsyncTask(String taskId)`）。
    async fn query_async_task(&self, task_id: &str) -> Result<AsyncTaskResult, WxErrorException>;

    /// 对话查询（对应 Java `query(DialogQueryRequest request)`）。
    ///
    /// 请求体先经 AES-CBC 加密（密钥为配置 aesKey）；响应非 JSON 时报文
    /// 解密后解析。`answer` 形如 JSON 时解析为 `raw_answer`
    /// （对应 Java `setRawAnswer`）。
    async fn query(&self, request: &DialogQueryRequest) -> Result<DialogResult, WxErrorException>;
}
