//! 公众号 API 接口地址。
//!
//! 对应 Java `me.chanjar.weixin.mp.enums.WxMpApiUrl`。Java 为 1472 行的巨型
//! 枚举（每子域一组）；本批次实现核心子域使用的地址，其余随各自子服务批次补齐。

use crate::config::{API_DEFAULT_HOST_URL, OPEN_DEFAULT_HOST_URL, WxMpConfigStorage};

/// 生成完整接口地址：域名前缀 + 路径。
fn url(_config: &dyn WxMpConfigStorage, host: &str, path: &str) -> String {
    format!("{host}{path}")
}

/// 基础能力接口地址（对应 Java `WxMpApiUrl.Other`）。
pub mod other {
    use super::*;

    /// 获取 access_token。
    pub fn get_access_token_url(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            &format!(
                "/cgi-bin/token?grant_type=client_credential&appid={}&secret={}",
                config.app_id(),
                config.secret()
            ),
        )
    }

    /// 获取稳定版 access_token。
    pub fn get_stable_access_token_url(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/stable_token")
    }

    /// 获得各种类型的 ticket。
    pub fn get_ticket_url(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/ticket/getticket?type=")
    }

    /// 长链接转短链接接口。
    pub fn shorturl_api_url(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/shorturl")
    }

    /// 语义查询接口。
    pub fn semantic_semproxy_search_url(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/semantic/semproxy/search")
    }

    /// 获取微信服务器 IP 地址。
    pub fn get_callback_ip_url(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/getcallbackip")
    }

    /// 网络检测。
    pub fn netcheck_url(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/callback/check")
    }

    /// 第三方使用网站应用授权登录的 url（域名 + 路径；参数由调用方拼接）。
    pub fn qrconnect_url(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.open_host, "/connect/qrconnect")
    }

    /// 获取公众号的自动回复规则。
    pub fn get_current_autoreply_info_url(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/get_current_autoreply_info")
    }

    /// 公众号调用或第三方平台帮公众号调用对公众号的所有 api 调用次数清零。
    pub fn clear_quota_url(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/clear_quota")
    }

    /// 短 key 托管（生成短 key 的 url）。
    pub fn gen_shorten_url(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/shorten/gen")
    }

    /// 短 key 解析（解析短 key 的 url）。
    pub fn fetch_shorten_url(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/shorten/fetch")
    }
}

/// 菜单接口地址（对应 Java `WxMpApiUrl.Menu`）。
pub mod menu {
    use super::*;

    /// 创建菜单。
    pub fn menu_create(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/menu/create")
    }

    /// 获取菜单。
    pub fn menu_get(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/menu/get")
    }

    /// 删除菜单。
    pub fn menu_delete(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/menu/delete")
    }

    /// 获取自定义菜单配置。
    pub fn get_self_menu_info(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/get_current_selfmenu_info")
    }
}

/// 模板消息接口地址（对应 Java `WxMpApiUrl.TemplateMsg`）。
pub mod template_msg {
    use super::*;

    /// 发送模板消息。
    pub fn message_template_send(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/message/template/send")
    }
}

/// 二维码接口地址（对应 Java `WxMpApiUrl.Qrcode`）。
pub mod qrcode {
    use super::*;

    /// 创建二维码 ticket。
    pub fn qrcode_create(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/qrcode/create")
    }
}

/// 客服接口地址（对应 Java `WxMpApiUrl.Kefu`）。
pub mod kefu {
    use super::*;

    /// 发送客服消息。
    pub fn message_custom_send(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/message/custom/send")
    }

    /// /customservice/kfaccount/add
    pub fn kfaccount_add(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/customservice/kfaccount/add")
    }

    /// /customservice/kfaccount/update
    pub fn kfaccount_update(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/customservice/kfaccount/update")
    }

    /// /customservice/kfaccount/inviteworker
    pub fn kfaccount_invite_worker(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/customservice/kfaccount/inviteworker")
    }

