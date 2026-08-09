#!/usr/bin/env python3
"""数据 bean 生成器：从 Java 纯数据类（@Data 类）自动生成 Rust struct。

用于 weixin-java-common 的纯数据包：bean/imgproc、bean/menu、bean/oauth2、
bean/ocr、bean/result、bean/subscribemsg 及 WxNetCheckResult/WxOAuth2UserInfo。

解析：顶层类的 private 字段（name type），含内部类（缩进更深）→ 生成嵌套 struct。

类型映射：
  String -> String（可空语义由调用方用 Option 包裹，这里保持 Java 默认非空）
  int/long -> i32/i64
  Integer -> i32
  boolean -> bool
  List<X> -> Vec<X>
  X[] -> Vec<X>
  其它类名 -> 对应 Rust 类型（同文件内嵌套或 crate 内引用）
"""
import os
import re
import sys

BASE = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
JAVA_BEAN = "/Users/wandl/workspaces/workspace-github/WxJava/weixin-java-common/src/main/java/me/chanjar/weixin/common/bean"
RUST_BEAN = os.path.join(BASE, "crates", "wx-rust-common", "src", "bean")

# 支持字段初始化值（= ...;），排除 static final（serialVersionUID 等）
FIELD_RE = re.compile(r"^\s*private\s+(?!static\s+final\s+)([\w<>\[\],. ]+?)\s+(\w+)\s*(?:=\s*[^;]*)?;")
CLASS_RE = re.compile(r"^\s*(?:public\s+)?(?:static\s+)*class\s+(\w+)")

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
}


def map_type(t: str, classes: set) -> str:
    t = t.strip()
    m = re.match(r"List<(.+)>", t)
    if m:
        inner = m.group(1).strip()
        # 泛型嵌套
        if inner.startswith("List<"):
            return f"Vec<{map_type(inner, classes)}>"
        return f"Vec<{map_type(inner, classes)}>"
    m = re.match(r"Map<([^,]+),\s*(.+)>", t)
    if m:
        k = map_type(m.group(1), classes)
        v = map_type(m.group(2), classes)
        return f"std::collections::HashMap<{k}, {v}>"
    if t.endswith("[]"):
        return f"Vec<{map_type(t[:-2], classes)}>"
    if t in TYPE_MAP:
        return TYPE_MAP[t]
    # 类名 -> snake_case 引用（同文件嵌套或全局 bean 引用）
    return t


def snake(name: str) -> str:
    s = re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", name)
    s = re.sub(r"([A-Z]+)([A-Z][a-z])", r"\1_\2", s)
    return s.lower()


def parse_java(path):
    """返回 (顶层类名, [嵌套类定义])；嵌套定义: (class_name, [(field_type, field_name, serialized_name)])"""
    classes = {}  # name -> [(type, name, serialized_name)]
    order = []
    cur = None
    pending_serialized = None
    with open(path, encoding="utf-8") as f:
        for line in f:
            cm = CLASS_RE.match(line)
            if cm:
                cur = cm.group(1)
                if cur not in classes:
                    classes[cur] = []
                    order.append(cur)
                continue
            if cur is None:
                continue
            # @SerializedName("xxx") 或 @SerializedName(value = "xxx", alternate = "yyy")
            sm = re.search(r'@SerializedName\((?:value\s*=\s*)?"([^"]+)"', line)
            if sm:
                pending_serialized = sm.group(1)
                # 带 alternate 的字段追加别名（仅反序列化）
                alt = re.search(r'alternate\s*=\s*"([^"]+)"', line)
                if alt:
                    pending_serialized = pending_serialized + "|" + alt.group(1)
                continue
            fm = FIELD_RE.match(line)
            if fm:
                ftype, fname = fm.group(1).strip(), fm.group(2)
                classes[cur].append((ftype, fname, pending_serialized))
                pending_serialized = None
    return order, classes


def rust_ident(name: str) -> str:
    """Rust 字段标识符：保留字加 r# 前缀"""
    keywords = {
        "type", "match", "move", "ref", "loop", "self", "super", "crate",
        "fn", "impl", "let", "mut", "pub", "struct", "enum", "trait",
        "where", "use", "as", "in", "for", "if", "else", "while", "return",
        "break", "continue", "true", "false", "static", "const", "mod",
        "async", "await", "dyn", "box",
    }
    if name in keywords:
        return f"r#{name}"
    return name


