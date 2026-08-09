//! 消息重复检查器接口。
//!
//! 对应 Java `me.chanjar.weixin.common.api.WxMessageDuplicateChecker`。

/// 消息重复检查器。
///
/// 微信服务器在五秒内收不到响应会断掉连接，并重新发起请求，总共重试三次。
/// 因此需要按消息 ID 判断是否重复，避免重复处理。
///
/// # 公众号的排重方式
/// - 普通消息：推荐使用 `msgid` 排重
/// - 事件消息：推荐使用 `FromUserName + CreateTime` 排重
///
/// # 企业号的排重方式
/// 官方文档未说明，参照公众号方式；简单方式为有 `MsgId` 用 `MsgId`，
/// 否则用 `FromUserName + CreateTime`。
pub trait WxMessageDuplicateChecker: Send + Sync {
    /// 判断消息是否重复。
    ///
    /// # 参数
    /// - `message_id`：按上述方式构造的消息 ID
    ///
    /// # 返回
    /// 如果是重复消息返回 `true`，否则返回 `false`
    fn is_duplicate(&self, message_id: &str) -> bool;
}
