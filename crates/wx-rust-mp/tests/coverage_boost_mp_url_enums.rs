//! 公众号 API 接口地址（WxMpApiUrl）覆盖率提升测试（纯离线，无网络依赖）。
//!
//! 覆盖 `enums/wx_mp_api_url.rs` 全部 `pub fn`：每个接口地址函数都以
//! 默认域名（api_host/open_host）断言完整 URL（域名前缀 + 路径字面量），
//! 含带查询参数的格式化地址（kefu 会话/material 类型/aiOpen 语音/
//! OAuth2 四接口）与 API_HOST/OPEN_HOST 域名常量。
//!
//! 本文件由 `scripts/gen` 风格脚本从源码字面量生成断言（对应 Java
//! `WxMpApiUrl` 各分组枚举常量），仅作离线线格式校验。

use wx_rust_mp::config::WxMpConfigStorage;
use wx_rust_mp::config::r#impl::WxMpDefaultConfig;
use wx_rust_mp::enums::wx_mp_api_url::*;

/// 默认域名配置（api/open 均为官方默认值）。
fn default_config() -> WxMpDefaultConfig {
    WxMpDefaultConfig::new("wxappid", "secret")
}

/// 对应 Java: WxMpApiUrl.API_DEFAULT_HOST_URL / OPEN_DEFAULT_HOST_URL 域名常量。
#[test]
fn url_host_constants() {
    let cfg = default_config();
    let h = cfg.host_config();
    assert_eq!(h.api_host, API_HOST);
    assert_eq!(h.open_host, OPEN_HOST);
    assert_eq!(API_HOST, "https://api.weixin.qq.com");
    assert_eq!(OPEN_HOST, "https://open.weixin.qq.com");
    // host_config 返回的是默认域名快照
    assert_eq!(h.api_host, wx_rust_mp::config::API_DEFAULT_HOST_URL);
    assert_eq!(h.open_host, wx_rust_mp::config::OPEN_DEFAULT_HOST_URL);
}

/// 对应 Java: WxMpApiUrl.Other 分组（other 模块全部地址）。
#[test]
fn other_urls() {
    let cfg = default_config();
    assert_eq!(
        other::get_access_token_url(&cfg),
        "https://api.weixin.qq.com/cgi-bin/token?grant_type=client_credential&appid=wxappid&secret=secret"
    );
    assert_eq!(
        other::get_stable_access_token_url(&cfg),
        "https://api.weixin.qq.com/cgi-bin/stable_token"
    );
    assert_eq!(
        other::get_ticket_url(&cfg),
        "https://api.weixin.qq.com/cgi-bin/ticket/getticket?type="
    );
    assert_eq!(
        other::shorturl_api_url(&cfg),
        "https://api.weixin.qq.com/cgi-bin/shorturl"
    );
    assert_eq!(
        other::semantic_semproxy_search_url(&cfg),
        "https://api.weixin.qq.com/semantic/semproxy/search"
    );
    assert_eq!(
        other::get_callback_ip_url(&cfg),
        "https://api.weixin.qq.com/cgi-bin/getcallbackip"
    );
    assert_eq!(
        other::netcheck_url(&cfg),
        "https://api.weixin.qq.com/cgi-bin/callback/check"
    );
    assert_eq!(
        other::qrconnect_url(&cfg),
        "https://open.weixin.qq.com/connect/qrconnect"
    );
    assert_eq!(
        other::get_current_autoreply_info_url(&cfg),
        "https://api.weixin.qq.com/cgi-bin/get_current_autoreply_info"
    );
    assert_eq!(
        other::clear_quota_url(&cfg),
        "https://api.weixin.qq.com/cgi-bin/clear_quota"
    );
    assert_eq!(
        other::gen_shorten_url(&cfg),
        "https://api.weixin.qq.com/cgi-bin/shorten/gen"
    );
    assert_eq!(
        other::fetch_shorten_url(&cfg),
        "https://api.weixin.qq.com/cgi-bin/shorten/fetch"
    );
}

/// 对应 Java: WxMpApiUrl.Menu 分组（menu 模块全部地址）。
#[test]
fn menu_urls() {
    let cfg = default_config();
    assert_eq!(
        menu::menu_create(&cfg),
        "https://api.weixin.qq.com/cgi-bin/menu/create"
    );
    assert_eq!(
        menu::menu_get(&cfg),
        "https://api.weixin.qq.com/cgi-bin/menu/get"
    );
    assert_eq!(
        menu::menu_delete(&cfg),
        "https://api.weixin.qq.com/cgi-bin/menu/delete"
    );
    assert_eq!(
        menu::get_self_menu_info(&cfg),
        "https://api.weixin.qq.com/cgi-bin/get_current_selfmenu_info"
    );
}

/// 对应 Java: WxMpApiUrl.TemplateMsg 分组（template_msg 模块全部地址）。
#[test]
fn template_msg_urls() {
    let cfg = default_config();
    assert_eq!(
        template_msg::message_template_send(&cfg),
        "https://api.weixin.qq.com/cgi-bin/message/template/send"
    );
}

