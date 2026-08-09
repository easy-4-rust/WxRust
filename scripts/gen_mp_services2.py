#!/usr/bin/env python3
"""mp 子域服务批量生成器（第二批）：material/shake/card/memberCard/guide/marketing/
subscribeMsg/aiOpen/ocr/imgProc/reimburseInvoice/merchantInvoice。

严格镜像 Java `WxMp*ServiceImpl` 语义：URL + payload + 响应解析。
"""
import os

BASE = "/Users/wandl/workspaces/workspace-github-easy-4-rust/WxRust"
API = os.path.join(BASE, "crates", "wx-rust-mp", "src", "api")
ENUM = os.path.join(BASE, "crates", "wx-rust-mp", "src", "enums", "wx_mp_api_url.rs")

# ---------------------------------------------------------------- URL 模块
URLS = {
    "material": [
        ("media_upload", "/cgi-bin/media/upload?type=%s"),
        ("media_get", "/cgi-bin/media/get"),
        ("media_img_upload", "/cgi-bin/media/uploadimg"),
        ("material_add", "/cgi-bin/material/add_material?type=%s"),
        ("material_get", "/cgi-bin/material/get_material"),
        ("material_del", "/cgi-bin/material/del_material"),
        ("material_count", "/cgi-bin/material/get_materialcount"),
        ("material_batch_get", "/cgi-bin/material/batchget_material"),
    ],
    "shake": [
        ("get_shake_info", "/shakearound/user/getshakeinfo"),
        ("page_add", "/shakearound/page/add"),
        ("device_bind_page", "/shakearound/device/bindpage"),
        ("relation_search", "/shakearound/relation/search"),
    ],
    "card": [
        ("card_create", "/card/create"),
        ("card_get", "/card/get"),
        ("card_get_ticket", "/cgi-bin/ticket/getticket?type=wx_card"),
        ("card_code_decrypt", "/card/code/decrypt"),
        ("card_code_get", "/card/code/get"),
        ("card_code_consume", "/card/code/consume"),
        ("card_code_mark", "/card/code/mark"),
        ("card_test_whitelist", "/card/testwhitelist/set"),
        ("card_qrcode_create", "/card/qrcode/create"),
        ("card_landing_page_create", "/card/landingpage/create"),
        ("card_delete", "/card/delete"),
    ],
    "member_card": [
        ("member_card_create", "/card/create"),
        ("member_card_activate", "/card/membercard/activate"),
        ("member_card_user_info_get", "/card/membercard/userinfo/get"),
        ("member_card_update_user", "/card/membercard/updateuser"),
        ("member_card_activate_user_form", "/card/membercard/activateuserform/set"),
        ("member_card_update", "/card/update"),
        ("member_card_activate_temp_info", "/card/membercard/activatetempinfo/get"),
        ("member_card_activate_plugin", "/card/membercard/activateplugin/get"),
    ],
    "guide": [
        ("add_guide", "/cgi-bin/guide/addguideacct"),
        ("update_guide", "/cgi-bin/guide/updateguideacct"),
        ("get_guide", "/cgi-bin/guide/getguideacct"),
        ("del_guide", "/cgi-bin/guide/delguideacct"),
        ("list_guide", "/cgi-bin/guide/getguideacctlist"),
        ("create_qr_code", "/cgi-bin/guide/guidecreateqrcode"),
        ("get_guide_chat_record", "/cgi-bin/guide/getguidebuyerchatrecord"),
        ("set_guide_config", "/cgi-bin/guide/setguideconfig"),
        ("get_guide_config", "/cgi-bin/guide/getguideconfig"),
        ("set_guide_acct_config", "/cgi-bin/guide/setguideacctconfig"),
        ("get_guide_acct_config", "/cgi-bin/guide/getguideacctconfig"),
        ("new_guide_group", "/cgi-bin/guide/newguidegroup"),
        ("get_guide_group_list", "/cgi-bin/guide/getguidegrouplist"),
    ],
    "marketing": [
        ("add_user_action_sets", "/cgi-bin/marketing/user_action_sets/add"),
        ("get_user_action_sets", "/cgi-bin/marketing/user_action_sets/get"),
        ("add_user_action", "/cgi-bin/marketing/user_actions/add"),
        ("get_ad_leads", "/marketing/wechat_ad_leads/get"),
    ],
    "subscribe_msg": [
        ("send_once", "/cgi-bin/message/template/subscribe"),
        ("send", "/cgi-bin/message/subscribe/bizsend"),
        ("get_pub_template_title_list", "/wxaapi/newtmpl/getpubtemplatetitles"),
        ("get_pub_template_key_words_by_id", "/wxaapi/newtmpl/getpubtemplatekeywords"),
        ("template_add", "/wxaapi/newtmpl/addtemplate"),
        ("template_list", "/wxaapi/newtmpl/gettemplate"),
        ("template_del", "/wxaapi/newtmpl/deltemplate"),
        ("get_category", "/wxaapi/newtmpl/getcategory"),
    ],
    "ai_open": [
        ("voice_upload", "/cgi-bin/media/voice/addvoicetorecofortext?format=%s&voice_id=%s&lang=%s"),
        ("voice_query_result", "/cgi-bin/media/voice/queryrecoresultfortext"),
        ("translate", "/cgi-bin/media/voice/translatecontent?lfrom=%s&lto=%s"),
    ],
    "ocr": [
        ("id_card", "/cgi-bin/ocr/idcard"),
        ("bank_card", "/cgi-bin/ocr/bankcard"),
        ("driving", "/cgi-bin/ocr/driving"),
        ("driving_license", "/cgi-bin/ocr/drivinglicense"),
        ("biz_license", "/cgi-bin/ocr/bizlicense"),
        ("comm", "/cgi-bin/ocr/comm"),
    ],
    "img_proc": [
        ("qr_code", "/cgi-bin/imgproc/qrcode"),
        ("super_resolution", "/cgi-bin/imgproc/superresolution"),
        ("ai_crop", "/cgi-bin/imgproc/aicrop"),
    ],
    "reimburse_invoice": [
        ("get_invoice_info", "/cgi-bin/invoice/getinvoicedetail"),
        ("get_invoice_batch", "/cgi-bin/invoice/getinvoicebatch"),
        ("update_invoice_status", "/cgi-bin/invoice/updateinvoicestatus"),
        ("update_status_batch", "/cgi-bin/invoice/updatestatusbatch"),
    ],
    "merchant_invoice": [
        ("get_auth_url", "/card/invoice/getauthurl"),
        ("get_auth_data", "/card/invoice/getauthdata"),
        ("reject_insert", "/card/invoice/rejectinsert"),
        ("make_out_invoice", "/card/invoice/makeoutinvoice"),
        ("clear_out_invoice", "/card/invoice/clearoutinvoice"),
        ("query_invoice_info", "/card/invoice/queryinvoceinfo"),
        ("set_contact", "/card/invoice/setbizattr?action=set_contact"),
        ("get_contact", "/card/invoice/setbizattr?action=query_contact"),
        ("set_auth_page", "/card/invoice/setbizattr?action=set_auth_field"),
        ("get_auth_page", "/card/invoice/setbizattr?action=query_auth_field"),
        ("set_platform", "/card/invoice/setbizattr?action=set_pay_mch"),
        ("get_platform", "/card/invoice/setbizattr?action=query_pay_mch"),
    ],
}


def gen_url_modules():
    lines = []
    for mod, items in URLS.items():
        lines.append(f"\n/// {mod} 接口地址（对应 Java `WxMpApiUrl.{mod}`）。")
        lines.append(f"pub mod {mod} {{")
        lines.append("    use super::*;")
        lines.append("")
        for name, path in items:
            lines.append(f"    /// {path}")
            if "%s" in path:
                lines.append(f'    pub fn {name}(config: &dyn WxMpConfigStorage, arg: &str) -> String {{')
                lines.append("        let h = config.host_config();")
                lines.append(f'        url(config, &h.api_host, &format!("{path}", arg))')
            else:
                lines.append(f'    pub fn {name}(config: &dyn WxMpConfigStorage) -> String {{')
                lines.append("        let h = config.host_config();")
                lines.append(f'        url(config, &h.api_host, "{path}")')
            lines.append("    }")
            lines.append("")
        lines.append("}")
    return "\n".join(lines)


if __name__ == "__main__":
    with open(ENUM, "a", encoding="utf-8") as f:
        f.write(gen_url_modules())
    print("url modules appended")
