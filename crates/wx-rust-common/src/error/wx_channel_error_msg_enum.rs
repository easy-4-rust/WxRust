//! 微信视频号全局返回码错误信息表。
//!
//! 对应 Java `me.chanjar.weixin.common.error.WxChannelErrorMsgEnum`，
//! 由 `scripts/gen_error_msg_enums.py` 从 Java 源码自动生成（27 条错误码）。

/// 按错误码查找微信视频号中文错误信息。
///
/// # 参数
/// - `code`：微信错误码
///
/// # 返回
/// 错误码对应的中文信息；未收录时返回 `None`。
pub fn find_msg_by_code(code: i32) -> Option<&'static str> {
    match code {
        -1 => Some("系统繁忙，此时请开发者稍候再试"),
        0 => Some("请求成功"),
        40001 => Some(
            "获取 access_token 时 AppSecret 错误，或者 access_token 无效。请开发者认真检查 AppSecret 的正确性",
        ),
        40003 => Some("请检查 openid 的正确性"),
        40013 => Some("请检查 appid 的正确性，避免异常字符，注意大小写"),
        40066 => Some("请检查API的URL是否与文档一致"),
        41001 => Some("缺少 access_token 参数"),
        41002 => Some("请检查URL参数中是否有 ?appid="),
        41018 => Some("请检查POST json中是否包含component_ appid宇段"),
        42001 => Some("access_token失效，需要重新获取新的access_token"),
        43002 => Some("请检查发起API请求的Method是否为POST"),
        43003 => Some("请使用HTTPS方式清求，不要使用HTTP方式"),
        44002 => Some("POST 的数据包为空"),
        45002 => Some("请对数据进行压缩"),
        45009 => Some(
            "查看调用次数是否符合预期，可通过get_api_quota接口获取每天的调用quota；用完后可通过clear_quota进行清空",
        ),
        45011 => Some("命中每分钟的频率限制"),
        45035 => Some("需要登录 channels.weixin.qq.com/shop 配置IP白名单"),
        47001 => Some("解析 JSON/XML 内容错误"),
        48001 => Some("没有该接口权限"),
        48004 => Some("接口被禁用"),
        50001 => Some("请找用户获取该api授权"),
        50002 => Some("请检查封禁原因"),
        61004 => Some("需要登录 channels.weixin.qq.com/shop 配置IP白名单"),
        61007 => Some("请检查第三方平台服务商检查已获取的授权集"),
        10080000 => Some("需要登录 channels.weixin.qq.com/shop 继续完成注销"),
        10080001 => Some("账号已注销"),
        10080002 => {
            Some("小店的视频号带货身份为达人号，不允许使用该功能，如需使用，请将带货身份修改为商家")
        }
        _ => None,
    }
}