    /// /customservice/kfaccount/del?kf_account=
    pub fn kfaccount_del(config: &dyn WxMpConfigStorage, kf_account: &str) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            &format!("/customservice/kfaccount/del?kf_account={kf_account}"),
        )
    }

    /// /customservice/kfsession/create
    pub fn kfsession_create(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/customservice/kfsession/create")
    }

    /// /customservice/kfsession/close
    pub fn kfsession_close(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/customservice/kfsession/close")
    }

    /// /customservice/kfsession/getsession?openid=
    pub fn kfsession_get(config: &dyn WxMpConfigStorage, openid: &str) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            &format!("/customservice/kfsession/getsession?openid={openid}"),
        )
    }

    /// /customservice/kfsession/getsessionlist?kf_account=
    pub fn kfsession_list(config: &dyn WxMpConfigStorage, kf_account: &str) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            &format!("/customservice/kfsession/getsessionlist?kf_account={kf_account}"),
        )
    }

    /// /customservice/kfsession/getwaitcase
    pub fn kfsession_get_wait_case(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/customservice/kfsession/getwaitcase")
    }

    /// /cgi-bin/customservice/getkflist
    pub fn getkflist(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/customservice/getkflist")
    }

    /// /cgi-bin/customservice/getonlinekflist
    pub fn getonlinekflist(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/cgi-bin/customservice/getonlinekflist",
        )
    }

    /// /cgi-bin/customservice/msgrecord/getmsglist
    pub fn getmsglist(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/cgi-bin/customservice/msgrecord/getmsglist",
        )
    }
}

/// 用户管理接口地址（对应 Java `WxMpApiUrl.User`）。
pub mod user {
    use super::*;

    /// 获取用户基本信息。
    pub fn user_info(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/user/info")
    }
}

/// 接口地址常量（仅保留核心地址；Java 默认域名字面量）。
pub const API_HOST: &str = API_DEFAULT_HOST_URL;
/// 开放平台域名字面量。
pub const OPEN_HOST: &str = OPEN_DEFAULT_HOST_URL;

/// tags 接口地址（对应 Java `WxMpApiUrl.tags`）。
pub mod tags {
    use super::*;

    /// /cgi-bin/tags/create
    pub fn create(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/tags/create")
    }

    /// /cgi-bin/tags/get
    pub fn get(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/tags/get")
    }

    /// /cgi-bin/tags/update
    pub fn update(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/tags/update")
    }

    /// /cgi-bin/tags/delete
    pub fn delete(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/tags/delete")
    }

    /// /cgi-bin/user/tag/get
    pub fn tag_user_get(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/user/tag/get")
    }

    /// /cgi-bin/tags/members/batchtagging
    pub fn batch_tagging(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/tags/members/batchtagging")
    }

    /// /cgi-bin/tags/members/batchuntagging
    pub fn batch_untagging(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/tags/members/batchuntagging")
    }

    /// /cgi-bin/tags/getidlist
    pub fn get_id_list(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/tags/getidlist")
    }
}

/// blacklist 接口地址（对应 Java `WxMpApiUrl.blacklist`）。
pub mod blacklist {
    use super::*;

    /// /cgi-bin/tags/members/getblacklist
    pub fn get_blacklist(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/tags/members/getblacklist")
    }

    /// /cgi-bin/tags/members/batchblacklist
    pub fn batch_blacklist(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/tags/members/batchblacklist")
    }

    /// /cgi-bin/tags/members/batchunblacklist
    pub fn batch_unblacklist(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/cgi-bin/tags/members/batchunblacklist",
        )
    }
}

/// store 接口地址（对应 Java `WxMpApiUrl.store`）。
pub mod store {
    use super::*;

    /// /cgi-bin/poi/addpoi
    pub fn poi_add(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/poi/addpoi")
    }

    /// /cgi-bin/poi/getpoi
    pub fn poi_get(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/poi/getpoi")
    }

    /// /cgi-bin/poi/delpoi
    pub fn poi_del(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/poi/delpoi")
    }

