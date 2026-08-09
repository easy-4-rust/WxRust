//! 企业微信成员（用户）信息。
//!
//! 对应 Java `me.chanjar.weixin.cp.bean.WxCpUser`，线格式以
//! `util/json/WxCpUserGsonAdapter` 为准（`userid`/`department`/`order`/
//! `extattr`/`external_profile` 等均为 adapter 自定义字段名）：
//!
//! - 序列化：字段按 Java adapter 的写入顺序输出；null 字段省略；
//!   `direct_leader` 非 null（含空数组）必输出（支持清空直连上级）；
//!   `external_profile` 恒输出（无内容时为 `{}`）；
//! - 反序列化：`gender` 为字符串码（"0" 未定义/"1" 男/"2" 女），未知码
//!   视为 None（对应 Java `Gender.fromCode` 返回 null）；
//!   `extattr.attrs` 兼容 `type==null → value` 直挂形态。

use serde::de::Error as DeError;
use serde::ser::Error as SerError;
/// 插入有序 JSON 对象（serde_json `preserve_order` 特性开启，保持
/// Java adapter 的字段写入顺序）。
type JsonMap = serde_json::Map<String, serde_json::Value>;

use super::Gender;

/// 企业微信成员（用户）。
#[derive(Debug, Clone, Default, PartialEq)]
pub struct WxCpUser {
    /// 成员 UserID（对应 Java `userId`，wire `userid`）。
    pub user_id: Option<String>,
    /// 新成员 UserID（对应 Java `newUserId`，wire `new_userid`）。
    pub new_user_id: Option<String>,
    /// 成员名称（对应 Java `name`）。
    pub name: Option<String>,
    /// 成员所属部门 id 列表（对应 Java `departIds`，wire `department`）。
    pub depart_ids: Option<Vec<i64>>,
    /// 部门内的排序值（对应 Java `orders`，wire `order`）。
    pub orders: Option<Vec<i32>>,
    /// 职位信息（对应 Java `position`）。
    pub position: Option<String>,
    /// 多个职位信息（对应 Java `positions`，wire `positions`）。
    pub positions: Option<Vec<String>>,
    /// 手机号码（对应 Java `mobile`）。
    pub mobile: Option<String>,
    /// 性别（对应 Java `Gender gender`，wire 为字符串码）。
    pub gender: Option<Gender>,
    /// 邮箱（对应 Java `email`）。
    pub email: Option<String>,
    /// 企业邮箱（对应 Java `bizMail`，wire `biz_mail`）。
    pub biz_mail: Option<String>,
    /// 头像 URL（对应 Java `avatar`）。
    pub avatar: Option<String>,
    /// 头像缩略图 URL（对应 Java `thumbAvatar`，wire `thumb_avatar`）。
    pub thumb_avatar: Option<String>,
    /// 地址（对应 Java `address`）。
    pub address: Option<String>,
    /// 头像媒体 id（对应 Java `avatarMediaId`，wire `avatar_mediaid`）。
    pub avatar_media_id: Option<String>,
    /// 激活状态（对应 Java `status`）。
    pub status: Option<i32>,
    /// 是否启用（对应 Java `enable`）。
    pub enable: Option<i32>,
    /// 别名（对应 Java `alias`）。
    pub alias: Option<String>,
    /// 是否上级（对应 Java `isLeader`，wire `isleader`）。
    pub is_leader: Option<i32>,
    /// 在所在部门内是否为上级（对应 Java `isLeaderInDept`，
    /// wire `is_leader_in_dept`；个数与 department 一致）。
    pub is_leader_in_dept: Option<Vec<i32>>,
    /// 是否隐藏手机号（对应 Java `hideMobile`，wire `hide_mobile`）。
    pub hide_mobile: Option<i32>,
    /// 英文名（对应 Java `englishName`，wire `english_name`）。
    pub english_name: Option<String>,
    /// 座机（对应 Java `telephone`）。
    pub telephone: Option<String>,
    /// 二维码（对应 Java `qrCode`，wire `qr_code`）。
    pub qr_code: Option<String>,
    /// 是否邀请该成员（对应 Java `toInvite`，wire `to_invite`）。
    pub to_invite: Option<bool>,
    /// 全局唯一 open_userid（对应 Java `openUserId`，wire `open_userid`）。
    pub open_user_id: Option<String>,
    /// 主部门（对应 Java `mainDepartment`，wire `main_department`）。
    pub main_department: Option<String>,
    /// 直属上级（对应 Java `directLeader`，wire `direct_leader`；
    /// 空数组也输出，用于重置直连上级）。
    pub direct_leader: Option<Vec<String>>,
    /// 扩展属性（对应 Java `extAttrs`，wire `extattr.attrs`）。
    pub ext_attrs: Vec<Attr>,
    /// 成员对外职位（对应 Java `externalPosition`，wire `external_position`）。
    pub external_position: Option<String>,
    /// 成员对外信息（对应 Java `externalAttrs`/`externalCorpName`/
    /// `wechatChannels`，wire `external_profile`；恒输出）。
    pub external_profile: ExternalProfile,
}