/// 对应 Java: WxMpApiUrl.Qrcode 分组（qrcode 模块全部地址）。
#[test]
fn qrcode_urls() {
    let cfg = default_config();
    assert_eq!(
        qrcode::qrcode_create(&cfg),
        "https://api.weixin.qq.com/cgi-bin/qrcode/create"
    );
}

/// 对应 Java: WxMpApiUrl.Kefu 分组（kefu 模块全部地址）。
#[test]
fn kefu_urls() {
    let cfg = default_config();
    assert_eq!(
        kefu::message_custom_send(&cfg),
        "https://api.weixin.qq.com/cgi-bin/message/custom/send"
    );
    assert_eq!(
        kefu::kfaccount_add(&cfg),
        "https://api.weixin.qq.com/customservice/kfaccount/add"
    );
    assert_eq!(
        kefu::kfaccount_update(&cfg),
        "https://api.weixin.qq.com/customservice/kfaccount/update"
    );
    assert_eq!(
        kefu::kfaccount_invite_worker(&cfg),
        "https://api.weixin.qq.com/customservice/kfaccount/inviteworker"
    );
    assert_eq!(
        kefu::kfaccount_del(&cfg, "kf001"),
        "https://api.weixin.qq.com/customservice/kfaccount/del?kf_account=kf001"
    );
    assert_eq!(
        kefu::kfsession_create(&cfg),
        "https://api.weixin.qq.com/customservice/kfsession/create"
    );
    assert_eq!(
        kefu::kfsession_close(&cfg),
        "https://api.weixin.qq.com/customservice/kfsession/close"
    );
    assert_eq!(
        kefu::kfsession_get(&cfg, "oX1"),
        "https://api.weixin.qq.com/customservice/kfsession/getsession?openid=oX1"
    );
    assert_eq!(
        kefu::kfsession_list(&cfg, "kf001"),
        "https://api.weixin.qq.com/customservice/kfsession/getsessionlist?kf_account=kf001"
    );
    assert_eq!(
        kefu::kfsession_get_wait_case(&cfg),
        "https://api.weixin.qq.com/customservice/kfsession/getwaitcase"
    );
    assert_eq!(
        kefu::getkflist(&cfg),
        "https://api.weixin.qq.com/cgi-bin/customservice/getkflist"
    );
    assert_eq!(
        kefu::getonlinekflist(&cfg),
        "https://api.weixin.qq.com/cgi-bin/customservice/getonlinekflist"
    );
    assert_eq!(
        kefu::getmsglist(&cfg),
        "https://api.weixin.qq.com/cgi-bin/customservice/msgrecord/getmsglist"
    );
}

/// 对应 Java: WxMpApiUrl.User 分组（user 模块全部地址）。
#[test]
fn user_urls() {
    let cfg = default_config();
    assert_eq!(
        user::user_info(&cfg),
        "https://api.weixin.qq.com/cgi-bin/user/info"
    );
}

/// 对应 Java: WxMpApiUrl.UserTag 分组（tags 模块全部地址）。
#[test]
fn tags_urls() {
    let cfg = default_config();
    assert_eq!(
        tags::create(&cfg),
        "https://api.weixin.qq.com/cgi-bin/tags/create"
    );
    assert_eq!(
        tags::get(&cfg),
        "https://api.weixin.qq.com/cgi-bin/tags/get"
    );
    assert_eq!(
        tags::update(&cfg),
        "https://api.weixin.qq.com/cgi-bin/tags/update"
    );
    assert_eq!(
        tags::delete(&cfg),
        "https://api.weixin.qq.com/cgi-bin/tags/delete"
    );
    assert_eq!(
        tags::tag_user_get(&cfg),
        "https://api.weixin.qq.com/cgi-bin/user/tag/get"
    );
    assert_eq!(
        tags::batch_tagging(&cfg),
        "https://api.weixin.qq.com/cgi-bin/tags/members/batchtagging"
    );
    assert_eq!(
        tags::batch_untagging(&cfg),
        "https://api.weixin.qq.com/cgi-bin/tags/members/batchuntagging"
    );
    assert_eq!(
        tags::get_id_list(&cfg),
        "https://api.weixin.qq.com/cgi-bin/tags/getidlist"
    );
}

/// 对应 Java: WxMpApiUrl.UserBlacklist 分组（blacklist 模块全部地址）。
#[test]
fn blacklist_urls() {
    let cfg = default_config();
    assert_eq!(
        blacklist::get_blacklist(&cfg),
        "https://api.weixin.qq.com/cgi-bin/tags/members/getblacklist"
    );
    assert_eq!(
        blacklist::batch_blacklist(&cfg),
        "https://api.weixin.qq.com/cgi-bin/tags/members/batchblacklist"
    );
    assert_eq!(
        blacklist::batch_unblacklist(&cfg),
        "https://api.weixin.qq.com/cgi-bin/tags/members/batchunblacklist"
    );
}

