//! 订阅消息。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.bean.WxMaSubscribeMessage`。
//! 线格式由 `WxMaSubscribeMessageGsonAdapter` 决定：`touser`/`template_id`/
//! `page`/`miniprogram_state`/`lang` + `data`（`{名称: {"value": ...}}`）。
//! 发送前按 `resetValue` 语义清洗各类型模板字段（thing/number/letter/symbol/
//! character_string/phone_number/car_number/name/phrase），以纯 Rust 字符过滤
//! 实现 Java 预编译正则等价逻辑（CJK 区间 `\u4e00-\u9fa5`）。

use serde::{Deserialize, Serialize};

/// 订阅消息模板数据项。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MsgData {
    /// 模板字段名（如 `thing1`）。
    pub name: String,
    /// 模板字段值。
    pub value: String,
}

/// 订阅消息（对应 Java `WxMaSubscribeMessage`）。
///
/// 序列化走手写 `Serialize`（`WxMaSubscribeMessageGsonAdapter` 线格式：
/// `data` 为 `{名称: {"value": ...}}` 对象）；`Deserialize` 派生为平铺
/// 解析便利（Java 无 fromJson，线格式不会回传）。
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct WxMaSubscribeMessage {
    /// 接收者（用户）的 openid。
    #[serde(rename = "touser", skip_serializing_if = "Option::is_none")]
    pub to_user: Option<String>,
    /// 所需下发的模板消息的 id。
    #[serde(rename = "template_id", skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    /// 点击模板卡片后的跳转页面（仅限本小程序内的页面）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,
    /// 模板内容，不填则下发空模板。
    #[serde(default)]
    pub data: Vec<MsgData>,
    /// 跳转小程序类型：developer 开发版 / trial 体验版 / formal 正式版（默认）。
    #[serde(rename = "miniprogram_state", skip_serializing_if = "Option::is_none")]
    pub miniprogram_state: Option<String>,
    /// 进入小程序查看的语言类型，默认 `zh_CN`。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
}

impl Default for WxMaSubscribeMessage {
    /// Java 字段默认值：`miniprogramState = "formal"`、`lang = "zh_CN"`。
    fn default() -> Self {
        Self {
            to_user: None,
            template_id: None,
            page: None,
            data: Vec::new(),
            miniprogram_state: Some("formal".to_string()),
            lang: Some("zh_CN".to_string()),
        }
    }
}

impl WxMaSubscribeMessage {
    /// 新建订阅消息（带 Java 默认字段值）。
    pub fn new() -> Self {
        Self::default()
    }

    /// 追加模板数据（对应 Java `addData`：先按 `resetValue` 清洗再追加）。
    pub fn add_data(&mut self, datum: MsgData) -> &mut Self {
        self.data.push(reset_value(datum));
        self
    }

    /// 序列化为 JSON（对应 Java `WxMaSubscribeMessageGsonAdapter` 线格式）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("订阅消息序列化失败: {e}"))
    }
}

impl Serialize for WxMaSubscribeMessage {
    /// 对应 Java `WxMaSubscribeMessageGsonAdapter.serialize`：
    /// `touser`/`template_id` 必填；`page`/`miniprogram_state`/`lang` 非空输出；
    /// `data` 恒输出（`{名称: {"value": ...}}`）。
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serde_json::Map::new();
        if let Some(v) = &self.to_user {
            map.insert("touser".into(), serde_json::json!(v));
        }
        if let Some(v) = &self.template_id {
            map.insert("template_id".into(), serde_json::json!(v));
        }
        if let Some(v) = &self.page {
            map.insert("page".into(), serde_json::json!(v));
        }
        if let Some(v) = &self.miniprogram_state {
            map.insert("miniprogram_state".into(), serde_json::json!(v));
        }
        if let Some(v) = &self.lang {
            map.insert("lang".into(), serde_json::json!(v));
        }
        let mut data = serde_json::Map::new();
        for d in &self.data {
            let mut data_json = serde_json::Map::new();
            data_json.insert("value".into(), serde_json::json!(&d.value));
            data.insert(d.name.clone(), serde_json::Value::Object(data_json));
        }
        map.insert("data".into(), serde_json::Value::Object(data));
        serde_json::Value::Object(map).serialize(serializer)
    }
}

/// 是否 CJK 汉字（Java `\u4e00-\u9fa5`）。
fn is_cjk(c: char) -> bool {
    ('\u{4e00}'..='\u{9fa5}').contains(&c)
}

/// 保留满足条件的字符（等价 Java `Pattern.matcher(value).replaceAll("")`）。
fn keep_chars(s: &str, keep: impl Fn(char) -> bool) -> String {
    s.chars().filter(|c| keep(*c)).collect()
}