impl WxCpUser {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpUser 解析失败: {e}"))
    }

    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxCpUser 序列化失败: {e}"))
    }

    /// 添加扩展属性（对应 Java `addExtAttr(String name, String value)`，
    /// 文本类型 type=0）。
    pub fn add_ext_attr(&mut self, name: impl Into<String>, value: impl Into<String>) {
        self.ext_attrs.push(Attr {
            r#type: Some(0),
            name: Some(name.into()),
            text_value: Some(value.into()),
            web_url: None,
            web_title: None,
        });
    }

    /// 添加成员对外属性（对应 Java `addExternalAttr(ExternalAttribute)`）。
    pub fn add_external_attr(&mut self, external_attr: ExternalAttribute) {
        self.external_profile.external_attrs.push(external_attr);
    }
}

impl serde::Serialize for WxCpUser {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut m = JsonMap::new();
        let put = |m: &mut JsonMap, k: &'static str, v: Option<serde_json::Value>| {
            if let Some(v) = v {
                m.insert(k.to_string(), v);
            }
        };
        put(
            &mut m,
            "userid",
            self.user_id.as_deref().map(serde_json::Value::from),
        );
        put(
            &mut m,
            "new_userid",
            self.new_user_id.as_deref().map(serde_json::Value::from),
        );
        put(
            &mut m,
            "name",
            self.name.as_deref().map(serde_json::Value::from),
        );
        put(
            &mut m,
            "department",
            self.depart_ids
                .as_ref()
                .filter(|v| !v.is_empty())
                .map(|v| serde_json::to_value(v).map_err(S::Error::custom))
                .transpose()?,
        );
        put(
            &mut m,
            "order",
            self.orders
                .as_ref()
                .filter(|v| !v.is_empty())
                .map(|v| serde_json::to_value(v).map_err(S::Error::custom))
                .transpose()?,
        );
        put(
            &mut m,
            "position",
            self.position.as_deref().map(serde_json::Value::from),
        );
        put(
            &mut m,
            "positions",
            self.positions
                .as_ref()
                .filter(|v| !v.is_empty())
                .map(|v| serde_json::to_value(v).map_err(S::Error::custom))
                .transpose()?,
        );
        put(
            &mut m,
            "mobile",
            self.mobile.as_deref().map(serde_json::Value::from),
        );
        if let Some(g) = &self.gender {
            // Java: o.addProperty(GENDER, user.getGender().getCode())
            put(&mut m, "gender", Some(serde_json::Value::from(g.code())));
        }
        put(
            &mut m,
            "email",
            self.email.as_deref().map(serde_json::Value::from),
        );
        put(
            &mut m,
            "biz_mail",
            self.biz_mail.as_deref().map(serde_json::Value::from),
        );
        put(
            &mut m,
            "avatar",
            self.avatar.as_deref().map(serde_json::Value::from),
        );
        put(
            &mut m,
            "thumb_avatar",
            self.thumb_avatar.as_deref().map(serde_json::Value::from),
        );
        put(
            &mut m,
            "address",
            self.address.as_deref().map(serde_json::Value::from),
        );
        put(
            &mut m,
            "avatar_mediaid",
            self.avatar_media_id.as_deref().map(serde_json::Value::from),
        );
        put(&mut m, "status", self.status.map(serde_json::Value::from));
        put(&mut m, "enable", self.enable.map(serde_json::Value::from));
        put(
            &mut m,
            "alias",
            self.alias.as_deref().map(serde_json::Value::from),
        );
        put(
            &mut m,
            "isleader",
            self.is_leader.map(serde_json::Value::from),
        );
        put(
            &mut m,
            "is_leader_in_dept",
            self.is_leader_in_dept
                .as_ref()
                .filter(|v| !v.is_empty())
                .map(|v| serde_json::to_value(v).map_err(S::Error::custom))
                .transpose()?,
        );
        put(
            &mut m,
            "hide_mobile",
            self.hide_mobile.map(serde_json::Value::from),
        );
        put(
            &mut m,
            "english_name",
            self.english_name.as_deref().map(serde_json::Value::from),
        );
        put(
            &mut m,
            "telephone",
            self.telephone.as_deref().map(serde_json::Value::from),
        );
        put(
            &mut m,
            "qr_code",
            self.qr_code.as_deref().map(serde_json::Value::from),
        );
        if let Some(t) = &self.to_invite {
            put(&mut m, "to_invite", Some(serde_json::Value::from(*t)));
        }
        put(
            &mut m,
            "open_userid",
            self.open_user_id.as_deref().map(serde_json::Value::from),
        );
        put(
            &mut m,
            "main_department",
            self.main_department.as_deref().map(serde_json::Value::from),
        );
        // Java: directLeader 非 null 时必输出（含空数组，用于清空）
        if let Some(leaders) = &self.direct_leader {
            m.insert(
                "direct_leader".to_string(),
                serde_json::to_value(leaders).map_err(S::Error::custom)?,
            );
        }
        if !self.ext_attrs.is_empty() {
            let attrs = serde_json::to_value(&self.ext_attrs).map_err(S::Error::custom)?;
            let mut wrap = JsonMap::new();
            wrap.insert("attrs".to_string(), attrs);
            m.insert("extattr".to_string(), serde_json::Value::Object(wrap));
        }
        put(
            &mut m,
            "external_position",
            self.external_position
                .as_deref()
                .map(serde_json::Value::from),
        );
        // Java: external_profile 恒输出（无内容时为 {}）
        m.insert(
            "external_profile".to_string(),
            serde_json::to_value(&self.external_profile).map_err(S::Error::custom)?,
        );
        s.collect_map(m)
    }
}