/// 对应 Java: WxMpApiUrl.Store 分组（store 模块全部地址）。
#[test]
fn store_urls() {
    let cfg = default_config();
    assert_eq!(
        store::poi_add(&cfg),
        "https://api.weixin.qq.com/cgi-bin/poi/addpoi"
    );
    assert_eq!(
        store::poi_get(&cfg),
        "https://api.weixin.qq.com/cgi-bin/poi/getpoi"
    );
    assert_eq!(
        store::poi_del(&cfg),
        "https://api.weixin.qq.com/cgi-bin/poi/delpoi"
    );
    assert_eq!(
        store::poi_list(&cfg),
        "https://api.weixin.qq.com/cgi-bin/poi/getpoilist"
    );
    assert_eq!(
        store::poi_update(&cfg),
        "https://api.weixin.qq.com/cgi-bin/poi/updatepoi"
    );
    assert_eq!(
        store::wx_category(&cfg),
        "https://api.weixin.qq.com/cgi-bin/poi/getwxcategory"
    );
}

/// 对应 Java: WxMpApiUrl.Comment 分组（comment 模块全部地址）。
#[test]
fn comment_urls() {
    let cfg = default_config();
    assert_eq!(
        comment::open(&cfg),
        "https://api.weixin.qq.com/cgi-bin/comment/open"
    );
    assert_eq!(
        comment::close(&cfg),
        "https://api.weixin.qq.com/cgi-bin/comment/close"
    );
    assert_eq!(
        comment::list(&cfg),
        "https://api.weixin.qq.com/cgi-bin/comment/list"
    );
    assert_eq!(
        comment::mark_elect(&cfg),
        "https://api.weixin.qq.com/cgi-bin/comment/markelect"
    );
    assert_eq!(
        comment::unmark_elect(&cfg),
        "https://api.weixin.qq.com/cgi-bin/comment/unmarkelect"
    );
    assert_eq!(
        comment::delete(&cfg),
        "https://api.weixin.qq.com/cgi-bin/comment/delete"
    );
    assert_eq!(
        comment::reply_add(&cfg),
        "https://api.weixin.qq.com/cgi-bin/comment/reply/add"
    );
    assert_eq!(
        comment::reply_delete(&cfg),
        "https://api.weixin.qq.com/cgi-bin/comment/reply/delete"
    );
}

/// 对应 Java: WxMpApiUrl.DataCube 分组（datacube 模块全部地址）。
#[test]
fn datacube_urls() {
    let cfg = default_config();
    assert_eq!(
        datacube::get_user_summary(&cfg),
        "https://api.weixin.qq.com/datacube/getusersummary"
    );
    assert_eq!(
        datacube::get_user_cumulate(&cfg),
        "https://api.weixin.qq.com/datacube/getusercumulate"
    );
    assert_eq!(
        datacube::get_article_summary(&cfg),
        "https://api.weixin.qq.com/datacube/getarticlesummary"
    );
    assert_eq!(
        datacube::get_article_total(&cfg),
        "https://api.weixin.qq.com/datacube/getarticletotal"
    );
}

/// 对应 Java: WxMpApiUrl.Wifi 分组（wifi 模块全部地址）。
#[test]
fn wifi_urls() {
    let cfg = default_config();
    assert_eq!(
        wifi::shop_list(&cfg),
        "https://api.weixin.qq.com/bizwifi/shop/list"
    );
    assert_eq!(
        wifi::shop_get(&cfg),
        "https://api.weixin.qq.com/bizwifi/shop/get"
    );
    assert_eq!(
        wifi::shop_update(&cfg),
        "https://api.weixin.qq.com/bizwifi/shop/update"
    );
}

/// 对应 Java: WxMpApiUrl.MassMessage 分组（mass_message 模块全部地址）。
#[test]
fn mass_message_urls() {
    let cfg = default_config();
    assert_eq!(
        mass_message::upload_news(&cfg),
        "https://api.weixin.qq.com/cgi-bin/media/uploadnews"
    );
    assert_eq!(
        mass_message::upload_video(&cfg),
        "https://api.weixin.qq.com/cgi-bin/media/uploadvideo"
    );
    assert_eq!(
        mass_message::send_all(&cfg),
        "https://api.weixin.qq.com/cgi-bin/message/mass/sendall"
    );
    assert_eq!(
        mass_message::send(&cfg),
        "https://api.weixin.qq.com/cgi-bin/message/mass/send"
    );
    assert_eq!(
        mass_message::preview(&cfg),
        "https://api.weixin.qq.com/cgi-bin/message/mass/preview"
    );
    assert_eq!(
        mass_message::delete(&cfg),
        "https://api.weixin.qq.com/cgi-bin/message/mass/delete"
    );
    assert_eq!(
        mass_message::speed_get(&cfg),
        "https://api.weixin.qq.com/cgi-bin/message/mass/speed/get"
    );
    assert_eq!(
        mass_message::speed_set(&cfg),
        "https://api.weixin.qq.com/cgi-bin/message/mass/speed/set"
    );
    assert_eq!(
        mass_message::get(&cfg),
        "https://api.weixin.qq.com/cgi-bin/message/mass/get"
    );
}

