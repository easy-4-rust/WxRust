//! 视频号小店工具。
//!
//! 对应 Java `me.chanjar.weixin.channel.util` 包。
//!
//! Java 工具类处置（Wave 5.1 补齐/归类，依据 `docs/migration/weixin-java-channel/对象级对照表.md`）：
//! - [`wx_ch_crypt_utils::WxChCryptUtils`]：消息加解密（包装 common `WxCryptUtil`）
//!   与用户会话数据解密（AES-128-CBC + PKCS7），真实算法逻辑 → 实现；
//! - `JsonUtils` / `ResponseUtils` / `XmlUtils`：线格式/解码语义已内化
//!   （serde_json / quick-xml / 执行引擎与 `post_as` 解码辅助），台账登记
//!   `DEPENDENCY_REUSED`，不再保留本地占位。

pub mod wx_ch_crypt_utils;

pub use wx_ch_crypt_utils::WxChCryptUtils;

#[cfg(test)]
mod tests {
    //! ChannelWxError（Java `me.chanjar.weixin.channel.common.ChannelWxError`）
    //! 归类 DEPENDENCY_REUSED 的验证性测试：Java @Deprecated 子类无 channel
    //! 特有字段，构造器语义（按 WxChannelErrorMsgEnum 翻译中文文案 + 回填
    //! errorMsgEn）由 common `WxError::from_json_with_type(WxType::Channel)`
    //! 完整承载；门面 `extract_access_token` 产线路径同款调用。

    use wx_rust_common::enums::WxType;
    use wx_rust_common::error::WxError;

    /// errcode 非 0：error_msg 译为 channel 错误表中文文案，原始 errmsg 回填
    /// error_msg_en（对应 Java `ChannelWxError(errorCode, errorMsgEn)` 构造器）。
    #[test]
    fn channel_wx_error_translates_and_backfills() {
        let err = WxError::from_json_with_type(
            r#"{"errcode":40003,"errmsg":"invalid openid"}"#,
            Some(WxType::Channel),
        );
        assert_eq!(err.error_code, 40003);
        // wx_channel_error_msg_enum::find_msg_by_code(40003) 的中文文案
        assert_eq!(err.error_msg.as_deref(), Some("请检查 openid 的正确性"));
        assert_eq!(err.error_msg_en.as_deref(), Some("invalid openid"));
    }

    /// errcode 0：不做翻译（Java `WxError` 成功态语义一致）。
    #[test]
    fn channel_wx_error_success_no_translation() {
        let err =
            WxError::from_json_with_type(r#"{"errcode":0,"errmsg":"ok"}"#, Some(WxType::Channel));
        assert_eq!(err.error_code, 0);
        assert_eq!(err.error_msg.as_deref(), Some("ok"));
        assert_eq!(err.error_msg_en, None);
    }
}