impl<'de> serde::Deserialize<'de> for WxCpUser {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let v = serde_json::Value::deserialize(d)?;
        let obj = v
            .as_object()
            .ok_or_else(|| D::Error::custom("WxCpUser 期望 JSON 对象"))?;
        let get = |k: &str| obj.get(k);
        let str_opt = |k: &str| {
            get(k)
                .and_then(serde_json::Value::as_str)
                .map(str::to_string)
        };
        let arr_opt = |k: &str| {
            get(k).and_then(serde_json::Value::as_array).map(|a| {
                a.iter()
                    .filter_map(serde_json::Value::as_str)
                    .map(str::to_string)
                    .collect::<Vec<_>>()
            })
        };
        let mut depart_ids = None;
        if let Some(a) = get("department").and_then(serde_json::Value::as_array) {
            depart_ids = Some(a.iter().filter_map(|e| e.as_i64()).collect());
        }
        let mut orders = None;
        if let Some(a) = get("order").and_then(serde_json::Value::as_array) {
            orders = Some(
                a.iter()
                    .filter_map(|e| e.as_i64().map(|v| v as i32))
                    .collect(),
            );
        }
        let mut is_leader_in_dept = None;
        if let Some(a) = get("is_leader_in_dept").and_then(serde_json::Value::as_array) {
            is_leader_in_dept = Some(
                a.iter()
                    .filter_map(|e| e.as_i64().map(|v| v as i32))
                    .collect(),
            );
        }
        // 性别：字符串码 "0"/"1"/"2" → Gender；未知码 → None（Java fromCode null）
        let gender = str_opt("gender").and_then(|c| Gender::from_code(&c));

        // extattr.attrs（Java buildExtraAttrs）
        let mut ext_attrs = Vec::new();
        if let Some(extattr) = get("extattr").and_then(serde_json::Value::as_object) {
            if let Some(attrs) = extattr.get("attrs").and_then(serde_json::Value::as_array) {
                for a in attrs {
                    if let Some(a) = a.as_object() {
                        // type==null 的 value 直挂形态由 Attr 反序列化兼容
                        let attr: Attr =
                            serde_json::from_value(serde_json::Value::Object(a.clone()))
                                .map_err(D::Error::custom)?;
                        ext_attrs.push(attr);
                    }
                }
            }
        }