/// 对应 Java: WxMpApiUrl.Draft 分组（draft 模块全部地址）。
#[test]
fn draft_urls() {
    let cfg = default_config();
    assert_eq!(
        draft::add(&cfg),
        "https://api.weixin.qq.com/cgi-bin/draft/add"
    );
    assert_eq!(
        draft::update(&cfg),
        "https://api.weixin.qq.com/cgi-bin/draft/update"
    );
    assert_eq!(
        draft::get(&cfg),
        "https://api.weixin.qq.com/cgi-bin/draft/get"
    );
    assert_eq!(
        draft::delete(&cfg),
        "https://api.weixin.qq.com/cgi-bin/draft/delete"
    );
    assert_eq!(
        draft::list(&cfg),
        "https://api.weixin.qq.com/cgi-bin/draft/batchget"
    );
    assert_eq!(
        draft::count(&cfg),
        "https://api.weixin.qq.com/cgi-bin/draft/count"
    );
}

/// 对应 Java: WxMpApiUrl.FreePublish 分组（free_publish 模块全部地址）。
#[test]
fn free_publish_urls() {
    let cfg = default_config();
    assert_eq!(
        free_publish::submit(&cfg),
        "https://api.weixin.qq.com/cgi-bin/freepublish/submit"
    );
    assert_eq!(
        free_publish::get_article(&cfg),
        "https://api.weixin.qq.com/cgi-bin/freepublish/getarticle"
    );
    assert_eq!(
        free_publish::get_push_status(&cfg),
        "https://api.weixin.qq.com/cgi-bin/freepublish/get"
    );
    assert_eq!(
        free_publish::del_push(&cfg),
        "https://api.weixin.qq.com/cgi-bin/freepublish/delete"
    );
    assert_eq!(
        free_publish::batch_get(&cfg),
        "https://api.weixin.qq.com/cgi-bin/freepublish/batchget"
    );
}

/// 对应 Java: WxMpApiUrl.Device 分组（device 模块全部地址）。
#[test]
fn device_urls() {
    let cfg = default_config();
    assert_eq!(
        device::trans_msg(&cfg),
        "https://api.weixin.qq.com/device/transmsg"
    );
    assert_eq!(
        device::get_qrcode(&cfg),
        "https://api.weixin.qq.com/device/getqrcode"
    );
    assert_eq!(
        device::authorize(&cfg),
        "https://api.weixin.qq.com/device/authorize_device"
    );
    assert_eq!(device::bind(&cfg), "https://api.weixin.qq.com/device/bind");
    assert_eq!(
        device::compel_bind(&cfg),
        "https://api.weixin.qq.com/device/compel_bind"
    );
    assert_eq!(
        device::unbind(&cfg),
        "https://api.weixin.qq.com/device/unbind"
    );
    assert_eq!(
        device::compel_unbind(&cfg),
        "https://api.weixin.qq.com/device/compel_unbind"
    );
    assert_eq!(
        device::get_openid(&cfg),
        "https://api.weixin.qq.com/device/get_openid"
    );
    assert_eq!(
        device::get_bind_device(&cfg),
        "https://api.weixin.qq.com/device/get_bind_device"
    );
}

/// 对应 Java: WxMpApiUrl.Material 分组（material 模块全部地址）。
#[test]
fn material_urls() {
    let cfg = default_config();
    assert_eq!(
        material::media_upload(&cfg, "image"),
        "https://api.weixin.qq.com/cgi-bin/media/upload?type=image"
    );
    assert_eq!(
        material::media_get(&cfg),
        "https://api.weixin.qq.com/cgi-bin/media/get"
    );
    assert_eq!(
        material::media_img_upload(&cfg),
        "https://api.weixin.qq.com/cgi-bin/media/uploadimg"
    );
    assert_eq!(
        material::material_add(&cfg, "image"),
        "https://api.weixin.qq.com/cgi-bin/material/add_material?type=image"
    );
    assert_eq!(
        material::material_get(&cfg),
        "https://api.weixin.qq.com/cgi-bin/material/get_material"
    );
    assert_eq!(
        material::material_del(&cfg),
        "https://api.weixin.qq.com/cgi-bin/material/del_material"
    );
    assert_eq!(
        material::material_count(&cfg),
        "https://api.weixin.qq.com/cgi-bin/material/get_materialcount"
    );
    assert_eq!(
        material::material_batch_get(&cfg),
        "https://api.weixin.qq.com/cgi-bin/material/batchget_material"
    );
}