    /// /cgi-bin/poi/getpoilist
    pub fn poi_list(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/poi/getpoilist")
    }

    /// /cgi-bin/poi/updatepoi
    pub fn poi_update(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/poi/updatepoi")
    }

    /// /cgi-bin/poi/getwxcategory
    pub fn wx_category(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/poi/getwxcategory")
    }
}

/// comment 接口地址（对应 Java `WxMpApiUrl.comment`）。
pub mod comment {
    use super::*;

    /// /cgi-bin/comment/open
    pub fn open(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/comment/open")
    }

    /// /cgi-bin/comment/close
    pub fn close(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/comment/close")
    }

    /// /cgi-bin/comment/list
    pub fn list(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/comment/list")
    }

    /// /cgi-bin/comment/markelect
    pub fn mark_elect(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/comment/markelect")
    }

    /// /cgi-bin/comment/unmarkelect
    pub fn unmark_elect(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/comment/unmarkelect")
    }

    /// /cgi-bin/comment/delete
    pub fn delete(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/comment/delete")
    }

    /// /cgi-bin/comment/reply/add
    pub fn reply_add(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/comment/reply/add")
    }

    /// /cgi-bin/comment/reply/delete
    pub fn reply_delete(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/comment/reply/delete")
    }
}

/// datacube 接口地址（对应 Java `WxMpApiUrl.datacube`）。
pub mod datacube {
    use super::*;

    /// /datacube/getusersummary
    pub fn get_user_summary(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/datacube/getusersummary")
    }

    /// /datacube/getusercumulate
    pub fn get_user_cumulate(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/datacube/getusercumulate")
    }

    /// /datacube/getarticlesummary
    pub fn get_article_summary(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/datacube/getarticlesummary")
    }

    /// /datacube/getarticletotal
    pub fn get_article_total(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/datacube/getarticletotal")
    }
}

/// wifi 接口地址（对应 Java `WxMpApiUrl.wifi`）。
pub mod wifi {
    use super::*;

    /// /bizwifi/shop/list
    pub fn shop_list(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/bizwifi/shop/list")
    }

    /// /bizwifi/shop/get
    pub fn shop_get(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/bizwifi/shop/get")
    }

    /// /bizwifi/shop/update
    pub fn shop_update(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/bizwifi/shop/update")
    }
}

/// mass_message 接口地址（对应 Java `WxMpApiUrl.mass_message`）。
pub mod mass_message {
    use super::*;

    /// /cgi-bin/media/uploadnews
    pub fn upload_news(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/media/uploadnews")
    }

    /// /cgi-bin/media/uploadvideo
    pub fn upload_video(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/media/uploadvideo")
    }

    /// /cgi-bin/message/mass/sendall
    pub fn send_all(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/message/mass/sendall")
    }

    /// /cgi-bin/message/mass/send
    pub fn send(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/message/mass/send")
    }

    /// /cgi-bin/message/mass/preview
    pub fn preview(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/message/mass/preview")
    }

    /// /cgi-bin/message/mass/delete
    pub fn delete(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/message/mass/delete")
    }

    /// /cgi-bin/message/mass/speed/get
    pub fn speed_get(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/message/mass/speed/get")
    }

    /// /cgi-bin/message/mass/speed/set
    pub fn speed_set(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/message/mass/speed/set")
    }

    /// /cgi-bin/message/mass/get
    pub fn get(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/message/mass/get")
    }
}

/// draft 接口地址（对应 Java `WxMpApiUrl.draft`）。
pub mod draft {
    use super::*;

    /// /cgi-bin/draft/add
    pub fn add(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/draft/add")
    }

    /// /cgi-bin/draft/update
    pub fn update(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/draft/update")
    }

    /// /cgi-bin/draft/get
    pub fn get(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/draft/get")
    }

    /// /cgi-bin/draft/delete
    pub fn delete(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/draft/delete")
    }

    /// /cgi-bin/draft/batchget
    pub fn list(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/draft/batchget")
    }

