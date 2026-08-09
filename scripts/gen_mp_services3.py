#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""109 批次：guide 4 子服务 + OAuth2 impl 生成。"""
import os

BASE = "/Users/wandl/workspaces/workspace-github-easy-4-rust/WxRust"
API = os.path.join(BASE, "crates", "wx-rust-mp", "src", "api")
ENUM = os.path.join(BASE, "crates", "wx-rust-mp", "src", "enums", "wx_mp_api_url.rs")

# ---- guide 子服务 URL 模块（追加到已有 guide 模块） ----
guide_extra = '''
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
        url(config, &h.api_host, "/cgi-bin/guide/getguidebuyerrelationlist")
    }

    /// /cgi-bin/guide/rebindguideacctforbuyer
    pub fn rebind_guide_acct_for_buyer(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/guide/rebindguideacctforbuyer")
    }

    /// /cgi-bin/guide/updateguidebuyerrelation
    pub fn update_guide_buyer_relation(config: &dyn WxMpConfigStorage) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/guide/updateguidebuyerrelation")
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
'''
# 追加到 guide 模块结尾（找 guide 模块的收尾 `}`）
src = open(ENUM, encoding="utf-8").read()
guide_end = src.find("/// guide 接口地址")
if guide_end == -1:
    # 找 pub mod guide 块的结束
    start = src.find("pub mod guide {")
    if start != -1:
        end = src.find("\n}", start)
        src = src[:end] + guide_extra + src[end:]
        open(ENUM, "w", encoding="utf-8").write(src)
        print("guide url module extended")
    else:
        print("guide module not found!")
else:
    print("guide module marker found at", guide_end)
