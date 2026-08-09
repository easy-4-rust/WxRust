#!/usr/bin/env python3
"""weixin-java-mp 数据 bean 批量生成器。

遍历 mp bean 子目录（card/guide/marketing/draft/freepublish/datacube/device/
shake/wifi/comment/store/tag/kefu/material/result/subscribe/template 及根目录），
对纯数据类（@Data 或仅字段的类）生成 Rust struct + serde 派生，写入
crates/wx-rust-mp/src/bean/<dir>/。

排除：含枚举常量/复杂逻辑/继承（extends）/@JsonIgnore 深层逻辑的类由人工迁移。
Date -> String（Java Gson 输出 ISO 8601 字符串，线格式原样保留）。
"""
import os
import re
import sys

BASE = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
JAVA_BEAN = "/Users/wandl/workspaces/workspace-github/WxJava/weixin-java-mp/src/main/java/me/chanjar/weixin/mp/bean"
RUST_BEAN = os.path.join(BASE, "crates", "wx-rust-mp", "src", "bean")

FIELD_RE = re.compile(r"^\s*private\s+(?!static\s+final\s+)([\w<>\[\],. ]+?)\s+(\w+)\s*(?:=\s*[^;]*)?;")
CLASS_RE = re.compile(r"^\s*(?:public\s+|private\s+)?(?:abstract\s+|final\s+|static\s+)*class\s+(\w+)")

TYPE_MAP = {
    "String": "String",
    "int": "i32", "Integer": "i32",
    "long": "i64", "Long": "i64",
    "boolean": "bool", "Boolean": "bool",
    "double": "f64", "Double": "f64",
    "float": "f32", "Float": "f32",
    "short": "i16", "Short": "i16",
    "byte": "u8", "Byte": "u8",
    "char": "char", "Character": "char",
    "Object": "serde_json::Value",
    "Date": "String",  # Java Gson 输出 ISO 8601 字符串
    "LocalDate": "String",
    "LocalDateTime": "String",
    "BigDecimal": "String",  # 金额精度保留原串
    "BigInteger": "String",
    "Long[]": "Vec<i64>",
    "String[]": "Vec<String>",
    "Integer[]": "Vec<i32>",
    "Map<String, Object>": "std::collections::HashMap<String, serde_json::Value>",
    "Map<String, String>": "std::collections::HashMap<String, String>",
    "Map<String, Integer>": "std::collections::HashMap<String, i32>",
    "Map<Integer, String>": "std::collections::HashMap<i32, String>",
    "JsonArray": "serde_json::Value",
    "JsonObject": "serde_json::Value",
    "Integer[]": "Vec<i32>",
}


def map_type(t: str, classes: set) -> str:
    t = t.strip()
    if t.startswith("final "):
        t = t[len("final "):]
    m = re.match(r"(?:List|Collection)<(.+)>", t)
    if m:
        return f"Vec<{map_type(m.group(1), classes)}>"
    m = re.match(r"Map<([^,]+),\s*(.+)>", t)
    if m:
        k = map_type(m.group(1), classes)
        v = map_type(m.group(2), classes)
        return f"std::collections::HashMap<{k}, {v}>"
    if t.endswith("[]"):
        return f"Vec<{map_type(t[:-2], classes)}>"
    if t in TYPE_MAP:
        return TYPE_MAP[t]
    return t  # 类名引用（同文件嵌套或模块内）


def snake(name: str) -> str:
    s = re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", name)
    s = re.sub(r"([A-Z]+)([A-Z][a-z])", r"\1_\2", s)
    return s.lower()


