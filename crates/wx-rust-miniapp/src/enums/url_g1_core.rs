//! 小程序核心服务组接口地址（G1 组：统计/代码/物流/客服/素材/设置）。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.constant.WxMaApiUrlConstants` 的
//! Analysis/Code/Express/Setting/Media 子域，以及
//! `WxMaKefuServiceImpl` 内的客服管理接口常量（未收敛进常量类，原文内联）。
//! 用户/安全/消息域地址已在 `url_business` 中存在，直接复用，不重复定义。
//! 函数风格与 `url_core`/`url_business` 一致：config 参数 + api_host 前缀
//! （自定义域名替换由执行引擎在 token 注入时统一处理）。

use crate::config::{DEFAULT_API_HOST_URL, WxMaConfig};

/// 生成完整接口地址：域名前缀 + 路径。
fn url(_config: &dyn WxMaConfig, host: &str, path: &str) -> String {
    format!("{host}{path}")
}

/// 数据分析接口地址（对应 Java `WxMaApiUrlConstants.Analysis`）。
pub mod analysis {
    use super::*;

    /// 概况趋势（对应 Java `Analysis.GET_DAILY_SUMMARY_TREND_URL`）。
    pub fn get_daily_summary_trend_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/datacube/getweanalysisappiddailysummarytrend",
        )
    }

    /// 日访问趋势（对应 Java `Analysis.GET_DAILY_VISIT_TREND_URL`）。
    pub fn get_daily_visit_trend_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/datacube/getweanalysisappiddailyvisittrend",
        )
    }

    /// 周访问趋势（对应 Java `Analysis.GET_WEEKLY_VISIT_TREND_URL`）。
    pub fn get_weekly_visit_trend_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/datacube/getweanalysisappidweeklyvisittrend",
        )
    }

    /// 月访问趋势（对应 Java `Analysis.GET_MONTHLY_VISIT_TREND_URL`）。
    pub fn get_monthly_visit_trend_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/datacube/getweanalysisappidmonthlyvisittrend",
        )
    }

    /// 访问分布（对应 Java `Analysis.GET_VISIT_DISTRIBUTION_URL`）。
    pub fn get_visit_distribution_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/datacube/getweanalysisappidvisitdistribution",
        )
    }

    /// 日留存数据（对应 Java `Analysis.GET_DAILY_RETAIN_INFO_URL`）。
    pub fn get_daily_retain_info_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/datacube/getweanalysisappiddailyretaininfo",
        )
    }

    /// 周留存数据（对应 Java `Analysis.GET_WEEKLY_RETAIN_INFO_URL`）。
    pub fn get_weekly_retain_info_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/datacube/getweanalysisappidweeklyretaininfo",
        )
    }

    /// 月留存数据（对应 Java `Analysis.GET_MONTHLY_RETAIN_INFO_URL`）。
    pub fn get_monthly_retain_info_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/datacube/getweanalysisappidmonthlyretaininfo",
        )
    }

    /// 访问页面数据（对应 Java `Analysis.GET_VISIT_PAGE_URL`）。
    pub fn get_visit_page_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/datacube/getweanalysisappidvisitpage")
    }

    /// 用户画像分布数据（对应 Java `Analysis.GET_USER_PORTRAIT_URL`）。
    pub fn get_user_portrait_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/datacube/getweanalysisappiduserportrait",
        )
    }
}

/// 小程序代码管理接口地址（对应 Java `WxMaApiUrlConstants.Code`）。
pub mod code {
    use super::*;

