//! ID 转换相关接口地址。
//!
//! 对应 Java `WxCpApiPathConsts.IdConvert`。

/// 将企业主体下的客户标签 ID 转换成服务商主体下的客户标签 ID。
pub const EXTERNAL_TAG_ID: &str = "/cgi-bin/idconvert/external_tagid";
/// 将微信客户的 unionid 转为第三方主体的 external_userid。
/// 该接口有调用频率限制，当 subject_type 为 0 时，按企业作如下的限制：
/// 10 万次/小时、48 万次/天、750 万次/月。
pub const UNION_ID_TO_EXTERNAL_USER_ID: &str = "/cgi-bin/idconvert/unionid_to_external_userid";
/// 将企业主体下的微信客服 ID 转换成服务商主体下的微信客服 ID。
pub const OPEN_KF_ID: &str = "/cgi-bin/idconvert/open_kfid";
/// 将应用获取的外部用户临时 id（tmp_external_userid）转换为 external_userid。
pub const CONVERT_TMP_EXTERNAL_USER_ID: &str = "/cgi-bin/idconvert/convert_tmp_external_userid";