def parse_java(path):
    """返回 (order, classes)：classes[name] = [(type, name, serialized)]"""
    classes = {}
    order = []
    cur = None
    pending_serialized = None
    skip_file = False
    parents = {}
    with open(path, encoding="utf-8") as f:
        for line in f:
            em = re.search(r"class\s+(\w+)\s+extends\s+([\w<>, ]+?)(?:\s*\{|\s*implements)", line)
            if em and "Serializable" not in em.group(2):
                parents[cur if cur else ""] = em.group(2).strip() if cur else None
                # 记录当前文件父类（可能后续 class 声明才出现，缓存由 main 二次解析）
            cm = CLASS_RE.match(line)
            if cm:
                cur = cm.group(1)
                if cur not in classes:
                    classes[cur] = []
                    order.append(cur)
                continue
            if cur is None:
                continue
            if re.search(r'@JsonIgnore', line):
                pending_serialized = "!skip"
                continue
            if "transient" in line:
                pending_serialized = "!skip"
                continue
            sm = re.search(r'@SerializedName\((?:value\s*=\s*)?"([^"]+)"', line)
            if sm:
                pending_serialized = sm.group(1)
                alt = re.search(r'alternate\s*=\s*"([^"]+)"', line)
                if alt:
                    pending_serialized = pending_serialized + "|" + alt.group(1)
                continue
            fm = FIELD_RE.match(line)
            if fm:
                ftype, fname = fm.group(1).strip(), fm.group(2)
                if pending_serialized == "!skip":
                    pending_serialized = None
                else:
                    classes[cur].append((ftype, fname, pending_serialized))
                    pending_serialized = None
    return order, classes, skip_file, parents


def rust_ident(name: str) -> str:
    keywords = {
        "type", "match", "move", "ref", "loop", "self", "super", "crate",
        "fn", "impl", "let", "mut", "pub", "struct", "enum", "trait",
        "where", "use", "as", "in", "for", "if", "else", "while", "return",
        "break", "continue", "true", "false", "static", "const", "mod",
        "async", "await", "dyn", "box",
    }
    return f"r#{name}" if name in keywords else name


def gen_struct(name, fields, classes, indent=""):
    lines = [f"{indent}#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]"]
    lines.append(f"{indent}pub struct {name} {{")
    for ftype, fname, serialized in fields:
        rt = map_type(ftype, set(classes.keys()))
        json_name = serialized.split("|")[0] if serialized else fname
        if serialized and "|" in serialized:
            primary, alternate = serialized.split("|", 1)
            lines.append(f"{indent}    #[serde(rename = \"{primary}\", alias = \"{alternate}\", default)]")
        else:
            lines.append(f"{indent}    #[serde(rename = \"{json_name}\", default)]")
        lines.append(f"{indent}    pub {rust_ident(snake(fname))}: {rt},")
    lines.append(f"{indent}}}")
    return "\n".join(lines)


def flatten_fields(name, classes, parents, cache, seen=None):
    """递归合并父类字段（父类在前，对应 Gson 反射序列化层级顺序）。"""
    seen = seen or set()
    if name in seen:
        return []
    seen.add(name)
    own = classes.get(name, [])
    parent = parents.get(name)
    if not parent or parent in ("Object", "Serializable"):
        return list(own)
    pfields = flatten_fields(parent, cache, seen)
    return pfields + list(own)


def gen_file(java_path, rust_path, cache=None):
    """为单个 Java 文件生成 Rust 文件；纯数据类才生成。"""
    order, classes, skip, parents = parse_java(java_path)
    if skip or not order:
        return None
    top = order[0]
    src = open(java_path, encoding="utf-8").read()
    if "enum " in src and "public enum" in src:
        return None  # 枚举人工迁移
    if cache is None:
        cache = {top: {"fields": classes.get(top, []), "parents": parents}}
    top_fields = flatten_fields(top, classes, parents, cache)
    body = gen_struct(top, top_fields, classes)
    for inner in order[1:]:
        inner_fields = flatten_fields(inner, classes, parents, cache)
        body += "\n\n" + gen_struct(inner, inner_fields, classes)
    structs = re.findall(r"pub struct (\w+)", body)
    return (f"""//! 对应 Java `{java_path.split('weixin/mp/')[-1].replace('/', '.')[:-5]}`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

{body}
""", structs)