/// 对应 Java: WxMpApiUrl.ShakeAround 分组（shake 模块全部地址）。
#[test]
fn shake_urls() {
    let cfg = default_config();
    assert_eq!(
        shake::get_shake_info(&cfg),
        "https://api.weixin.qq.com/shakearound/user/getshakeinfo"
    );
    assert_eq!(
        shake::page_add(&cfg),
        "https://api.weixin.qq.com/shakearound/page/add"
    );
    assert_eq!(
        shake::device_bind_page(&cfg),
        "https://api.weixin.qq.com/shakearound/device/bindpage"
    );
    assert_eq!(
        shake::relation_search(&cfg),
        "https://api.weixin.qq.com/shakearound/relation/search"
    );
}

/// 对应 Java: WxMpApiUrl.Card 分组（card 模块全部地址）。
#[test]
fn card_urls() {
    let cfg = default_config();
    assert_eq!(
        card::card_create(&cfg),
        "https://api.weixin.qq.com/card/create"
    );
    assert_eq!(card::card_get(&cfg), "https://api.weixin.qq.com/card/get");
    assert_eq!(
        card::card_get_ticket(&cfg),
        "https://api.weixin.qq.com/cgi-bin/ticket/getticket?type=wx_card"
    );
    assert_eq!(
        card::card_code_decrypt(&cfg),
        "https://api.weixin.qq.com/card/code/decrypt"
    );
    assert_eq!(
        card::card_code_get(&cfg),
        "https://api.weixin.qq.com/card/code/get"
    );
    assert_eq!(
        card::card_code_consume(&cfg),
        "https://api.weixin.qq.com/card/code/consume"
    );
    assert_eq!(
        card::card_code_mark(&cfg),
        "https://api.weixin.qq.com/card/code/mark"
    );
    assert_eq!(
        card::card_test_whitelist(&cfg),
        "https://api.weixin.qq.com/card/testwhitelist/set"
    );
    assert_eq!(
        card::card_qrcode_create(&cfg),
        "https://api.weixin.qq.com/card/qrcode/create"
    );
    assert_eq!(
        card::card_landing_page_create(&cfg),
        "https://api.weixin.qq.com/card/landingpage/create"
    );
    assert_eq!(
        card::card_delete(&cfg),
        "https://api.weixin.qq.com/card/delete"
    );
}

/// 对应 Java: WxMpApiUrl.MemberCard 分组（member_card 模块全部地址）。
#[test]
fn member_card_urls() {
    let cfg = default_config();
    assert_eq!(
        member_card::member_card_create(&cfg),
        "https://api.weixin.qq.com/card/create"
    );
    assert_eq!(
        member_card::member_card_activate(&cfg),
        "https://api.weixin.qq.com/card/membercard/activate"
    );
    assert_eq!(
        member_card::member_card_user_info_get(&cfg),
        "https://api.weixin.qq.com/card/membercard/userinfo/get"
    );
    assert_eq!(
        member_card::member_card_update_user(&cfg),
        "https://api.weixin.qq.com/card/membercard/updateuser"
    );
    assert_eq!(
        member_card::member_card_activate_user_form(&cfg),
        "https://api.weixin.qq.com/card/membercard/activateuserform/set"
    );
    assert_eq!(
        member_card::member_card_update(&cfg),
        "https://api.weixin.qq.com/card/update"
    );
    assert_eq!(
        member_card::member_card_activate_temp_info(&cfg),
        "https://api.weixin.qq.com/card/membercard/activatetempinfo/get"
    );
    assert_eq!(
        member_card::member_card_activate_plugin(&cfg),
        "https://api.weixin.qq.com/card/membercard/activateplugin/get"
    );
}