        // external_profile（Java buildExternalAttrs）
        let mut external_profile = ExternalProfile::default();
        if let Some(profile) = get("external_profile").and_then(serde_json::Value::as_object) {
            external_profile.external_corp_name = profile
                .get("external_corp_name")
                .and_then(serde_json::Value::as_str)
                .map(str::to_string);
            if let Some(wc) = profile
                .get("wechat_channels")
                .and_then(serde_json::Value::as_object)
            {
                external_profile.wechat_channels = Some(WechatChannels {
                    nickname: wc
                        .get("nickname")
                        .and_then(serde_json::Value::as_str)
                        .map(str::to_string),
                    status: wc
                        .get("status")
                        .and_then(serde_json::Value::as_i64)
                        .map(|v| v as i32),
                });
            }
            if let Some(attrs) = profile
                .get("external_attr")
                .and_then(serde_json::Value::as_array)
            {
                for a in attrs {
                    if let Some(a) = a.as_object() {
                        let Some(type_val) = a.get("type").and_then(serde_json::Value::as_i64)
                        else {
                            continue; // Java：type==null 跳过
                        };
                        let name = a
                            .get("name")
                            .and_then(serde_json::Value::as_str)
                            .map(str::to_string);
                        let mut ea = ExternalAttribute {
                            r#type: type_val as i32,
                            name,
                            value: None,
                            url: None,
                            title: None,
                            appid: None,
                            page_path: None,
                        };
                        match type_val {
                            0 => {
                                if let Some(text) =
                                    a.get("text").and_then(serde_json::Value::as_object)
                                {
                                    ea.value = text
                                        .get("value")
                                        .and_then(serde_json::Value::as_str)
                                        .map(str::to_string);
                                }
                            }
                            1 => {
                                if let Some(web) =
                                    a.get("web").and_then(serde_json::Value::as_object)
                                {
                                    ea.url = web
                                        .get("url")
                                        .and_then(serde_json::Value::as_str)
                                        .map(str::to_string);
                                    ea.title = web
                                        .get("title")
                                        .and_then(serde_json::Value::as_str)
                                        .map(str::to_string);
                                }
                            }
                            2 => {
                                if let Some(mp) =
                                    a.get("miniprogram").and_then(serde_json::Value::as_object)
                                {
                                    ea.appid = mp
                                        .get("appid")
                                        .and_then(serde_json::Value::as_str)
                                        .map(str::to_string);
                                    ea.page_path = mp
                                        .get("pagepath")
                                        .and_then(serde_json::Value::as_str)
                                        .map(str::to_string);
                                    ea.title = mp
                                        .get("title")
                                        .and_then(serde_json::Value::as_str)
                                        .map(str::to_string);
                                }
                            }
                            _ => {}
                        }
                        external_profile.external_attrs.push(ea);
                    }
                }
            }
        }

        Ok(WxCpUser {
            user_id: str_opt("userid"),
            new_user_id: str_opt("new_userid"),
            name: str_opt("name"),
            depart_ids,
            orders,
            position: str_opt("position"),
            positions: arr_opt("positions"),
            mobile: str_opt("mobile"),
            gender,
            email: str_opt("email"),
            biz_mail: str_opt("biz_mail"),
            avatar: str_opt("avatar"),
            thumb_avatar: str_opt("thumb_avatar"),
            address: str_opt("address"),
            avatar_media_id: str_opt("avatar_mediaid"),
            status: get("status")
                .and_then(serde_json::Value::as_i64)
                .map(|v| v as i32),
            enable: get("enable")
                .and_then(serde_json::Value::as_i64)
                .map(|v| v as i32),
            alias: str_opt("alias"),
            is_leader: get("isleader")
                .and_then(serde_json::Value::as_i64)
                .map(|v| v as i32),
            is_leader_in_dept,
            hide_mobile: get("hide_mobile")
                .and_then(serde_json::Value::as_i64)
                .map(|v| v as i32),
            english_name: str_opt("english_name"),
            telephone: str_opt("telephone"),
            qr_code: str_opt("qr_code"),
            to_invite: get("to_invite").and_then(serde_json::Value::as_bool),
            open_user_id: str_opt("open_userid"),
            main_department: str_opt("main_department"),
            direct_leader: arr_opt("direct_leader"),
            ext_attrs,
            external_position: str_opt("external_position"),
            external_profile,
        })
    }
}