# 手写版本（Gson adapter 线格式权威，生成器禁止覆盖）
HAND_WRITTEN = {
    'WxMpTemplateMessage.java', 'WxMpTemplateData.java', 'WxMpTemplateMessage$MiniProgram.java',
    'WxMpTemplateIndustryEnum.java', 'WxMpTemplateIndustry.java',
    'WxMpKefuMessage.java', 'WxMpKefuMessage$WxArticle.java', 'WxMpKefuMessage$MsgMenu.java',
    'WxMpUser.java', 'WxMpQrCodeTicket.java', 'WxMpCurrentAutoReplyInfo.java',
    'WxMpShortKeyResult.java', 'WxMpMenu.java', 'WxMpMenu$WxMpConditionalMenu.java',
    'WxMpSubscribeMessage.java', 'WxMpSubscribeMessage$MiniProgram.java',
    'WxMpUserBlacklistGetResult.java', 'WxMpMassSendResult.java', 'WxMpMassUploadResult.java',
}


SUBDIRS = {
    "card": ["membercard", "enums"],
    "invoice": ["merchant", "reimburse"],
    "kefu": ["request", "result"],
}

# 子目录 mod.rs 的已知命名冲突修复（Java 同名内部类跨文件）
SUBDIR_MOD_FIXES = {
    "invoice/merchant/mod.rs": [
        ("pub use invoice_auth_data_result::UserField;", "pub use invoice_auth_data_result::UserField as AuthDataUserField;"),
        ("pub use invoice_auth_data_result::BizField;", "pub use invoice_auth_data_result::BizField as AuthDataBizField;"),
    ],
}

def gen_subdirs(parent, sub, hand_written):
    """生成子目录 bean（父目录/子目录结构）。"""
    jdir = os.path.join(JAVA_BEAN, parent, sub)
    rdir = os.path.join(RUST_BEAN, parent, sub)
    if not os.path.isdir(jdir):
        return [], []
    os.makedirs(rdir, exist_ok=True)
    mod_lines, mod_uses, generated, skipped = [], [], [], []
    cache = {}
    for fn in sorted(os.listdir(jdir)):
        if not fn.endswith(".java"):
            continue
        o, c, s, par = parse_java(os.path.join(jdir, fn))
        if not s and o:
            cache[o[0]] = {"fields": c.get(o[0], []), "parents": par}
    for fn in sorted(os.listdir(jdir)):
        if not fn.endswith(".java"):
            continue
        if fn in hand_written:
            skipped.append(f"{parent}/{sub}/{fn}")
            continue
        content = gen_file(os.path.join(jdir, fn), None, cache)
        if content is None:
            skipped.append(f"{parent}/{sub}/{fn}")
            continue
        content, structs = content
        # 子目录文件引用父包类型：追加父包通配导入（如 card/membercard 引用 card 根类型）
        enum_import = f"\nuse crate::bean::{parent}::enums::*;" if os.path.isdir(os.path.join(JAVA_BEAN, parent, "enums")) else ""
        content = content.replace("use super::*;", f"use super::*;\nuse crate::bean::{parent}::*;{enum_import}")
        rust_name = snake(fn[:-5]) + ".rs"
        with open(os.path.join(rdir, rust_name), "w", encoding="utf-8") as f:
            f.write(content)
        post_process(rdir, rust_name)
        mod_name = rust_name[:-3]
        mod_lines.append(f"pub mod {mod_name};")
        for s in structs:
            mod_uses.append(f"pub use {mod_name}::{s};")
        generated.append(f"{parent}/{sub}/{rust_name}")
    if mod_lines:
        header = f"//! 对应 Java `me.chanjar.weixin.mp.bean.{parent}.{sub}` 包（生成）。\n"
        with open(os.path.join(rdir, "mod.rs"), "w", encoding="utf-8") as f:
            f.write(header + "\n" + "\n".join(mod_lines) + "\n\n" + "\n".join(mod_uses) + "\n")
    # 命名冲突修复
    fix_key = f"{parent}/{sub}/mod.rs"
    for old, new in SUBDIR_MOD_FIXES.get(fix_key, []):
        mp = os.path.join(rdir, "mod.rs")
        if os.path.isfile(mp):
            msrc = open(mp, encoding="utf-8").read()
            if old in msrc:
                open(mp, "w", encoding="utf-8").write(msrc.replace(old, new))
    return generated, skipped



