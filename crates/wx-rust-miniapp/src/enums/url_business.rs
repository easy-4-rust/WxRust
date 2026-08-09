//! 小程序业务接口地址（门面业务方法使用）。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.constant.WxMaApiUrlConstants` 中门面
//! 业务方法（用户/安全/二维码/订阅消息/消息/网络/链接）使用的各子域地址。
//! 函数风格与 `url_core` 一致：config 参数 + api_host 前缀模式（自定义域名
//! 替换由执行引擎在 token 注入时统一处理）。各业务子域完整地址表随对应
//! 子服务批次补齐。

use crate::config::{DEFAULT_API_HOST_URL, WxMaConfig};

/// 生成完整接口地址：域名前缀 + 路径。
fn url(_config: &dyn WxMaConfig, host: &str, path: &str) -> String {
    format!("{host}{path}")
}

/// 用户相关接口地址（对应 Java `WxMaApiUrlConstants.User`）。
pub mod user {
    use super::*;

    /// 获取手机号（对应 Java `User.GET_PHONE_NUMBER_URL`）。
    pub fn get_phone_number_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/wxa/business/getuserphonenumber")
    }

    /// 设置用户数据（对应 Java `User.SET_USER_STORAGE`，`%s` 依次为
    /// appid/signature/openid，`sig_method` 固定 `hmac_sha256`）。
    pub fn set_user_storage_url(
        config: &dyn WxMaConfig,
        appid: &str,
        signature: &str,
        openid: &str,
    ) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            &format!(
                "/wxa/set_user_storage?appid={appid}&signature={signature}&openid={openid}&sig_method=hmac_sha256"
            ),
        )
    }

    /// 多端登录验证（对应 Java `User.CODE_2_VERIFY_INFO_URL`）。
    pub fn code2_verify_info_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/wxa/sec/checkcode2verifyinfo")
    }

    /// 检查登录态（对应 Java `User.CHECK_SESSION_KEY_URL`，`%s` 依次为
    /// openid/signature，`sig_method` 固定 `hmac_sha256`）。
    pub fn check_session_key_url(config: &dyn WxMaConfig, openid: &str, signature: &str) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            &format!(
                "/wxa/checksessionkey?openid={openid}&signature={signature}&sig_method=hmac_sha256"
            ),
        )
    }
}

/// 内容安全检测接口地址（对应 Java `WxMaApiUrlConstants.SecCheck`）。
pub mod sec_check {
    use super::*;

    /// 图片安全检测（对应 Java `SecCheck.IMG_SEC_CHECK_URL`）。
    pub fn img_sec_check_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/wxa/img_sec_check")
    }

    /// 文本安全检测（对应 Java `SecCheck.MSG_SEC_CHECK_URL`）。
    pub fn msg_sec_check_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/wxa/msg_sec_check")
    }

    /// 媒体安全异步检测（对应 Java `SecCheck.MEDIA_CHECK_ASYNC_URL`）。
    pub fn media_check_async_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/wxa/media_check_async")
    }

    /// 获取用户安全等级（对应 Java `SecCheck.GET_USER_RISK_RANK`）。
    pub fn get_user_risk_rank_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/wxa/getuserriskrank")
    }
}

/// 小程序码/二维码接口地址（对应 Java `WxMaApiUrlConstants.Qrcode`）。
pub mod qrcode {
    use super::*;

    /// 获取小程序二维码（对应 Java `Qrcode.CREATE_QRCODE_URL`）。
    pub fn create_qrcode_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/wxaapp/createwxaqrcode")
    }

    /// 获取小程序码（对应 Java `Qrcode.GET_WXACODE_URL`）。
    pub fn get_wxacode_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/wxa/getwxacode")
    }

    /// 获取不限制数量的小程序码（对应 Java `Qrcode.GET_WXACODE_UNLIMIT_URL`）。
    pub fn get_wxacode_unlimit_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/wxa/getwxacodeunlimit")
    }
}

/// 订阅消息接口地址（对应 Java `WxMaApiUrlConstants.Subscribe`）。
pub mod subscribe {
    use super::*;