/// 扩展属性项（对应 Java `WxCpUser.Attr`）。
///
/// 线格式（Java `buildExtraAttrs`/serialize）：
/// - `type == null`：`{"name":..., "value":...}` 直挂；
/// - `type == 0`（文本）：`{"type":0,"name":...,"text":{"value":...}}`；
/// - `type == 1`（网页）：`{"type":1,"name":...,"web":{"url":...,"title":...}}`；
/// - 其他类型忽略（不输出内容）。
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Attr {
    /// 属性类型: 0-文本 1-网页。
    pub r#type: Option<i32>,
    /// 属性名称。
    pub name: Option<String>,
    /// 文本属性内容（type 0）。
    pub text_value: Option<String>,
    /// 网页 url（type 1）。
    pub web_url: Option<String>,
    /// 网页展示标题（type 1）。
    pub web_title: Option<String>,
}

impl serde::Serialize for Attr {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut m = JsonMap::new();
        if let Some(t) = self.r#type {
            m.insert("type".to_string(), serde_json::Value::from(t));
        }
        if let Some(n) = &self.name {
            m.insert("name".to_string(), serde_json::Value::from(n.as_str()));
        }
        match self.r#type {
            None => {
                // Java：type==null 时输出 value 直挂
                if let Some(v) = &self.text_value {
                    m.insert("value".to_string(), serde_json::Value::from(v.as_str()));
                }
            }
            Some(0) => {
                let mut text = JsonMap::new();
                if let Some(v) = &self.text_value {
                    text.insert("value".to_string(), serde_json::Value::from(v.as_str()));
                }
                m.insert("text".to_string(), serde_json::Value::Object(text));
            }
            Some(1) => {
                let mut web = JsonMap::new();
                if let Some(u) = &self.web_url {
                    web.insert("url".to_string(), serde_json::Value::from(u.as_str()));
                }
                if let Some(t) = &self.web_title {
                    web.insert("title".to_string(), serde_json::Value::from(t.as_str()));
                }
                m.insert("web".to_string(), serde_json::Value::Object(web));
            }
            _ => {}
        }
        s.collect_map(m)
    }
}

impl<'de> serde::Deserialize<'de> for Attr {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let a = serde_json::Value::deserialize(d)?;
        let obj = a
            .as_object()
            .ok_or_else(|| D::Error::custom("Attr 期望 JSON 对象"))?;
        let name = obj
            .get("name")
            .and_then(serde_json::Value::as_str)
            .map(str::to_string);
        let type_val = obj
            .get("type")
            .and_then(serde_json::Value::as_i64)
            .map(|v| v as i32);
        let mut attr = Attr {
            r#type: type_val,
            name,
            text_value: None,
            web_url: None,
            web_title: None,
        };
        match type_val {
            None => {
                attr.text_value = obj
                    .get("value")
                    .and_then(serde_json::Value::as_str)
                    .map(str::to_string);
            }
            Some(0) => {
                if let Some(text) = obj.get("text").and_then(serde_json::Value::as_object) {
                    attr.text_value = text
                        .get("value")
                        .and_then(serde_json::Value::as_str)
                        .map(str::to_string);
                }
            }
            Some(1) => {
                if let Some(web) = obj.get("web").and_then(serde_json::Value::as_object) {
                    attr.web_url = web
                        .get("url")
                        .and_then(serde_json::Value::as_str)
                        .map(str::to_string);
                    attr.web_title = web
                        .get("title")
                        .and_then(serde_json::Value::as_str)
                        .map(str::to_string);
                }
            }
            _ => {}
        }
        Ok(attr)
    }
}

/// 成员对外属性（对应 Java `WxCpUser.ExternalAttribute`）。
///
/// 线格式：`{"type":0,"name":...,"text":{"value":...}}`（文本）/
/// `{"type":1,...,"web":{"url":...,"title":...}}`（网页）/
/// `{"type":2,...,"miniprogram":{"appid":...,"pagepath":...,"title":...}}`（小程序）。
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ExternalAttribute {
    /// 属性类型: 0-文本 1-网页 2-小程序。
    pub r#type: i32,
    /// 属性名称。
    pub name: Option<String>,
    /// 文本属性内容（type 0）。
    pub value: Option<String>,
    /// 网页 url（type 1）。
    pub url: Option<String>,
    /// 展示标题（type 1 网页 / type 2 小程序）。
    pub title: Option<String>,
    /// 小程序 appid（type 2）。
    pub appid: Option<String>,
    /// 小程序页面路径（type 2，wire `pagepath`）。
    pub page_path: Option<String>,
}