/// 对应 Java: WxMpApiUrl.Guide 分组（guide 模块全部地址）。
#[test]
fn guide_urls() {
    let cfg = default_config();
    assert_eq!(
        guide::add_guide(&cfg),
        "https://api.weixin.qq.com/cgi-bin/guide/addguideacct"
    );
    assert_eq!(
        guide::update_guide(&cfg),
        "https://api.weixin.qq.com/cgi-bin/guide/updateguideacct"
    );
    assert_eq!(
        guide::get_guide(&cfg),
        "https://api.weixin.qq.com/cgi-bin/guide/getguideacct"
    );
    assert_eq!(
        guide::del_guide(&cfg),
        "https://api.weixin.qq.com/cgi-bin/guide/delguideacct"
    );
    assert_eq!(
        guide::list_guide(&cfg),
        "https://api.weixin.qq.com/cgi-bin/guide/getguideacctlist"
    );
    assert_eq!(
        guide::create_qr_code(&cfg),
        "https://api.weixin.qq.com/cgi-bin/guide/guidecreateqrcode"
    );
    assert_eq!(
        guide::get_guide_chat_record(&cfg),
        "https://api.weixin.qq.com/cgi-bin/guide/getguidebuyerchatrecord"
    );
    assert_eq!(
        guide::set_guide_config(&cfg),
        "https://api.weixin.qq.com/cgi-bin/guide/setguideconfig"
    );
    assert_eq!(
        guide::get_guide_config(&cfg),
        "https://api.weixin.qq.com/cgi-bin/guide/getguideconfig"
    );
    assert_eq!(
        guide::set_guide_acct_config(&cfg),
        "https://api.weixin.qq.com/cgi-bin/guide/setguideacctconfig"
    );
    assert_eq!(
        guide::get_guide_acct_config(&cfg),
        "https://api.weixin.qq.com/cgi-bin/guide/getguideacctconfig"
    );
    assert_eq!(
        guide::new_guide_group(&cfg),
        "https://api.weixin.qq.com/cgi-bin/guide/newguidegroup"
    );
    assert_eq!(
        guide::get_guide_group_list(&cfg),
        "https://api.weixin.qq.com/cgi-bin/guide/getguidegrouplist"
    );
    assert_eq!(
        guide::add_guide_buyer_relation(&cfg),
        "https://api.weixin.qq.com/cgi-bin/guide/addguidebuyerrelation"
    );
    assert_eq!(
        guide::del_guide_buyer_relation(&cfg),
        "https://api.weixin.qq.com/cgi-bin/guide/delguidebuyerrelation"
    );
    assert_eq!(
        guide::get_guide_buyer_relation_list(&cfg),
        "https://api.weixin.qq.com/cgi-bin/guide/getguidebuyerrelationlist"
    );
    assert_eq!(
        guide::rebind_guide_acct_for_buyer(&cfg),
        "https://api.weixin.qq.com/cgi-bin/guide/rebindguideacctforbuyer"
    );
    assert_eq!(
        guide::update_guide_buyer_relation(&cfg),
        "https://api.weixin.qq.com/cgi-bin/guide/updateguidebuyerrelation"
    );
    assert_eq!(
        guide::new_guide_tag_option(&cfg),
        "https://api.weixin.qq.com/cgi-bin/guide/newguidetagoption"
    );
    assert_eq!(
        guide::del_guide_tag_option(&cfg),
        "https://api.weixin.qq.com/cgi-bin/guide/delguidetagoption"
    );
    assert_eq!(
        guide::add_guide_tag_option(&cfg),
        "https://api.weixin.qq.com/cgi-bin/guide/addguidetagoption"
    );
    assert_eq!(
        guide::get_guide_tag_option(&cfg),
        "https://api.weixin.qq.com/cgi-bin/guide/getguidetagoption"
    );
    assert_eq!(
        guide::add_guide_buyer_tag(&cfg),
        "https://api.weixin.qq.com/cgi-bin/guide/addguidebuyertag"
    );
    assert_eq!(
        guide::get_guide_buyer_tag(&cfg),
        "https://api.weixin.qq.com/cgi-bin/guide/getguidebuyertag"
    );
    assert_eq!(
        guide::query_guide_buyer_by_tag(&cfg),
        "https://api.weixin.qq.com/cgi-bin/guide/queryguidebuyerbytag"
    );
    assert_eq!(
        guide::set_guide_card_material(&cfg),
        "https://api.weixin.qq.com/cgi-bin/guide/setguidecardmaterial"
    );
    assert_eq!(
        guide::get_guide_card_material(&cfg),
        "https://api.weixin.qq.com/cgi-bin/guide/getguidecardmaterial"
    );
    assert_eq!(
        guide::del_guide_card_material(&cfg),
        "https://api.weixin.qq.com/cgi-bin/guide/delguidecardmaterial"
    );
    assert_eq!(
        guide::set_guide_image_material(&cfg),
        "https://api.weixin.qq.com/cgi-bin/guide/setguideimagematerial"
    );
    assert_eq!(
        guide::get_guide_image_material(&cfg),
        "https://api.weixin.qq.com/cgi-bin/guide/getguideimagematerial"
    );
    assert_eq!(
        guide::del_guide_image_material(&cfg),
        "https://api.weixin.qq.com/cgi-bin/guide/delguideimagematerial"
    );
    assert_eq!(
        guide::set_guide_word_material(&cfg),
        "https://api.weixin.qq.com/cgi-bin/guide/setguidewordmaterial"
    );
    assert_eq!(
        guide::get_guide_word_material(&cfg),
        "https://api.weixin.qq.com/cgi-bin/guide/getguidewordmaterial"
    );
    assert_eq!(
        guide::del_guide_word_material(&cfg),
        "https://api.weixin.qq.com/cgi-bin/guide/delguidewordmaterial"
    );
    assert_eq!(
        guide::add_guide_massed_job(&cfg),
        "https://api.weixin.qq.com/cgi-bin/guide/addguidemassendjob"
    );
    assert_eq!(
        guide::get_guide_massed_job_list(&cfg),
        "https://api.weixin.qq.com/cgi-bin/guide/getguidemassendjoblist"
    );
    assert_eq!(
        guide::get_guide_massed_job(&cfg),
        "https://api.weixin.qq.com/cgi-bin/guide/getguidemassendjob"
    );
    assert_eq!(
        guide::update_guide_massed_job(&cfg),
        "https://api.weixin.qq.com/cgi-bin/guide/updateguidemassendjob"
    );
    assert_eq!(
        guide::cancel_guide_massed_job(&cfg),
        "https://api.weixin.qq.com/cgi-bin/guide/cancelguidemassendjob"
    );
}