/// 处理订阅消息字符串长度及格式问题（对应 Java `resetValue`）。
fn reset_value(datum: MsgData) -> MsgData {
    let name = datum.name.clone();
    let value = datum.value.clone();

    if value.trim().is_empty() {
        // 空值会发送失败，改为 -
        return MsgData {
            name,
            value: "-".to_string(),
        };
    }

    let mut value = value;
    if name.starts_with("thing") && value.chars().count() > 20 {
        // thing.DATA: 20 个以内字符，可汉字、数字、字母或符号组合
        value = truncate(&value, 17) + "...";
    } else if name.starts_with("number") {
        // number.DATA: 32 位以内数字，只能数字，可带小数
        value = keep_chars(&value, |c| c.is_ascii_digit() || c == '.' || c == '-');
        if !is_number_valid(&value) {
            value = "0".to_string();
        }
        if value.chars().count() > 32 {
            value = truncate(&value, 32);
        }
    } else if name.starts_with("letter") {
        // letter.DATA: 32 位以内字母，只能字母
        value = keep_chars(&value, |c| c.is_ascii_alphabetic());
        if value.is_empty() {
            value = "A".to_string();
        }
        if value.chars().count() > 32 {
            value = truncate(&value, 32);
        }
    } else if name.starts_with("symbol") {
        // symbol.DATA: 5 位以内符号，只能符号（除中文、英文、数字外的常见符号）
        value = keep_chars(&value, |c| !(c.is_ascii_alphanumeric() || is_cjk(c)));
        if value.is_empty() {
            value = "-".to_string();
        }
        if value.chars().count() > 5 {
            value = truncate(&value, 5);
        }
    } else if name.starts_with("character_string") {
        // character_string.DATA: 32 位以内，可数字、字母或符号组合（不含中文）
        value = keep_chars(&value, |c| !is_cjk(c));
        if value.is_empty() {
            value = "0".to_string();
        }
        if value.chars().count() > 32 {
            value = truncate(&value, 32);
        }
    } else if name.starts_with("phone_number") {
        // phone_number.DATA: 17 位以内，数字、符号
        value = keep_chars(&value, |c| c.is_ascii_digit() || c == '+' || c == '-');
        // 只允许一个前导 + 号，且必须在开头
        if value.starts_with('+') {
            value = format!("+{}", value[1..].replace('+', ""));
        } else {
            value = value.replace('+', "");
        }
        if value.is_empty() {
            value = "0".to_string();
        }
        if value.chars().count() > 17 {
            value = truncate(&value, 17);
        }
    } else if name.starts_with("car_number") {
        // car_number.DATA: 8 位以内，第一位与最后一位可为汉字，其余为字母或数字
        if value.chars().count() > 8 {
            value = truncate(&value, 8);
        }
    } else if name.starts_with("name") {
        // name.DATA: 10 个以内纯汉字或 20 个以内纯字母或符号，中文和字母混合按中文名算 10 个字内
        // 过滤非法字符，不保留数字（name 类型不允许数字）
        value = keep_chars(&value, |c| {
            is_cjk(c)
                || c.is_ascii_alphabetic()
                || c == ' '
                || c == '\u{b7}'
                || c == '\u{3001}'
                || c == '\u{ff0c}'
                || c == '\u{3002}'
                || c == '-'
        });
        if value.is_empty() {
            value = "-".to_string();
        }
        let contains_chinese = value.chars().any(is_cjk);
        if contains_chinese {
            // 含中文，按中文名算，10 个字内
            if value.chars().count() > 10 {
                value = truncate(&value, 7) + "...";
            }
        } else {
            // 纯字母或符号，20 个以内
            if value.chars().count() > 20 {
                value = truncate(&value, 17) + "...";
            }
        }
    } else if name.starts_with("phrase") {
        // phrase.DATA: 5 个以内纯汉字
        value = keep_chars(&value, is_cjk);
        if value.is_empty() {
            value = "好".to_string();
        }
        if value.chars().count() > 5 {
            value = truncate(&value, 5);
        }
    }

    MsgData { name, value }
}

/// 截取前 n 个字符（对应 Java `StringUtils.substring`，按字符计数）。
fn truncate(s: &str, n: usize) -> String {
    s.chars().take(n).collect()
}

/// 数值合法性（Java `NUMBER_VALID_PATTERN: -?\d+\.?\d*|-?\.\d+`）。
fn is_number_valid(s: &str) -> bool {
    let rest = s.strip_prefix('-').unwrap_or(s);
    if rest.starts_with('.') {
        // 情形 B：`-?\.\d+`（如 ".5"）
        let after_dot: String = rest.chars().skip(1).collect();
        return !after_dot.is_empty() && after_dot.chars().all(|c| c.is_ascii_digit());
    }
    // 情形 A：`\d+\.?\d*`（整数或带前导数字的小数）
    let mut saw_digit = false;
    let mut saw_dot = false;
    for c in rest.chars() {
        if c.is_ascii_digit() {
            saw_digit = true;
        } else if c == '.' && !saw_dot {
            saw_dot = true;
        } else {
            return false;
        }
    }
    saw_digit
}