    /// /cgi-bin/draft/count
    pub fn count(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/draft/count")
    }
}

/// free_publish 接口地址（对应 Java `WxMpApiUrl.free_publish`）。
pub mod free_publish {
    use super::*;

    /// /cgi-bin/freepublish/submit
    pub fn submit(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/freepublish/submit")
    }

    /// /cgi-bin/freepublish/getarticle
    pub fn get_article(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/freepublish/getarticle")
    }

    /// /cgi-bin/freepublish/get
    pub fn get_push_status(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/freepublish/get")
    }

    /// /cgi-bin/freepublish/delete
    pub fn del_push(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/freepublish/delete")
    }

    /// /cgi-bin/freepublish/batchget
    pub fn batch_get(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/freepublish/batchget")
    }
}

/// device 接口地址（对应 Java `WxMpApiUrl.device`）。
pub mod device {
    use super::*;

    /// /device/transmsg
    pub fn trans_msg(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/device/transmsg")
    }

    /// /device/getqrcode
    pub fn get_qrcode(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/device/getqrcode")
    }

    /// /device/authorize_device
    pub fn authorize(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/device/authorize_device")
    }

    /// /device/bind
    pub fn bind(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/device/bind")
    }

    /// /device/compel_bind
    pub fn compel_bind(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/device/compel_bind")
    }

    /// /device/unbind
    pub fn unbind(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/device/unbind")
    }

    /// /device/compel_unbind
    pub fn compel_unbind(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/device/compel_unbind")
    }

    /// /device/get_openid
    pub fn get_openid(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/device/get_openid")
    }

    /// /device/get_bind_device
    pub fn get_bind_device(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/device/get_bind_device")
    }
}
/// material 接口地址（对应 Java `WxMpApiUrl.material`）。
pub mod material {
    use super::*;

    /// /cgi-bin/media/upload?type=%s
    pub fn media_upload(config: &dyn WxMpConfigStorage, arg: &str) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            &format!("/cgi-bin/media/upload?type={arg}"),
        )
    }

    /// /cgi-bin/media/get
    pub fn media_get(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/media/get")
    }

    /// /cgi-bin/media/uploadimg
    pub fn media_img_upload(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/media/uploadimg")
    }

    /// /cgi-bin/material/add_material?type=%s
    pub fn material_add(config: &dyn WxMpConfigStorage, arg: &str) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            &format!("/cgi-bin/material/add_material?type={arg}"),
        )
    }

    /// /cgi-bin/material/get_material
    pub fn material_get(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/material/get_material")
    }

    /// /cgi-bin/material/del_material
    pub fn material_del(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/material/del_material")
    }

    /// /cgi-bin/material/get_materialcount
    pub fn material_count(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/material/get_materialcount")
    }

    /// /cgi-bin/material/batchget_material
    pub fn material_batch_get(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/material/batchget_material")
    }
}

/// shake 接口地址（对应 Java `WxMpApiUrl.shake`）。
pub mod shake {
    use super::*;

    /// /shakearound/user/getshakeinfo
    pub fn get_shake_info(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/shakearound/user/getshakeinfo")
    }

    /// /shakearound/page/add
    pub fn page_add(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/shakearound/page/add")
    }

    /// /shakearound/device/bindpage
    pub fn device_bind_page(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/shakearound/device/bindpage")
    }

    /// /shakearound/relation/search
    pub fn relation_search(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/shakearound/relation/search")
    }
}

/// card 接口地址（对应 Java `WxMpApiUrl.card`）。
pub mod card {
    use super::*;

    /// /card/create
    pub fn card_create(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/card/create")
    }

    /// /card/get
    pub fn card_get(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/card/get")
    }