impl serde::Serialize for ExternalAttribute {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut m = JsonMap::new();
        m.insert("type".to_string(), serde_json::Value::from(self.r#type));
        if let Some(n) = &self.name {
            m.insert("name".to_string(), serde_json::Value::from(n.as_str()));
        }
        match self.r#type {
            0 => {
                let mut text = JsonMap::new();
                if let Some(v) = &self.value {
                    text.insert("value".to_string(), serde_json::Value::from(v.as_str()));
                }
                m.insert("text".to_string(), serde_json::Value::Object(text));
            }
            1 => {
                let mut web = JsonMap::new();
                if let Some(u) = &self.url {
                    web.insert("url".to_string(), serde_json::Value::from(u.as_str()));
                }
                if let Some(t) = &self.title {
                    web.insert("title".to_string(), serde_json::Value::from(t.as_str()));
                }
                m.insert("web".to_string(), serde_json::Value::Object(web));
            }
            2 => {
                let mut mp = JsonMap::new();
                if let Some(a) = &self.appid {
                    mp.insert("appid".to_string(), serde_json::Value::from(a.as_str()));
                }
                if let Some(p) = &self.page_path {
                    mp.insert("pagepath".to_string(), serde_json::Value::from(p.as_str()));
                }
                if let Some(t) = &self.title {
                    mp.insert("title".to_string(), serde_json::Value::from(t.as_str()));
                }
                m.insert("miniprogram".to_string(), serde_json::Value::Object(mp));
            }
            _ => {}
        }
        s.collect_map(m)
    }
}

impl<'de> serde::Deserialize<'de> for ExternalAttribute {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let a = serde_json::Value::deserialize(d)?;
        let obj = a
            .as_object()
            .ok_or_else(|| D::Error::custom("ExternalAttribute 期望 JSON 对象"))?;
        let name = obj
            .get("name")
            .and_then(serde_json::Value::as_str)
            .map(str::to_string);
        let mut ea = ExternalAttribute {
            r#type: obj
                .get("type")
                .and_then(serde_json::Value::as_i64)
                .unwrap_or_default() as i32,
            name,
            value: None,
            url: None,
            title: None,
            appid: None,
            page_path: None,
        };
        match ea.r#type {
            0 => {
                if let Some(text) = obj.get("text").and_then(serde_json::Value::as_object) {
                    ea.value = text
                        .get("value")
                        .and_then(serde_json::Value::as_str)
                        .map(str::to_string);
                }
            }
            1 => {
                if let Some(web) = obj.get("web").and_then(serde_json::Value::as_object) {
                    ea.url = web
                        .get("url")
                        .and_then(serde_json::Value::as_str)
                        .map(str::to_string);
                    ea.title = web
                        .get("title")
                        .and_then(serde_json::Value::as_str)
                        .map(str::to_string);
                }
            }
            2 => {
                if let Some(mp) = obj
                    .get("miniprogram")
                    .and_then(serde_json::Value::as_object)
                {
                    ea.appid = mp
                        .get("appid")
                        .and_then(serde_json::Value::as_str)
                        .map(str::to_string);
                    ea.page_path = mp
                        .get("pagepath")
                        .and_then(serde_json::Value::as_str)
                        .map(str::to_string);
                    ea.title = mp
                        .get("title")
                        .and_then(serde_json::Value::as_str)
                        .map(str::to_string);
                }
            }
            _ => {}
        }
        Ok(ea)
    }
}

/// 视频号（对应 Java `WxCpUser.WechatChannels`，wire `wechat_channels`）。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WechatChannels {
    /// 视频号名字。
    #[serde(rename = "nickname", skip_serializing_if = "Option::is_none", default)]
    pub nickname: Option<String>,
    /// 视频号状态。
    #[serde(rename = "status", skip_serializing_if = "Option::is_none", default)]
    pub status: Option<i32>,
}

/// 成员对外信息（对应 Java `WxCpUser` 的 `external_profile` 子对象）。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ExternalProfile {
    /// 对外企业简称（wire `external_corp_name`）。
    #[serde(
        rename = "external_corp_name",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub external_corp_name: Option<String>,
    /// 视频号信息（wire `wechat_channels`）。
    #[serde(
        rename = "wechat_channels",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub wechat_channels: Option<WechatChannels>,
    /// 成员对外属性列表（wire `external_attr`）。
    #[serde(
        rename = "external_attr",
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    pub external_attrs: Vec<ExternalAttribute>,
}