def gen_struct(name, fields, classes, indent=""):
    lines = []
    lines.append(f"{indent}#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]")
    lines.append(f"{indent}pub struct {name} {{")
    for ftype, fname, serialized in fields:
        rt = map_type(ftype, set(classes.keys()))
        # serde 字段名：优先 @SerializedName 覆盖，否则 Java camelCase 保持原样
        json_name = serialized if serialized else fname
        lines.append(f"{indent}    /// {fname}")
        if serialized and "|" in serialized:
            primary, alternate = serialized.split("|", 1)
            lines.append(f"{indent}    #[serde(rename = \"{primary}\", alias = \"{alternate}\", default)]")
        else:
            lines.append(f"{indent}    #[serde(rename = \"{json_name}\", default)]")
        lines.append(f"{indent}    pub {rust_ident(snake(fname))}: {rt},")
    lines.append(f"{indent}}}")
    return "\n".join(lines)


def main():
    targets = {
        "imgproc": ["WxImgProcAiCropResult", "WxImgProcQrCodeResult", "WxImgProcSuperResolutionResult"],
        "menu": ["WxMenu", "WxMenuButton", "WxMenuRule"],
        "oauth2": ["WxOAuth2AccessToken"],
        "ocr": None,  # 通配：目录下全部
        "result": None,
        "subscribemsg": None,
    }
    root_files = ["WxNetCheckResult.java", "WxOAuth2UserInfo.java"]

    generated = 0
    # 根目录文件
    for fn in root_files:
        path = os.path.join(JAVA_BEAN, fn)
        order, classes = parse_java(path)
        if not order:
            print(f"[skip] {fn}: 无字段")
            continue
        top = order[0]
        body = gen_struct(top, classes[top], classes)
        for inner in order[1:]:
            body += "\n\n" + gen_struct(inner, classes[inner], classes)
        dst = os.path.join(RUST_BEAN, snake(top) + ".rs")
        with open(dst, "w", encoding="utf-8") as f:
            f.write(f"//! 对应 Java `me.chanjar.weixin.common.bean.{top}`（由 gen_bean_structs.py 生成）。\n\n")
            f.write(body + "\n")
        generated += 1
        print(f"[ok] {fn} -> {os.path.relpath(dst, BASE)}")

    # 子目录
    for sub, names in targets.items():
        if names is None:
            names = sorted(fn[:-5] for fn in os.listdir(os.path.join(JAVA_BEAN, sub)) if fn.endswith(".java"))
        for name in names:
            path = os.path.join(JAVA_BEAN, sub, name + ".java")
            if not os.path.exists(path):
                print(f"[skip] {path} 不存在")
                continue
            order, classes = parse_java(path)
            if not order:
                print(f"[skip] {name}: 无字段")
                continue
            top = order[0]
            body = gen_struct(top, classes[top], classes)
            for inner in order[1:]:
                body += "\n\n" + gen_struct(inner, classes[inner], classes)
            dst = os.path.join(RUST_BEAN, sub, snake(top) + ".rs")
            os.makedirs(os.path.dirname(dst), exist_ok=True)
            with open(dst, "w", encoding="utf-8") as f:
                f.write(f"//! 对应 Java `me.chanjar.weixin.common.bean.{sub}.{top}`（由 gen_bean_structs.py 生成）。\n\n")
                f.write(body + "\n")
            # 跨文件引用修复（menu/ocr/result 的嵌套类型在独立文件）
            refs = {
                "WxMenu": [("WxMenuButton", "super::wx_menu_button::WxMenuButton"),
                           ("WxMenuRule", "super::wx_menu_rule::WxMenuRule")],
                "WxOcrBizLicenseResult": [("WxOcrImgSize", "super::wx_ocr_img_size::WxOcrImgSize"),
                                          ("WxOcrPos", "super::wx_ocr_pos::WxOcrPos")],
                "WxOcrCommResult": [("WxOcrImgSize", "super::wx_ocr_img_size::WxOcrImgSize"),
                                    ("WxOcrPos", "super::wx_ocr_pos::WxOcrPos")],
                "WxOcrDrivingResult": [("WxOcrImgSize", "super::wx_ocr_img_size::WxOcrImgSize"),
                                       ("WxOcrPos", "super::wx_ocr_pos::WxOcrPos")],
                "WxMinishopImageUploadCustomizeResult": [
                    ("WxMinishopPicFileCustomizeResult", "super::wx_minishop_pic_file_customize_result::WxMinishopPicFileCustomizeResult")],
                "WxMinishopImageUploadResult": [
                    ("WxMinishopPicFileResult", "super::wx_minishop_pic_file_result::WxMinishopPicFileResult")],
            }
            if top in refs:
                path = os.path.join(RUST_BEAN, sub, snake(top) + ".rs")
                t2 = open(path).read()
                imports = "\n".join(f"use {full};" for _, full in refs[top])
                if "use super::" not in t2:
                    t2 = t2.replace(
                        "\n#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]",
                        f"\n{imports}\n\n#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]", 1)
                    open(path, 'w').write(t2)
            generated += 1
            print(f"[ok] {sub}/{name} -> {os.path.relpath(dst, BASE)}")
    print(f"合计生成 {generated} 个数据类文件")


if __name__ == "__main__":
    sys.exit(main())