    /// /cgi-bin/ticket/getticket?type=wx_card
    pub fn card_get_ticket(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/cgi-bin/ticket/getticket?type=wx_card",
        )
    }

    /// /card/code/decrypt
    pub fn card_code_decrypt(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/card/code/decrypt")
    }

    /// /card/code/get
    pub fn card_code_get(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/card/code/get")
    }

    /// /card/code/consume
    pub fn card_code_consume(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/card/code/consume")
    }

    /// /card/code/mark
    pub fn card_code_mark(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/card/code/mark")
    }

    /// /card/testwhitelist/set
    pub fn card_test_whitelist(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/card/testwhitelist/set")
    }

    /// /card/qrcode/create
    pub fn card_qrcode_create(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/card/qrcode/create")
    }

    /// /card/landingpage/create
    pub fn card_landing_page_create(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/card/landingpage/create")
    }

    /// /card/delete
    pub fn card_delete(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/card/delete")
    }
}

/// member_card 接口地址（对应 Java `WxMpApiUrl.member_card`）。
pub mod member_card {
    use super::*;

    /// /card/create
    pub fn member_card_create(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/card/create")
    }

    /// /card/membercard/activate
    pub fn member_card_activate(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/card/membercard/activate")
    }

    /// /card/membercard/userinfo/get
    pub fn member_card_user_info_get(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/card/membercard/userinfo/get")
    }

    /// /card/membercard/updateuser
    pub fn member_card_update_user(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/card/membercard/updateuser")
    }

    /// /card/membercard/activateuserform/set
    pub fn member_card_activate_user_form(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/card/membercard/activateuserform/set")
    }

    /// /card/update
    pub fn member_card_update(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/card/update")
    }

    /// /card/membercard/activatetempinfo/get
    pub fn member_card_activate_temp_info(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/card/membercard/activatetempinfo/get")
    }

    /// /card/membercard/activateplugin/get
    pub fn member_card_activate_plugin(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/card/membercard/activateplugin/get")
    }
}

/// guide 接口地址（对应 Java `WxMpApiUrl.guide`）。
pub mod guide {
    use super::*;

    /// /cgi-bin/guide/addguideacct
    pub fn add_guide(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/guide/addguideacct")
    }

    /// /cgi-bin/guide/updateguideacct
    pub fn update_guide(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/guide/updateguideacct")
    }

    /// /cgi-bin/guide/getguideacct
    pub fn get_guide(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/guide/getguideacct")
    }

    /// /cgi-bin/guide/delguideacct
    pub fn del_guide(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/guide/delguideacct")
    }

    /// /cgi-bin/guide/getguideacctlist
    pub fn list_guide(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/guide/getguideacctlist")
    }

    /// /cgi-bin/guide/guidecreateqrcode
    pub fn create_qr_code(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/guide/guidecreateqrcode")
    }