# 生成后追加的辅助 impl（from_json/to_json 语义，幂等：已存在则跳过）
POST_PROCESS = {
    "tag/wx_user_tag.rs": "tag_user_tag",
    "tag/wx_tag_list_user.rs": "from_json",
    "store/wx_mp_store_base_info.rs": "store_base_info",
    "store/wx_mp_store_list_result.rs": "from_json",
    "comment/wx_mp_comment_list_vo.rs": "from_json",
    "wifi/wx_mp_wifi_shop_list_result.rs": "from_json",
    "wifi/wx_mp_wifi_shop_data_result.rs": "from_json",
    "draft/wx_mp_draft_info.rs": "from_json",
    "draft/wx_mp_draft_list.rs": "from_json",
    "freepublish/wx_mp_free_publish_status.rs": "from_json",
    "freepublish/wx_mp_free_publish_list.rs": "from_json",
    "freepublish/wx_mp_free_publish_info.rs": "from_json",
    "device/trans_msg_resp.rs": "from_json",
    "device/wx_device_qr_code_result.rs": "from_json",
    "device/wx_device_authorize_result.rs": "from_json",
    "device/wx_device_bind_result.rs": "from_json",
    "device/wx_device_open_id_result.rs": "from_json",
    "device/wx_device_bind_device_result.rs": "from_json",
    "result/wx_mp_mass_get_result.rs": "from_json",
    "result/wx_mp_mass_speed_get_result.rs": "from_json",
    "datacube/wx_data_cube_user_summary.rs": "datacube_summary",
    "datacube/wx_data_cube_user_cumulate.rs": "datacube_cumulate",
    "datacube/wx_data_cube_article_total.rs": "from_json_list",
    "datacube/wx_data_cube_article_result.rs": "from_json_list",
    "wx_mp_mass_news.rs": "mass_news_import",
    "invoice/merchant/invoice_auth_page_setting.rs": "kvpair",
    "material/wx_mp_material.rs": "material_file",
    "material/wx_media_img_upload_result.rs": "from_json",
    "material/wx_mp_material_upload_result.rs": "from_json",
    "material/wx_mp_material_video_info_result.rs": "from_json",
    "material/wx_mp_material_news.rs": "from_json",
    "material/wx_mp_material_count_result.rs": "from_json",
    "material/wx_mp_material_news_batch_get_result.rs": "from_json",
    "material/wx_mp_material_file_batch_get_result.rs": "from_json",
    "guide/wx_mp_guide_list.rs": "from_json",
    "guide/wx_mp_guide_acct_config.rs": "from_json",
    "guide/wx_mp_guide_img_material_info_list.rs": "from_json",
    "guide/wx_mp_guide_word_material_info_list.rs": "from_json",
    "guide/wx_mp_guide_massed_info.rs": "from_json",
    "card/wx_mp_card_result.rs": "from_json",
    "card/wx_mp_card_create_result.rs": "from_json",
    "card/wx_mp_card_landing_page_create_result.rs": "from_json",
    "card/wx_mp_card_qrcode_create_result.rs": "from_json",
    "card/wx_mp_card_delete_result.rs": "from_json",
    "card/card_update_result.rs": "from_json",
    "card/membercard/member_card_activate_user_form_result.rs": "from_json",
    "card/membercard/wx_mp_member_card_user_info_result.rs": "from_json",
    "card/membercard/wx_mp_member_card_update_result.rs": "from_json",
    "card/membercard/wx_mp_member_card_activate_temp_info_result.rs": "from_json",
    "invoice/reimburse/invoice_info_response.rs": "from_json",
    "invoice/merchant/invoice_auth_page_result.rs": "from_json",
    "invoice/merchant/invoice_auth_data_result.rs": "from_json",
    "invoice/merchant/invoice_result.rs": "from_json",
    "marketing/wx_mp_ad_lead_result.rs": "from_json",
    "shake/wx_mp_shake_around_page_add_result.rs": "from_json",
    "shake/wx_mp_shake_around_relation_search_result.rs": "from_json",
    "wx_mp_shake_info_result.rs": "from_json",
    "kefu/result/wx_mp_kf_list.rs": "from_json",
    "kefu/result/wx_mp_kf_online_list.rs": "from_json",
    "kefu/result/wx_mp_kf_session_get_result.rs": "from_json",
    "kefu/result/wx_mp_kf_session_list.rs": "from_json",
    "kefu/result/wx_mp_kf_session_wait_case_list.rs": "from_json",
    "kefu/result/wx_mp_kf_msg_list.rs": "from_json",
}