    /// 获取订阅消息公共模板标题列表（对应 Java `Subscribe.GET_PUB_TEMPLATE_TITLE_LIST_URL`）。
    pub fn get_pub_template_title_list_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/wxaapi/newtmpl/getpubtemplatetitles")
    }

    /// 获取模板标题下的关键词列表（对应 Java `Subscribe.GET_PUB_TEMPLATE_KEY_WORDS_BY_ID_URL`）。
    pub fn get_pub_template_keywords_by_id_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/wxaapi/newtmpl/getpubtemplatekeywords",
        )
    }

    /// 组合模板并添加至账号下的个人模板库（对应 Java `Subscribe.TEMPLATE_ADD_URL`）。
    pub fn add_template_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/wxaapi/newtmpl/addtemplate")
    }

    /// 获取当前账号下的个人模板列表（对应 Java `Subscribe.TEMPLATE_LIST_URL`）。
    pub fn template_list_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/wxaapi/newtmpl/gettemplate")
    }

    /// 删除账号下的某个模板（对应 Java `Subscribe.TEMPLATE_DEL_URL`）。
    pub fn del_template_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/wxaapi/newtmpl/deltemplate")
    }

    /// 获取小程序账号的类目（对应 Java `Subscribe.GET_CATEGORY_URL`）。
    pub fn get_category_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/wxaapi/newtmpl/getcategory")
    }

    /// 发送订阅消息（对应 Java `Subscribe.SUBSCRIBE_MSG_SEND_URL`）。
    pub fn subscribe_msg_send_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/message/subscribe/send")
    }

    /// 激活与更新服务卡片（对应 Java `Subscribe.SERVICE_NOTIFY_SET_URL`）。
    pub fn set_user_notify_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/wxa/setusernotify")
    }

    /// 更新服务卡片扩展信息（对应 Java `Subscribe.SERVICE_NOTIFY_SET_EXT_URL`）。
    pub fn set_user_notify_ext_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/wxa/setusernotifyext")
    }

    /// 查询服务卡片状态（对应 Java `Subscribe.SERVICE_NOTIFY_GET_URL`）。
    pub fn get_user_notify_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/wxa/getusernotify")
    }
}

/// 消息发送接口地址（对应 Java `WxMaApiUrlConstants.Msg`）。
pub mod msg {
    use super::*;

    /// 发送客服消息（对应 Java `Msg.KEFU_MESSAGE_SEND_URL`）。
    pub fn kefu_message_send_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/message/custom/send")
    }

    /// 发送订阅消息（对应 Java `Msg.SUBSCRIBE_MSG_SEND_URL`，与 Subscribe 域同地址）。
    pub fn subscribe_msg_send_url(config: &dyn WxMaConfig) -> String {
        super::subscribe::subscribe_msg_send_url(config)
    }

    /// 下发模板消息（对应 Java `Msg.UNIFORM_MSG_SEND_URL`）。
    pub fn uniform_msg_send_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/cgi-bin/message/wxopen/template/uniform_send",
        )
    }

    /// 创建被分享动态消息的 activity_id（对应 Java `Msg.ACTIVITY_ID_CREATE_URL`）。
    pub fn activity_id_create_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/cgi-bin/message/wxopen/activityid/create",
        )
    }

    /// 修改被分享的动态消息（对应 Java `Msg.UPDATABLE_MSG_SEND_URL`）。
    pub fn updatable_msg_send_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/cgi-bin/message/wxopen/updatablemsg/send",
        )
    }
}

/// 服务端网络接口地址（对应 Java `WxMaApiUrlConstants.Internet`）。
pub mod internet {
    use super::*;

    /// 获取用户加密 key（对应 Java `Internet.GET_USER_ENCRYPT_KEY`）。
    pub fn get_user_encrypt_key_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/wxa/business/getuserencryptkey")
    }
}

/// 链接接口地址（对应 Java `WxMaApiUrlConstants.Link` 与 `ShortLink`）。
pub mod link {
    use super::*;

    /// 生成 URL Link（对应 Java `Link.GENERATE_URLLINK_URL`）。
    pub fn generate_url_link_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/wxa/generate_urllink")
    }

    /// 查询 URL Link 信息（对应 Java `Link.QUERY_URLLINK_URL`）。
    pub fn query_url_link_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/wxa/query_urllink")
    }

    /// 生成短链接（对应 Java `ShortLink.GENERATE_SHORT_LINK_URL`）。
    pub fn generate_short_link_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/wxa/genwxashortlink")
    }
}

/// 默认 API 域名字面量（与 `url_core::API_HOST` 一致，供本模块内部使用）。
#[allow(unused)]
const API_HOST: &str = DEFAULT_API_HOST_URL;