    /// /cgi-bin/guide/getguidebuyerchatrecord
    pub fn get_guide_chat_record(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/cgi-bin/guide/getguidebuyerchatrecord",
        )
    }

    /// /cgi-bin/guide/setguideconfig
    pub fn set_guide_config(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/guide/setguideconfig")
    }

    /// /cgi-bin/guide/getguideconfig
    pub fn get_guide_config(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/guide/getguideconfig")
    }

    /// /cgi-bin/guide/setguideacctconfig
    pub fn set_guide_acct_config(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/guide/setguideacctconfig")
    }

    /// /cgi-bin/guide/getguideacctconfig
    pub fn get_guide_acct_config(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/guide/getguideacctconfig")
    }

    /// /cgi-bin/guide/newguidegroup
    pub fn new_guide_group(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/guide/newguidegroup")
    }

    /// /cgi-bin/guide/getguidegrouplist
    pub fn get_guide_group_list(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/guide/getguidegrouplist")
    }

    /// /cgi-bin/guide/addguidebuyerrelation
    pub fn add_guide_buyer_relation(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/guide/addguidebuyerrelation")
    }

    /// /cgi-bin/guide/delguidebuyerrelation
    pub fn del_guide_buyer_relation(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/guide/delguidebuyerrelation")
    }

    /// /cgi-bin/guide/getguidebuyerrelationlist
    pub fn get_guide_buyer_relation_list(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/cgi-bin/guide/getguidebuyerrelationlist",
        )
    }

    /// /cgi-bin/guide/rebindguideacctforbuyer
    pub fn rebind_guide_acct_for_buyer(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/cgi-bin/guide/rebindguideacctforbuyer",
        )
    }

    /// /cgi-bin/guide/updateguidebuyerrelation
    pub fn update_guide_buyer_relation(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/cgi-bin/guide/updateguidebuyerrelation",
        )
    }

    /// /cgi-bin/guide/newguidetagoption
    pub fn new_guide_tag_option(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/guide/newguidetagoption")
    }

    /// /cgi-bin/guide/delguidetagoption
    pub fn del_guide_tag_option(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/guide/delguidetagoption")
    }

    /// /cgi-bin/guide/addguidetagoption
    pub fn add_guide_tag_option(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/guide/addguidetagoption")
    }

    /// /cgi-bin/guide/getguidetagoption
    pub fn get_guide_tag_option(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/guide/getguidetagoption")
    }

    /// /cgi-bin/guide/addguidebuyertag
    pub fn add_guide_buyer_tag(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/guide/addguidebuyertag")
    }

    /// /cgi-bin/guide/getguidebuyertag
    pub fn get_guide_buyer_tag(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/guide/getguidebuyertag")
    }

    /// /cgi-bin/guide/queryguidebuyerbytag
    pub fn query_guide_buyer_by_tag(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/guide/queryguidebuyerbytag")
    }

    /// /cgi-bin/guide/setguidecardmaterial
    pub fn set_guide_card_material(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/guide/setguidecardmaterial")
    }

    /// /cgi-bin/guide/getguidecardmaterial
    pub fn get_guide_card_material(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/guide/getguidecardmaterial")
    }

    /// /cgi-bin/guide/delguidecardmaterial
    pub fn del_guide_card_material(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/guide/delguidecardmaterial")
    }

    /// /cgi-bin/guide/setguideimagematerial
    pub fn set_guide_image_material(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/guide/setguideimagematerial")
    }

    /// /cgi-bin/guide/getguideimagematerial
    pub fn get_guide_image_material(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/guide/getguideimagematerial")
    }

    /// /cgi-bin/guide/delguideimagematerial
    pub fn del_guide_image_material(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/guide/delguideimagematerial")
    }

    /// /cgi-bin/guide/setguidewordmaterial
    pub fn set_guide_word_material(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/guide/setguidewordmaterial")
    }

    /// /cgi-bin/guide/getguidewordmaterial
    pub fn get_guide_word_material(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/guide/getguidewordmaterial")
    }

    /// /cgi-bin/guide/delguidewordmaterial
    pub fn del_guide_word_material(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/guide/delguidewordmaterial")
    }

    /// /cgi-bin/guide/addguidemassendjob
    pub fn add_guide_massed_job(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/guide/addguidemassendjob")
    }

    /// /cgi-bin/guide/getguidemassendjoblist
    pub fn get_guide_massed_job_list(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/guide/getguidemassendjoblist")
    }

    /// /cgi-bin/guide/getguidemassendjob
    pub fn get_guide_massed_job(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/guide/getguidemassendjob")
    }

    /// /cgi-bin/guide/updateguidemassendjob
    pub fn update_guide_massed_job(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/guide/updateguidemassendjob")
    }

    /// /cgi-bin/guide/cancelguidemassendjob
    pub fn cancel_guide_massed_job(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/guide/cancelguidemassendjob")
    }
}

/// marketing 接口地址（对应 Java `WxMpApiUrl.marketing`）。
pub mod marketing {
    use super::*;

    /// /cgi-bin/marketing/user_action_sets/add
    pub fn add_user_action_sets(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/cgi-bin/marketing/user_action_sets/add",
        )
    }

    /// /cgi-bin/marketing/user_action_sets/get
    pub fn get_user_action_sets(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/cgi-bin/marketing/user_action_sets/get",
        )
    }

    /// /cgi-bin/marketing/user_actions/add
    pub fn add_user_action(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/marketing/user_actions/add")
    }

    /// /marketing/wechat_ad_leads/get
    pub fn get_ad_leads(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/marketing/wechat_ad_leads/get")
    }
}