def from_json_block(struct: str) -> str:
    return (
        "impl " + struct + " {\n"
        "    /// 从 JSON 构建（对应 Java `fromJson`）。\n"
        "    pub fn from_json(json: &str) -> Result<Self, String> {\n"
        "        serde_json::from_str(json).map_err(|e| format!(\"" + struct + " 解析失败: {e}\"))\n"
        "    }\n"
        "}\n"
    )


def from_json_list_block(struct: str) -> str:
    return (
        "impl " + struct + " {\n"
        "    /// 从 JSON 构建列表（对应 Java `fromJson`：取 `list` 数组）。\n"
        "    pub fn from_json_list(json: &str) -> Result<Vec<Self>, String> {\n"
        "        let value: serde_json::Value =\n"
        "            serde_json::from_str(json).map_err(|e| format!(\"" + struct + " 列表解析失败: {e}\"))?;\n"
        "        let list = value.get(\"list\").ok_or_else(|| \"缺少 list 字段\".to_string())?;\n"
        "        serde_json::from_value(list.clone()).map_err(|e| format!(\"" + struct + " 列表解析失败: {e}\"))\n"
        "    }\n"
        "}\n"
    )


def store_base_info_block() -> str:
    return (
        "impl WxMpStoreBaseInfo {\n"
        "    /// 序列化为 JSON（对应 Java `toJson`：`{\"business\": {\"base_info\": {...}}}`）。\n"
        "    pub fn to_json(&self) -> String {\n"
        "        serde_json::json!({ \"business\": { \"base_info\": self } })\n"
        "            .to_string()\n"
        "    }\n"
        "}\n"
    )


def tag_user_tag_block() -> str:
    return (
        "impl WxUserTag {\n"
        "    /// 从 JSON 构建（对应 Java `fromJson`：取 `tag` 子对象）。\n"
        "    pub fn from_json(json: &str) -> Result<Self, String> {\n"
        "        let value: serde_json::Value =\n"
        "            serde_json::from_str(json).map_err(|e| format!(\"用户标签解析失败: {e}\"))?;\n"
        "        let tag = value.get(\"tag\").ok_or_else(|| \"缺少 tag 字段\".to_string())?;\n"
        "        serde_json::from_value(tag.clone()).map_err(|e| format!(\"用户标签解析失败: {e}\"))\n"
        "    }\n"
        "\n"
        "    /// 从 JSON 构建标签列表（对应 Java `listFromJson`：取 `tags` 数组）。\n"
        "    pub fn list_from_json(json: &str) -> Result<Vec<Self>, String> {\n"
        "        let value: serde_json::Value =\n"
        "            serde_json::from_str(json).map_err(|e| format!(\"用户标签列表解析失败: {e}\"))?;\n"
        "        let tags = value\n"
        "            .get(\"tags\")\n"
        "            .ok_or_else(|| \"缺少 tags 字段\".to_string())?;\n"
        "        serde_json::from_value(tags.clone()).map_err(|e| format!(\"用户标签列表解析失败: {e}\"))\n"
        "    }\n"
        "}\n"
    )