    /// 为授权的小程序帐号上传小程序代码（对应 Java `Code.COMMIT_URL`）。
    pub fn commit_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/wxa/commit")
    }

    /// 获取体验小程序的体验二维码（对应 Java `Code.GET_QRCODE_URL`）。
    pub fn get_qrcode_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/wxa/get_qrcode")
    }

    /// 获取授权小程序帐号的可选类目（对应 Java `Code.GET_CATEGORY_URL`）。
    pub fn get_category_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/wxa/get_category")
    }

    /// 获取小程序的第三方提交代码的页面配置（对应 Java `Code.GET_PAGE_URL`）。
    pub fn get_page_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/wxa/get_page")
    }

    /// 提交代码审核（对应 Java `Code.SUBMIT_AUDIT_URL`）。
    pub fn submit_audit_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/wxa/submit_audit")
    }

    /// 查询某个指定版本的审核状态（对应 Java `Code.GET_AUDIT_STATUS_URL`）。
    pub fn get_audit_status_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/wxa/get_auditstatus")
    }

    /// 查询最新一次提交的审核状态（对应 Java `Code.GET_LATEST_AUDIT_STATUS_URL`）。
    pub fn get_latest_audit_status_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/wxa/get_latest_auditstatus")
    }

    /// 发布已通过审核的小程序（对应 Java `Code.RELEASE_URL`）。
    pub fn release_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/wxa/release")
    }

    /// 修改小程序线上代码的可见状态（对应 Java `Code.CHANGE_VISIT_STATUS_URL`）。
    pub fn change_visit_status_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/wxa/change_visitstatus")
    }

    /// 小程序版本回退（对应 Java `Code.REVERT_CODE_RELEASE_URL`）。
    pub fn revert_code_release_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/wxa/revertcoderelease")
    }

    /// 查询当前设置的最低基础库版本及各版本用户占比（对应 Java `Code.GET_SUPPORT_VERSION_URL`）。
    pub fn get_support_version_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/cgi-bin/wxopen/getweappsupportversion",
        )
    }

    /// 设置最低基础库版本（对应 Java `Code.SET_SUPPORT_VERSION_URL`）。
    pub fn set_support_version_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/cgi-bin/wxopen/setweappsupportversion",
        )
    }

    /// 小程序审核撤回（对应 Java `Code.UNDO_CODE_AUDIT_URL`）。
    pub fn undo_code_audit_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/wxa/undocodeaudit")
    }

    /// 查询小程序版本信息（对应 Java `Code.GET_VERSION_INFO_URL`）。
    pub fn get_version_info_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/wxa/getversioninfo")
    }
}

/// 小程序物流助手接口地址（对应 Java `WxMaApiUrlConstants.Express`）。
pub mod express {
    use super::*;

    /// 获取支持的快递公司列表（对应 Java `Express.ALL_DELIVERY_URL`）。
    pub fn all_delivery_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/cgi-bin/express/business/delivery/getall",
        )
    }

    /// 获取所有绑定的物流账号（对应 Java `Express.ALL_ACCOUNT_URL`）。
    pub fn all_account_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/cgi-bin/express/business/account/getall",
        )
    }

    /// 绑定、解绑物流账号（对应 Java `Express.BIND_ACCOUNT_URL`）。
    pub fn bind_account_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/cgi-bin/express/business/account/bind",
        )
    }

    /// 获取电子面单余额（对应 Java `Express.GET_QUOTA_URL`）。
    pub fn get_quota_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/express/business/quota/get")
    }

    /// 配置面单打印员（对应 Java `Express.UPDATE_PRINTER_URL`）。
    pub fn update_printer_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/cgi-bin/express/business/printer/update",
        )
    }

    /// 获取打印员（对应 Java `Express.GET_PRINTER_URL`）。
    pub fn get_printer_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/cgi-bin/express/business/printer/getall",
        )
    }

    /// 生成运单（对应 Java `Express.ADD_ORDER_URL`）。
    pub fn add_order_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/express/business/order/add")
    }

    /// 批量获取运单数据（对应 Java `Express.BATCH_GET_ORDER_URL`）。
    pub fn batch_get_order_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/cgi-bin/express/business/order/batchget",
        )
    }

    /// 取消运单（对应 Java `Express.CANCEL_ORDER_URL`）。
    pub fn cancel_order_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/cgi-bin/express/business/order/cancel",
        )
    }

    /// 获取运单数据（对应 Java `Express.GET_ORDER_URL`）。
    pub fn get_order_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/express/business/order/get")
    }

    /// 查询运单轨迹（对应 Java `Express.GET_PATH_URL`）。
    pub fn get_path_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/express/business/path/get")
    }

    /// 模拟快递公司更新订单状态（对应 Java `Express.TEST_UPDATE_ORDER_URL`）。
    pub fn test_update_order_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/cgi-bin/express/business/test_update_order",
        )
    }
}