/// subscribe_msg 接口地址（对应 Java `WxMpApiUrl.subscribe_msg`）。
pub mod subscribe_msg {
    use super::*;

    /// /cgi-bin/message/template/subscribe
    pub fn send_once(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/message/template/subscribe")
    }

    /// /cgi-bin/message/subscribe/bizsend
    pub fn send(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/message/subscribe/bizsend")
    }

    /// /wxaapi/newtmpl/getpubtemplatetitles
    pub fn get_pub_template_title_list(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/wxaapi/newtmpl/getpubtemplatetitles")
    }

    /// /wxaapi/newtmpl/getpubtemplatekeywords
    pub fn get_pub_template_key_words_by_id(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/wxaapi/newtmpl/getpubtemplatekeywords",
        )
    }

    /// /wxaapi/newtmpl/addtemplate
    pub fn template_add(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/wxaapi/newtmpl/addtemplate")
    }

    /// /wxaapi/newtmpl/gettemplate
    pub fn template_list(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/wxaapi/newtmpl/gettemplate")
    }

    /// /wxaapi/newtmpl/deltemplate
    pub fn template_del(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/wxaapi/newtmpl/deltemplate")
    }

    /// /wxaapi/newtmpl/getcategory
    pub fn get_category(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/wxaapi/newtmpl/getcategory")
    }
}

/// ai_open 接口地址（对应 Java `WxMpApiUrl.ai_open`）。
pub mod ai_open {
    use super::*;

    /// /cgi-bin/media/voice/addvoicetorecofortext?format=&voice_id=&lang=
    pub fn voice_upload(
        config: &dyn WxMpConfigStorage,
        format: &str,
        voice_id: &str,
        lang: &str,
    ) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            &format!(
                "/cgi-bin/media/voice/addvoicetorecofortext?format={format}&voice_id={voice_id}&lang={lang}"
            ),
        )
    }

    /// /cgi-bin/media/voice/queryrecoresultfortext
    pub fn voice_query_result(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/cgi-bin/media/voice/queryrecoresultfortext",
        )
    }

    /// /cgi-bin/media/voice/translatecontent?lfrom=&lto=
    pub fn translate(config: &dyn WxMpConfigStorage, lfrom: &str, lto: &str) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            &format!("/cgi-bin/media/voice/translatecontent?lfrom={lfrom}&lto={lto}"),
        )
    }
}

/// ocr 接口地址（对应 Java `WxMpApiUrl.ocr`）。
pub mod ocr {
    use super::*;

    /// /cgi-bin/ocr/idcard
    pub fn id_card(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/ocr/idcard")
    }

    /// /cgi-bin/ocr/bankcard
    pub fn bank_card(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/ocr/bankcard")
    }

    /// /cgi-bin/ocr/driving
    pub fn driving(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/ocr/driving")
    }

    /// /cgi-bin/ocr/drivinglicense
    pub fn driving_license(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/ocr/drivinglicense")
    }

    /// /cgi-bin/ocr/bizlicense
    pub fn biz_license(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/ocr/bizlicense")
    }

    /// /cgi-bin/ocr/comm
    pub fn comm(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/ocr/comm")
    }
}

/// img_proc 接口地址（对应 Java `WxMpApiUrl.img_proc`）。
pub mod img_proc {
    use super::*;

    /// /cgi-bin/imgproc/qrcode
    pub fn qr_code(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/imgproc/qrcode")
    }

    /// /cgi-bin/imgproc/superresolution
    pub fn super_resolution(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/imgproc/superresolution")
    }

    /// /cgi-bin/imgproc/aicrop
    pub fn ai_crop(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/imgproc/aicrop")
    }
}