def post_process(rdir: str, rust_name: str):
    """追加 POST_PROCESS 中登记的辅助 impl（已存在则跳过）。"""
    rel = os.path.relpath(os.path.join(rdir, rust_name), RUST_BEAN)
    spec = POST_PROCESS.get(rel) or POST_PROCESS.get(rust_name)
    if spec is None:
        return
    p = os.path.join(rdir, rust_name)
    cur = open(p, encoding="utf-8").read()
    if spec in ("datacube_summary", "datacube_cumulate", "material_file"):
        pass  # 替换型 spec 不受早退守卫约束
    elif "from_json" in cur or "to_json" in cur:
        return
    if spec == "from_json":
        m = re.search(r"pub struct (\w+)", cur)
        block = from_json_block(m.group(1)) if m else None
    elif spec == "from_json_list":
        m = re.search(r"pub struct (\w+)", cur)
        block = from_json_list_block(m.group(1)) if m else None
    elif spec == "store_base_info":
        block = store_base_info_block()
    elif spec == "tag_user_tag":
        block = tag_user_tag_block()
    elif spec == "mass_news_import":
        block = "use crate::bean::material::WxMpNewsArticle;"
    elif spec == "datacube_summary":
        # adapter 线格式：ref_date/user_source/new_user/cancel_user（Java 无 @SerializedName，由 WxMpUserSummaryGsonAdapter 映射）
        cur2 = cur.replace('#[serde(rename = "refDate", default)]', '#[serde(rename = "ref_date", default)]')
        cur2 = cur2.replace('#[serde(rename = "userSource", default)]', '#[serde(rename = "user_source", default)]')
        cur2 = cur2.replace('#[serde(rename = "newUser", default)]', '#[serde(rename = "new_user", default)]')
        cur2 = cur2.replace('#[serde(rename = "cancelUser", default)]', '#[serde(rename = "cancel_user", default)]')
        if cur2 != cur:
            open(p, "w", encoding="utf-8").write(cur2)
            cur = cur2  # 后续追加基于改名后的内容
        m = re.search(r"pub struct (\w+)", cur)
        block = from_json_list_block(m.group(1)) if m else None
    elif spec == "datacube_cumulate":
        cur2 = cur.replace('#[serde(rename = "refDate", default)]', '#[serde(rename = "ref_date", default)]')
        cur2 = cur2.replace('#[serde(rename = "cumulateUser", default)]', '#[serde(rename = "cumulate_user", default)]')
        if cur2 != cur:
            open(p, "w", encoding="utf-8").write(cur2)
            cur = cur2  # 后续追加基于改名后的内容
        m = re.search(r"pub struct (\w+)", cur)
        block = from_json_list_block(m.group(1)) if m else None
    elif spec == "kvpair":
        cur2 = cur.replace("InvoiceAuthDataResult.KeyValuePair", "KeyValuePair")
        if cur2 != cur:
            open(p, "w", encoding="utf-8").write(cur2)
        return
    elif spec == "material_file":
        # 直接替换 File 字段为路径字符串字段
        cur2 = cur.replace(
            '    #[serde(rename = "file", default)]\n    pub file: File,',
            '    /// 文件路径（对应 Java `File`，Rust 中以本地路径承载；不参与 JSON 序列化）。\n    #[serde(skip)]\n    pub file: Option<String>,')
        if cur2 != cur:
            open(p, "w", encoding="utf-8").write(cur2)
        return
    else:
        block = None
    if block:
        open(p, "w", encoding="utf-8").write(cur.rstrip() + "\n\n" + block + "\n")