/// 对应 Java: WxMpApiUrl.Marketing 分组（marketing 模块全部地址）。
#[test]
fn marketing_urls() {
    let cfg = default_config();
    assert_eq!(
        marketing::add_user_action_sets(&cfg),
        "https://api.weixin.qq.com/cgi-bin/marketing/user_action_sets/add"
    );
    assert_eq!(
        marketing::get_user_action_sets(&cfg),
        "https://api.weixin.qq.com/cgi-bin/marketing/user_action_sets/get"
    );
    assert_eq!(
        marketing::add_user_action(&cfg),
        "https://api.weixin.qq.com/cgi-bin/marketing/user_actions/add"
    );
    assert_eq!(
        marketing::get_ad_leads(&cfg),
        "https://api.weixin.qq.com/marketing/wechat_ad_leads/get"
    );
}

/// 对应 Java: WxMpApiUrl.SubscribeMsg 分组（subscribe_msg 模块全部地址）。
#[test]
fn subscribe_msg_urls() {
    let cfg = default_config();
    assert_eq!(
        subscribe_msg::send_once(&cfg),
        "https://api.weixin.qq.com/cgi-bin/message/template/subscribe"
    );
    assert_eq!(
        subscribe_msg::send(&cfg),
        "https://api.weixin.qq.com/cgi-bin/message/subscribe/bizsend"
    );
    assert_eq!(
        subscribe_msg::get_pub_template_title_list(&cfg),
        "https://api.weixin.qq.com/wxaapi/newtmpl/getpubtemplatetitles"
    );
    assert_eq!(
        subscribe_msg::get_pub_template_key_words_by_id(&cfg),
        "https://api.weixin.qq.com/wxaapi/newtmpl/getpubtemplatekeywords"
    );
    assert_eq!(
        subscribe_msg::template_add(&cfg),
        "https://api.weixin.qq.com/wxaapi/newtmpl/addtemplate"
    );
    assert_eq!(
        subscribe_msg::template_list(&cfg),
        "https://api.weixin.qq.com/wxaapi/newtmpl/gettemplate"
    );
    assert_eq!(
        subscribe_msg::template_del(&cfg),
        "https://api.weixin.qq.com/wxaapi/newtmpl/deltemplate"
    );
    assert_eq!(
        subscribe_msg::get_category(&cfg),
        "https://api.weixin.qq.com/wxaapi/newtmpl/getcategory"
    );
}

/// 对应 Java: WxMpApiUrl.AiOpen 分组（ai_open 模块全部地址）。
#[test]
fn ai_open_urls() {
    let cfg = default_config();
    assert_eq!(
        ai_open::voice_upload(&cfg, "mp3", "v1", "zh_CN"),
        "https://api.weixin.qq.com/cgi-bin/media/voice/addvoicetorecofortext?format=mp3&voice_id=v1&lang=zh_CN"
    );
    assert_eq!(
        ai_open::voice_query_result(&cfg),
        "https://api.weixin.qq.com/cgi-bin/media/voice/queryrecoresultfortext"
    );
    assert_eq!(
        ai_open::translate(&cfg, "en", "zh"),
        "https://api.weixin.qq.com/cgi-bin/media/voice/translatecontent?lfrom=en&lto=zh"
    );
}

/// 对应 Java: WxMpApiUrl.Ocr 分组（ocr 模块全部地址）。
#[test]
fn ocr_urls() {
    let cfg = default_config();
    assert_eq!(
        ocr::id_card(&cfg),
        "https://api.weixin.qq.com/cgi-bin/ocr/idcard"
    );
    assert_eq!(
        ocr::bank_card(&cfg),
        "https://api.weixin.qq.com/cgi-bin/ocr/bankcard"
    );
    assert_eq!(
        ocr::driving(&cfg),
        "https://api.weixin.qq.com/cgi-bin/ocr/driving"
    );
    assert_eq!(
        ocr::driving_license(&cfg),
        "https://api.weixin.qq.com/cgi-bin/ocr/drivinglicense"
    );
    assert_eq!(
        ocr::biz_license(&cfg),
        "https://api.weixin.qq.com/cgi-bin/ocr/bizlicense"
    );
    assert_eq!(
        ocr::comm(&cfg),
        "https://api.weixin.qq.com/cgi-bin/ocr/comm"
    );
}