/// reimburse_invoice 接口地址（对应 Java `WxMpApiUrl.reimburse_invoice`）。
pub mod reimburse_invoice {
    use super::*;

    /// /cgi-bin/invoice/getinvoicedetail
    pub fn get_invoice_info(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/invoice/getinvoicedetail")
    }

    /// /cgi-bin/invoice/getinvoicebatch
    pub fn get_invoice_batch(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/invoice/getinvoicebatch")
    }

    /// /cgi-bin/invoice/updateinvoicestatus
    pub fn update_invoice_status(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/invoice/updateinvoicestatus")
    }

    /// /cgi-bin/invoice/updatestatusbatch
    pub fn update_status_batch(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/invoice/updatestatusbatch")
    }
}

/// merchant_invoice 接口地址（对应 Java `WxMpApiUrl.merchant_invoice`）。
pub mod merchant_invoice {
    use super::*;

    /// /card/invoice/getauthurl
    pub fn get_auth_url(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/card/invoice/getauthurl")
    }

    /// /card/invoice/getauthdata
    pub fn get_auth_data(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/card/invoice/getauthdata")
    }

    /// /card/invoice/rejectinsert
    pub fn reject_insert(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/card/invoice/rejectinsert")
    }

    /// /card/invoice/makeoutinvoice
    pub fn make_out_invoice(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/card/invoice/makeoutinvoice")
    }

    /// /card/invoice/clearoutinvoice
    pub fn clear_out_invoice(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/card/invoice/clearoutinvoice")
    }

    /// /card/invoice/queryinvoceinfo
    pub fn query_invoice_info(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/card/invoice/queryinvoceinfo")
    }

    /// /card/invoice/setbizattr?action=set_contact
    pub fn set_contact(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/card/invoice/setbizattr?action=set_contact",
        )
    }

    /// /card/invoice/setbizattr?action=query_contact
    pub fn get_contact(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/card/invoice/setbizattr?action=query_contact",
        )
    }

    /// /card/invoice/setbizattr?action=set_auth_field
    pub fn set_auth_page(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/card/invoice/setbizattr?action=set_auth_field",
        )
    }

    /// /card/invoice/setbizattr?action=query_auth_field
    pub fn get_auth_page(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/card/invoice/setbizattr?action=query_auth_field",
        )
    }

    /// /card/invoice/setbizattr?action=set_pay_mch
    pub fn set_platform(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/card/invoice/setbizattr?action=set_pay_mch",
        )
    }

    /// /card/invoice/setbizattr?action=query_pay_mch
    pub fn get_platform(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/card/invoice/setbizattr?action=query_pay_mch",
        )
    }
}
/// OAuth2 网页授权接口地址（对应 Java `WxMpApiUrl.OAuth2`）。
pub mod oauth2 {
    use super::*;

    /// 通过 code 换取 access token。
    pub fn sns_oauth2_access_token(
        config: &dyn WxMpConfigStorage,
        app_id: &str,
        secret: &str,
        code: &str,
    ) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            &format!(
                "/sns/oauth2/access_token?appid={app_id}&secret={secret}&code={code}&grant_type=authorization_code"
            ),
        )
    }

    /// 刷新 access token。
    pub fn sns_oauth2_refresh_token(
        config: &dyn WxMpConfigStorage,
        app_id: &str,
        refresh_token: &str,
    ) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            &format!(
                "/sns/oauth2/refresh_token?appid={app_id}&grant_type=refresh_token&refresh_token={refresh_token}"
            ),
        )
    }

    /// 拉取用户信息。
    pub fn sns_userinfo(
        config: &dyn WxMpConfigStorage,
        access_token: &str,
        openid: &str,
        lang: &str,
    ) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            &format!("/sns/userinfo?access_token={access_token}&openid={openid}&lang={lang}"),
        )
    }

    /// 校验 access token 有效性。
    pub fn sns_auth(config: &dyn WxMpConfigStorage, access_token: &str, openid: &str) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            &format!("/sns/auth?access_token={access_token}&openid={openid}"),
        )
    }
}