/// 小程序客服管理接口地址。
///
/// 对应 Java `WxMaKefuServiceImpl` 内联常量（`KFLIST_GET_URL` 等，
/// 未收敛进 `WxMaApiUrlConstants`）。
pub mod kefu {
    use super::*;

    /// 获取客服基本信息（对应 Java `KFLIST_GET_URL`）。
    pub fn get_kf_list_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/customservice/getkflist")
    }

    /// 添加客服账号（对应 Java `KFACCOUNT_ADD_URL`）。
    pub fn kf_account_add_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/customservice/kfaccount/add")
    }

    /// 修改客服账号（对应 Java `KFACCOUNT_UPDATE_URL`）。
    pub fn kf_account_update_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/customservice/kfaccount/update")
    }

    /// 删除客服账号（对应 Java `KFACCOUNT_DEL_URL`，`%s` 为 kf_account）。
    pub fn kf_account_del_url(config: &dyn WxMaConfig, kf_account: &str) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            &format!("/customservice/kfaccount/del?kf_account={kf_account}"),
        )
    }

    /// 创建会话（对应 Java `KFSESSION_CREATE_URL`）。
    pub fn kf_session_create_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/customservice/kfsession/create")
    }

    /// 关闭会话（对应 Java `KFSESSION_CLOSE_URL`）。
    pub fn kf_session_close_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/customservice/kfsession/close")
    }

    /// 获取客户的会话状态（对应 Java `KFSESSION_GET_URL`，`%s` 为 openid）。
    pub fn kf_session_get_url(config: &dyn WxMaConfig, openid: &str) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            &format!("/customservice/kfsession/getsession?openid={openid}"),
        )
    }

    /// 获取客服的会话列表（对应 Java `KFSESSION_LIST_URL`，`%s` 为 kf_account）。
    pub fn kf_session_list_url(config: &dyn WxMaConfig, kf_account: &str) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            &format!("/customservice/kfsession/getsessionlist?kf_account={kf_account}"),
        )
    }
}

/// 临时素材接口地址（对应 Java `WxMaApiUrlConstants.Media`）。
pub mod media {
    use super::*;

    /// 新增临时素材（对应 Java `Media.MEDIA_UPLOAD_URL`，`%s` 为媒体类型）。
    pub fn media_upload_url(config: &dyn WxMaConfig, media_type: &str) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            &format!("/cgi-bin/media/upload?type={media_type}"),
        )
    }

    /// 获取临时素材（对应 Java `Media.MEDIA_GET_URL`）。
    pub fn media_get_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/media/get")
    }
}

/// 小程序设置接口地址（对应 Java `WxMaApiUrlConstants.Setting`）。
pub mod setting {
    use super::*;

    /// 操作服务器域名（对应 Java `Setting.MODIFY_DOMAIN_URL`）。
    pub fn modify_domain_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/wxa/modify_domain")
    }

    /// 设置小程序业务域名（对应 Java `Setting.SET_WEB_VIEW_DOMAIN_URL`）。
    pub fn set_web_view_domain_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/wxa/setwebviewdomain")
    }

    /// 绑定微信用户为小程序体验者（对应 Java `Setting.BIND_TESTER_URL`）。
    pub fn bind_tester_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/wxa/bind_tester")
    }

    /// 解除绑定小程序的体验者（对应 Java `Setting.UNBIND_TESTER_URL`）。
    pub fn unbind_tester_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/wxa/unbind_tester")
    }
}

/// 默认 API 域名字面量（与 `url_core::API_HOST` 一致，供本模块内部使用）。
#[allow(unused)]
const API_HOST: &str = DEFAULT_API_HOST_URL;