/// 对应 Java: WxMpApiUrl.ImgProc 分组（img_proc 模块全部地址）。
#[test]
fn img_proc_urls() {
    let cfg = default_config();
    assert_eq!(
        img_proc::qr_code(&cfg),
        "https://api.weixin.qq.com/cgi-bin/imgproc/qrcode"
    );
    assert_eq!(
        img_proc::super_resolution(&cfg),
        "https://api.weixin.qq.com/cgi-bin/imgproc/superresolution"
    );
    assert_eq!(
        img_proc::ai_crop(&cfg),
        "https://api.weixin.qq.com/cgi-bin/imgproc/aicrop"
    );
}

/// 对应 Java: WxMpApiUrl.Invoice 分组（reimburse_invoice 模块全部地址）。
#[test]
fn reimburse_invoice_urls() {
    let cfg = default_config();
    assert_eq!(
        reimburse_invoice::get_invoice_info(&cfg),
        "https://api.weixin.qq.com/cgi-bin/invoice/getinvoicedetail"
    );
    assert_eq!(
        reimburse_invoice::get_invoice_batch(&cfg),
        "https://api.weixin.qq.com/cgi-bin/invoice/getinvoicebatch"
    );
    assert_eq!(
        reimburse_invoice::update_invoice_status(&cfg),
        "https://api.weixin.qq.com/cgi-bin/invoice/updateinvoicestatus"
    );
    assert_eq!(
        reimburse_invoice::update_status_batch(&cfg),
        "https://api.weixin.qq.com/cgi-bin/invoice/updatestatusbatch"
    );
}

/// 对应 Java: WxMpApiUrl.Invoice 分组（merchant_invoice 模块全部地址）。
#[test]
fn merchant_invoice_urls() {
    let cfg = default_config();
    assert_eq!(
        merchant_invoice::get_auth_url(&cfg),
        "https://api.weixin.qq.com/card/invoice/getauthurl"
    );
    assert_eq!(
        merchant_invoice::get_auth_data(&cfg),
        "https://api.weixin.qq.com/card/invoice/getauthdata"
    );
    assert_eq!(
        merchant_invoice::reject_insert(&cfg),
        "https://api.weixin.qq.com/card/invoice/rejectinsert"
    );
    assert_eq!(
        merchant_invoice::make_out_invoice(&cfg),
        "https://api.weixin.qq.com/card/invoice/makeoutinvoice"
    );
    assert_eq!(
        merchant_invoice::clear_out_invoice(&cfg),
        "https://api.weixin.qq.com/card/invoice/clearoutinvoice"
    );
    assert_eq!(
        merchant_invoice::query_invoice_info(&cfg),
        "https://api.weixin.qq.com/card/invoice/queryinvoceinfo"
    );
    assert_eq!(
        merchant_invoice::set_contact(&cfg),
        "https://api.weixin.qq.com/card/invoice/setbizattr?action=set_contact"
    );
    assert_eq!(
        merchant_invoice::get_contact(&cfg),
        "https://api.weixin.qq.com/card/invoice/setbizattr?action=query_contact"
    );
    assert_eq!(
        merchant_invoice::set_auth_page(&cfg),
        "https://api.weixin.qq.com/card/invoice/setbizattr?action=set_auth_field"
    );
    assert_eq!(
        merchant_invoice::get_auth_page(&cfg),
        "https://api.weixin.qq.com/card/invoice/setbizattr?action=query_auth_field"
    );
    assert_eq!(
        merchant_invoice::set_platform(&cfg),
        "https://api.weixin.qq.com/card/invoice/setbizattr?action=set_pay_mch"
    );
    assert_eq!(
        merchant_invoice::get_platform(&cfg),
        "https://api.weixin.qq.com/card/invoice/setbizattr?action=query_pay_mch"
    );
}

/// 对应 Java: WxMpApiUrl.OAuth2 分组（oauth2 模块全部地址）。
#[test]
fn oauth2_urls() {
    let cfg = default_config();
    assert_eq!(
        oauth2::sns_oauth2_access_token(&cfg, "wxappid2", "secret2", "CODE1"),
        "https://api.weixin.qq.com/sns/oauth2/access_token?appid=wxappid2&secret=secret2&code=CODE1&grant_type=authorization_code"
    );
    assert_eq!(
        oauth2::sns_oauth2_refresh_token(&cfg, "wxappid2", "RT1"),
        "https://api.weixin.qq.com/sns/oauth2/refresh_token?appid=wxappid2&grant_type=refresh_token&refresh_token=RT1"
    );
    assert_eq!(
        oauth2::sns_userinfo(&cfg, "AT1", "oX1", "zh_CN"),
        "https://api.weixin.qq.com/sns/userinfo?access_token=AT1&openid=oX1&lang=zh_CN"
    );
    assert_eq!(
        oauth2::sns_auth(&cfg, "AT1", "oX1"),
        "https://api.weixin.qq.com/sns/auth?access_token=AT1&openid=oX1"
    );
}