def main():
    dirs = ["card", "guide", "marketing", "draft", "freepublish", "datacube",
            "device", "shake", "wifi", "comment", "store", "tag", "kefu",
            "material", "result", "subscribe", "template", "invoice"]
    all_generated = []
    all_skipped = []
    generated = []
    skipped = []
    for d in dirs:
        jdir = os.path.join(JAVA_BEAN, d)
        rdir = os.path.join(RUST_BEAN, d)
        if not os.path.isdir(jdir):
            continue
        os.makedirs(rdir, exist_ok=True)
        mod_lines = []
        mod_uses = []
        hand_written_dir = any(
            fn in HAND_WRITTEN for fn in os.listdir(jdir) if fn.endswith(".java")
        )
        cache = {}
        for fn in sorted(os.listdir(jdir)):
            if not fn.endswith(".java"):
                continue
            o, c, s, par = parse_java(os.path.join(jdir, fn))
            if not s and o:
                cache[o[0]] = {"fields": c.get(o[0], []), "parents": par}
        for fn in sorted(os.listdir(jdir)):
            if not fn.endswith(".java"):
                continue
            if fn in HAND_WRITTEN:
                skipped.append(f"{d}/{fn}")
                continue
            content = gen_file(os.path.join(jdir, fn), None, cache)
            if content is None:
                skipped.append(f"{d}/{fn}")
                continue
            content, structs = content
            rust_name = snake(fn[:-5]) + ".rs"
            with open(os.path.join(rdir, rust_name), "w", encoding="utf-8") as f:
                f.write(content)
            post_process(rdir, rust_name)
            mod_name = rust_name[:-3]
            if mod_name in ("abstract", "final"):
                mod_name = f"r#{mod_name}"
            mod_lines.append(f"pub mod {mod_name};")
            if not hand_written_dir:
                for s in structs:
                    mod_uses.append(f"pub use {mod_name}::{s};")
            generated.append(f"{d}/{rust_name}")
        if mod_lines and not hand_written_dir:
            header = "//! 对应 Java `me.chanjar.weixin.mp.bean.%s` 包（生成）。\n" % d
            with open(os.path.join(rdir, "mod.rs"), "w", encoding="utf-8") as f:
                f.write(header + "\n" + "\n".join(mod_lines) + "\n\n" + "\n".join(mod_uses) + "\n")
    # 根目录文件
    rdir_root = RUST_BEAN
    mod_root = []
    for fn in sorted(os.listdir(JAVA_BEAN)):
        if not fn.endswith(".java"):
            continue
        content = gen_file(os.path.join(JAVA_BEAN, fn), None)
        if content is None:
            skipped.append(fn)
            continue
        content, _ = content
        rust_name = snake(fn[:-5]) + ".rs"
        with open(os.path.join(rdir_root, rust_name), "w", encoding="utf-8") as f:
            f.write(content)
        post_process(rdir_root, rust_name)
        mod_root.append(f"pub mod {rust_name[:-3]};")
        generated.append(rust_name)
    for parent, subs in SUBDIRS.items():
        for sub in subs:
            gen, skip = gen_subdirs(parent, sub, set())
            all_generated.extend(gen)
            all_skipped.extend(skip)
        # 父 mod.rs 声明子目录
        pmod = os.path.join(RUST_BEAN, parent, "mod.rs")
        if os.path.isfile(pmod):
            pmod_src = open(pmod).read()
            changed = False
            for sub in subs:
                if f"pub mod {sub};" not in pmod_src:
                    pmod_src = pmod_src.rstrip() + f"\npub mod {sub};\n"
                    changed = True
                if f"pub use {sub}::*;" not in pmod_src:
                    pmod_src = pmod_src.rstrip() + f"\npub use {sub}::*;\n"
                    changed = True
            if changed:
                open(pmod, "w").write(pmod_src)
    generated = []
    skipped = []

    generated.extend(all_generated)
    skipped.extend(all_skipped)
    print(f"generated: {len(generated)}")
    print(f"skipped: {len(skipped)}")
    for s in skipped:
        print("  skip:", s)


if __name__ == "__main__":
    main()
