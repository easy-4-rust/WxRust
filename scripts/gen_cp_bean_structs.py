#!/usr/bin/env python3
"""weixin-java-cp 数据 bean 批量生成器。

由 `scripts/gen_miniapp_bean_structs.py` 复制改造而来，服务于
crates/wx-rust-cp（对应 Java `me.chanjar.weixin.cp.bean` 包）。

遍历 cp bean 目录树（article/corpgroup/export/external/hr/intelligentrobot/
kf/license/linkedcorp/living/media/msgaudit/oa/order/school/taskcard/
templatecard/user/workbench 及多级子目录如 external/contact、oa/applydata、
school/health 等，目录自动发现，不硬编码），对纯数据类（@Data 或仅字段的
类）生成 Rust struct + serde 派生，写入 crates/wx-rust-cp/src/bean/<dir>/。

保留 miniapp 生成器全部机制：
- 继承扁平化（flatten_fields，跨文件/跨包父类经全局 cache 递归合并，父类字段在前）
- 内嵌类（static class → 同文件多 struct）
- `use super::*;` + 祖先包通配导入（多级子目录逐层导入）
- 每目录 mod.rs（pub mod + pub use），根 bean/mod.rs 汇总声明
- HAND_WRITTEN 保护集（Gson/XStream adapter 线格式权威 / 消息类，禁止覆盖）
- POST_PROCESS（from_json / from_json_list / to_json 辅助，幂等）
- `#[allow(unused_imports)]`、`#[serde(rename = ...)]`、`alternate` → alias

cp 适配点（相对 miniapp）：
- 花括号深度跟踪：字段归属「最近一个未闭合的类」而非「最后声明的类」，
  修正 Java 常见写法「内嵌类声明之后的外层字段」（如 WxCpUserExternalContactInfo
  的 ExternalContact 字段在嵌套 MiniProgram 之后）；
- 作用域感知的嵌套类型解析：同名嵌套类（不同父作用域，如 WxCpCheckinDayData
  的 SpTitle.Data 与 SpDescription.Data）以「父名+类名」唯一键生成，字段引用
  沿声明类作用域链解析；跨文件引用（如 kf 引用 external.contact.ExternalContact）
  生成完整模块路径，消除 glob 导入歧义（E0659）；
- 枚举支持：Java 顶层枚举（Gender/WxCpSpStatus 等）与内嵌枚举（如
  WxCpContactWayInfo.TYPE/SCENE）均生成 serde 枚举；变体名取 @SerializedName
  值（缺省取常量名），与 Gson 对 @SerializedName 枚举常量的线格式一致；
- FQN 前缀剥离（java.util./java.lang./java.math./com.google.gson.）与
  JsonElement -> serde_json::Value；
- HAND_WRITTEN 增加：消息域 XML/JSON 复杂类（bean/message 子包，人工迁移
  至 src/bean/message/，生成器仅登记）与 Gson adapter 驱动类（WxCpUser/
  WxCpTag/WxCpDepart/WxCpChat/WxCpKfGetCorpStatisticResp/WxCpTagGetResult/
  WxCpTpTagGetResult，Wave 2 人工迁移）；messagebuilder/outxmlbuilder 整个
  子目录跳过（迁移至 src/message/）；
- NEW_IMPLS：Wave 0 门面（WxCpService）引用的 bean 追加 `new` 构造器
  （参数顺序与门面调用一致，门面签名无需改动）。

Date -> String（Java Gson 输出 ISO 8601 字符串，线格式原样保留）。
"""
import os
import re
import sys

BASE = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
JAVA_BEAN = "/Users/wandl/workspaces/workspace-github/WxJava/weixin-java-cp/src/main/java/me/chanjar/weixin/cp/bean"
RUST_BEAN = os.path.join(BASE, "crates", "wx-rust-cp", "src", "bean")
JAVA_PKG = "me.chanjar.weixin.cp.bean"

FIELD_RE = re.compile(r"^\s*(?:private|protected)\s+(?!static\s+final\s+)([\w<>\[\],. ]+?)\s+(\w+)\s*(?:=\s*[^;]*)?;")
# 类声明行：捕获类名、泛型参数与可选 extends 父类（同一行声明，如
# `public class X<T> extends Y implements Serializable {` / `public class X extends Y {`）
CLASS_RE = re.compile(
    r"^\s*(?:public\s+|private\s+)?(?:abstract\s+|final\s+|static\s+)*class\s+(\w+)"
    r"(?:<([^>]*)>)?(?:\s+extends\s+([\w<>,. ]+?))?(?:\s+implements\s+[\w<>,. ]+)?\s*\{"
)
# 枚举声明行（顶层无缩进 / 内嵌有缩进；允许 @Getter 等注解同行前缀）
ENUM_RE = re.compile(
    r"^\s*(?:@\w+\s+)*(?:public\s+)?(?:abstract\s+|final\s+)*enum\s+(\w+)"
    r"(?:\s+implements\s+[\w<>,. ]+)?\s*\{"
)
# 枚举常量：`NAME(args),` / `NAME,` / `NAME;`（允许缩进；排除方法调用 `Gender.values()` 等）
ENUM_CONST_RE = re.compile(r"^\s*([A-Z][A-Z0-9_]*)\s*(?:\([^)]*\))?\s*[,;]")
SERIALIZED_RE = re.compile(r'@SerializedName\((?:\s*value\s*=\s*)?"([^"]+)"')

# Java FQN 前缀剥离（java.util.List<X> 等）
FQN_PREFIXES = ("java.util.", "java.lang.", "java.math.", "com.google.gson.")

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
    "JsonElement": "serde_json::Value",
    "Long[]": "Vec<i64>",
    "String[]": "Vec<String>",
    "Integer[]": "Vec<i32>",
    "Map<String, Object>": "std::collections::HashMap<String, serde_json::Value>",
    "Map<String, String>": "std::collections::HashMap<String, String>",
    "Map<String, Integer>": "std::collections::HashMap<String, i32>",
    "Map<Integer, String>": "std::collections::HashMap<i32, String>",
    "Map<String, Long>": "std::collections::HashMap<String, i64>",
    "Map<Integer, Integer>": "std::collections::HashMap<i32, i32>",
    "JsonArray": "serde_json::Value",
    "JsonObject": "serde_json::Value",
}


def pascal_enum_name(name: str) -> str:
    """枚举类型名：Java ALL_CAPS -> Rust PascalCase（KEY -> Key、TYPE -> Type）。"""
    if name.isupper():
        if "_" in name:
            return "".join(p.capitalize() for p in name.split("_"))
        return name[:1] + name[1:].lower()
    return name


def enum_variant_name(name: str) -> str:
    """枚举常量名：JAVA_CONSTANT -> Rust PascalCase（ONE_SIGN -> OneSign）。"""
    return "".join(p.capitalize() for p in name.split("_"))


def snake(name: str) -> str:
    s = re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", name)
    s = re.sub(r"([A-Z]+)([A-Z][a-z])", r"\1_\2", s)
    return s.lower()


def rust_ident(name: str) -> str:
    keywords = {
        "type", "match", "move", "ref", "loop", "self", "super", "crate",
        "fn", "impl", "let", "mut", "pub", "struct", "enum", "trait",
        "where", "use", "as", "in", "for", "if", "else", "while", "return",
        "break", "continue", "true", "false", "static", "const", "mod",
        "async", "await", "dyn", "box",
    }
    return f"r#{name}" if name in keywords else name


def parse_java(path):
    """解析单个 Java 文件，返回结构化数据。

    返回 (order, classes, flags, type_params, enums, enum_order, top_enum,
    nested, top_level)。

    - classes：唯一键（Rust 类型名）-> {java, parent(唯一键或 None), fields}
      fields 为 [(ftype, fname, serialized, decl_key)]，decl_key 为字段声明
      所在类（作用域解析用；继承扁平化后字段挂子类下）。
    - order：[唯一键]，文件内声明顺序（首个为顶层类）
    - nested：唯一键 -> {java 简单名: 唯一键}（直接内嵌类索引，作用域解析）
    - top_level：java 简单名 -> 唯一键（文件顶层类，作用域解析兜底）
    - flags：dict(is_enum, has_from_json, has_to_json)
    - type_params：文件内声明的泛型类型参数集合
    - enums：java 名 -> [(variant, serialized)]（内嵌枚举）
    - enum_order：内嵌枚举 java 名列表
    - top_enum：顶层枚举名（整个文件仅一个枚举时非 None）
    """
    classes = {}
    order = []
    nested = {}
    top_level = {}
    parents_chain = {}  # 唯一键 -> 父类唯一键（仅文件内解析，跨文件父类另行解析）
    type_params = set()
    has_from_json = False
    has_to_json = False
    enums = {}
    enum_order = []
    top_enum = None

    stack = []  # [(唯一键, 花括号深度)]
    depth = 0
    cur_enum = None
    pending_serialized = None
    cur = None
    cur_parent_key = None  # 当前类声明时的父（用于 duplicate 命名）

    with open(path, encoding="utf-8") as f:
        for line in f:
            # ---- 枚举块内：收集常量 ----
            if cur_enum is not None:
                if line.strip() == "}" or line.strip() == "};" or line.strip() == ";":
                    cur_enum = None
                    depth -= 1  # 枚举体闭合
                    continue
                ec = ENUM_CONST_RE.match(line)
                if ec:
                    name = ec.group(1)
                    sm = SERIALIZED_RE.search(line)  # @SerializedName 可与常量同行
                    ser = sm.group(1) if sm else pending_serialized
                    enums[cur_enum].append((name, ser if ser else name))
                    pending_serialized = None
                    continue
                sm = SERIALIZED_RE.search(line)
                if sm:
                    pending_serialized = sm.group(1)
                    continue
                continue
            # ---- 枚举声明 ----
            em = ENUM_RE.match(line)
            if em:
                name = em.group(1)
                indent = line[: len(line) - len(line.lstrip())]
                if not indent:
                    top_enum = name
                elif name not in enums:
                    enums[name] = []
                    enum_order.append(name)
                enums.setdefault(name, [])
                cur_enum = name
                depth += 1  # 枚举体开启
                continue
            # ---- 类声明 ----
            cm = CLASS_RE.match(line)
            if cm:
                java_name = cm.group(1)
                parent_key = stack[-1][0] if stack else None
                # 唯一键：同名嵌套类（不同父作用域）以父链前缀区分；
                # Rust 预置/std 类型名冲突（如 Option）以「顶层类名+类名」重命名
                if java_name not in classes and java_name not in RUST_PRELUDE_BLACKLIST:
                    key = java_name
                else:
                    base = (parent_key or "") + java_name
                    key = base
                    i = 2
                    while key in classes:
                        key = f"{base}{i}"
                        i += 1
                if cm.group(2):
                    for p in re.split(r"\s*,\s*", cm.group(2)):
                        p = p.strip()
                        if p:
                            type_params.add(re.split(r"\s+", p)[0])
                parent = cm.group(3)
                if parent and "Serializable" not in parent and "Object" not in parent:
                    # 泛型父类（如 Base<T>）去掉类型参数
                    parent = re.split(r"[<,]", parent)[0].strip()
                    parents_chain[key] = parent  # java 简单名，跨文件经全局 cache 解析
                classes[key] = {
                    "java": java_name,
                    "parent_java": parents_chain.get(key),
                    "fields": [],
                }
                nested.setdefault(key, {})
                if parent_key is not None:
                    nested[parent_key][java_name] = key
                else:
                    top_level[java_name] = key
                order.append(key)
                depth += 1  # 类体开启（声明行上的 {）
                stack.append((key, depth))
                pending_serialized = None
                cur = key
                continue
            # ---- 花括号深度跟踪（方法体/代码块闭合后弹出类） ----
            opens = line.count("{")
            closes = line.count("}")
            depth += opens - closes
            while stack and depth < stack[-1][1]:
                stack.pop()
            cur = stack[-1][0] if stack else None
            if cur is None:
                continue
            if "public static" in line and "fromJson" in line:
                has_from_json = True
                continue
            if "public String" in line and "toJson" in line:
                has_to_json = True
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
                    classes[cur]["fields"].append(
                        (ftype, fname, pending_serialized, cur)
                    )
                    pending_serialized = None
    flags = {
        "is_enum": top_enum is not None,
        "has_from_json": has_from_json,
        "has_to_json": has_to_json,
    }
    return (
        order, classes, flags, type_params, enums, enum_order, top_enum,
        nested, top_level,
    )


def resolve_scope_type(java_name, decl_key, classes, nested, top_level):
    """在文件内解析字段类型：沿 decl_key 作用域链向上查找同名嵌套类。

    返回 Rust 类型键（唯一键）或 None。
    """
    cur = decl_key
    while cur is not None:
        if java_name in nested.get(cur, {}):
            return nested[cur][java_name]
        cur = classes[cur]["parent_java"] if cur in classes else None
        # 父类可能是文件内类（用其键）或跨文件类（其字段类型在解析时另行处理）
        if cur is not None and cur not in classes:
            break
    if java_name in top_level:
        return top_level[java_name]
    return None


def map_type(t, decl_key, classes, nested, top_level, type_params, enum_names, cur_file):
    """将 Java 字段类型映射为 Rust 类型。

    - 同文件内嵌类/枚举：解析为文件内唯一键（bare 名）；
    - 跨文件类引用：解析为完整模块路径（消除 glob 导入歧义）；
    - 内嵌枚举（TYPE/SCENE 等）：Pascal 化（Type/Scene）。
    """
    t = t.strip()
    if t.startswith("final "):
        t = t[len("final "):]
    for prefix in FQN_PREFIXES:
        if t.startswith(prefix):
            t = t[len(prefix):]
            break
    m = re.match(r"(?:List|Collection)<(.+)>", t)
    if m:
        return f"Vec<{map_type(m.group(1), decl_key, classes, nested, top_level, type_params, enum_names, cur_file)}>"
    m = re.match(r"Map<([^,]+),\s*(.+)>", t)
    if m:
        k = map_type(m.group(1), decl_key, classes, nested, top_level, type_params, enum_names, cur_file)
        v = map_type(m.group(2), decl_key, classes, nested, top_level, type_params, enum_names, cur_file)
        return f"std::collections::HashMap<{k}, {v}>"
    if t.endswith("[]"):
        return f"Vec<{map_type(t[:-2], decl_key, classes, nested, top_level, type_params, enum_names, cur_file)}>"
    if t in TYPE_MAP:
        return TYPE_MAP[t]
    if t in type_params:
        return "serde_json::Value"  # 泛型类型参数
    if t in enum_names:
        # 同文件内嵌枚举（KEY -> Key）
        return pascal_enum_name(t)
    # 未知泛型基类（如嵌套类 Option<String>）：剥壳后解析基类再包回
    gm = re.match(r"([\w.]+)<(.+)>", t)
    if gm:
        base = map_type(gm.group(1), decl_key, classes, nested, top_level, type_params, enum_names, cur_file)
        if base != gm.group(1):  # 基类解析成功
            args = map_type(gm.group(2), decl_key, classes, nested, top_level, type_params, enum_names, cur_file)
            return f"{base}<{args}>"
    # 同文件作用域解析（含同名嵌套类区分）
    key = resolve_scope_type(t, decl_key, classes, nested, top_level)
    if key is not None:
        return key
    if "." in t:
        outer, inner = t.rsplit(".", 1)
        # 跨文件内嵌类引用（Outer.Inner）：定位包含两者的文件
        entry = resolve_class(outer, cur_file)
        if entry is None:
            entry = resolve_class(inner, cur_file)
        if entry is not None:
            return module_path(entry["file"]) + "::" + inner
        return t.split(".")[-1].strip()
    # 跨文件顶层类引用：完整模块路径
    entry = resolve_class(t, cur_file)
    if entry is not None:
        return module_path(entry["file"]) + "::" + entry["key"]
    return t  # 未知引用（HAND_WRITTEN 类型等），编译期暴露


def module_path(rel_java: str) -> str:
    """Java 相对路径 -> Rust 模块路径（如 external/msg/Attachment.java ->
    crate::bean::external::msg::attachment）。"""
    parts = rel_java[:-5].split("/")
    mod = snake(parts[-1])
    if mod in ("abstract", "final", "type"):
        mod = f"r#{mod}"
    parts[-1] = mod
    return "crate::bean::" + "::".join(parts)


# Rust 预置/std 类型名黑名单：Java 嵌套类同名时（如 WxCpHrEmployeeFieldInfo.Option）
# 经 glob 导入会遮蔽 std 类型，生成时以「顶层类名+类名」重命名。
RUST_PRELUDE_BLACKLIST = {
    "Option", "Result", "Box", "Vec", "String", "HashMap", "BTreeMap", "Self",
    "None", "Some", "Ok", "Err", "Default", "Clone", "Debug", "Copy", "Hash",
    "Into", "From", "Sized", "Send", "Sync", "Drop", "Fn", "FnMut", "FnOnce",
    "Iterator", "IntoIterator", "ToString", "AsRef", "AsMut", "Borrow",
    "ToOwned", "Error", "Serialize", "Deserialize", "PartialEq", "PartialOrd",
    "Eq", "Ord", "Display", "TryFrom", "TryInto", "Future", "AsyncFn",
}


def gen_struct(name, fields, classes, nested, top_level, type_params, enum_names, cur_file, indent=""):
    lines = [f"{indent}#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]"]
    lines.append(f"{indent}pub struct {name} {{")
    for ftype, fname, serialized, decl_key in fields:
        rt = map_type(ftype, decl_key, classes, nested, top_level, type_params, enum_names, cur_file)
        json_name = serialized.split("|")[0] if serialized else fname
        if serialized and "|" in serialized:
            primary, alternate = serialized.split("|", 1)
            lines.append(f"{indent}    #[serde(rename = \"{primary}\", alias = \"{alternate}\", default)]")
        else:
            lines.append(f"{indent}    #[serde(rename = \"{json_name}\", default)]")
        lines.append(f"{indent}    pub {rust_ident(snake(fname))}: {rt},")
    lines.append(f"{indent}}}")
    return "\n".join(lines)


def gen_enum(name, variants, indent=""):
    """生成 serde 枚举：变体 rename 取 @SerializedName 值（缺省取常量名），
    与 Gson 对枚举常量的线格式一致（@SerializedName 优先，否则常量名）。
    首个变体标 `#[default]` 并派生 Default（对应 serde `default` 缺省填充）。"""
    lines = [f"{indent}#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]"]
    lines.append(f"{indent}pub enum {pascal_enum_name(name)} {{")
    for i, (variant, ser) in enumerate(variants):
        lines.append(f"{indent}    #[serde(rename = \"{ser}\")]")
        if i == 0:
            lines.append(f"{indent}    #[default]")
        lines.append(f"{indent}    {enum_variant_name(variant)},")
    lines.append(f"{indent}}}")
    return "\n".join(lines)


# ---------------------------------------------------------------------------
# 全局类缓存：整个 bean 目录树的类（含跨文件/跨包父类），用于继承扁平化与
# 跨文件类型解析。缓存条目携带该文件的完整解析结构（classes/nested/top）。
# ---------------------------------------------------------------------------
GLOBAL_CACHE = {}
FILE_CLASSES = {}


def build_global_cache():
    """扫描 JAVA_BEAN 下全部 .java，填充 GLOBAL_CACHE / FILE_CLASSES。"""
    for root, _dirs, files in os.walk(JAVA_BEAN):
        for fn in sorted(files):
            if not fn.endswith(".java") or fn == "package-info.java":
                continue
            path = os.path.join(root, fn)
            rel = os.path.relpath(path, JAVA_BEAN)
            order, classes, flags, type_params, enums, _, _, nested, top_level = parse_java(path)
            file_cls = {}
            for key in order:
                entry = {
                    "file": rel,
                    "key": key,
                    "java": classes[key]["java"],
                    "fields": classes[key]["fields"],
                    "parent_java": classes[key]["parent_java"],
                    "params": type_params,
                    "classes": classes,
                    "nested": nested,
                    "top_level": top_level,
                }
                GLOBAL_CACHE.setdefault(classes[key]["java"], []).append(entry)
                file_cls[key] = entry
            FILE_CLASSES[rel] = file_cls


def resolve_class(name, cur_file):
    """按名称解析类条目：优先当前文件，其次唯一命中；支持 `Outer.Inner` 形式。"""
    if "." in name:
        outer, inner = name.rsplit(".", 1)
        for f, cls in FILE_CLASSES.items():
            for key, entry in cls.items():
                if entry["java"] == outer and inner in entry["nested"]:
                    return entry["nested"][inner] and _entry_for(f, entry["nested"][inner])
        return None
    cand = GLOBAL_CACHE.get(name, [])
    if len(cand) == 1:
        return cand[0]
    for e in cand:
        if e["file"] == cur_file:
            return e
    return cand[0] if cand else None


def _entry_for(rel, key):
    return FILE_CLASSES[rel].get(key)


def flatten_fields(key, cur_file, seen=None):
    """递归合并父类字段（父类在前，对应 Gson 反射序列化层级顺序），跨文件生效。

    字段保留各自声明类作用域（decl_key），供类型解析使用。
    """
    seen = seen or set()
    if key in seen:
        return []
    seen.add(key)
    file_cls = FILE_CLASSES.get(cur_file, {})
    info = file_cls.get(key)
    if not info:
        return []
    parent_java = info["parent_java"]
    if not parent_java or parent_java in ("Object", "Serializable"):
        return list(info["fields"])
    parent_entry = resolve_class(parent_java, cur_file)
    if not parent_entry:
        return list(info["fields"])
    return flatten_fields(parent_entry["key"], parent_entry["file"], seen) + list(info["fields"])


def collect_params(key, cur_file, seen=None):
    """沿父类链收集泛型类型参数（子类引用父类泛型参数时按 Value 展开）。"""
    seen = seen or set()
    if key in seen:
        return set()
    seen.add(key)
    file_cls = FILE_CLASSES.get(cur_file, {})
    info = file_cls.get(key)
    if not info:
        return set()
    params = set(info["params"])
    parent_java = info["parent_java"]
    if parent_java and parent_java not in ("Object", "Serializable"):
        parent_entry = resolve_class(parent_java, cur_file)
        if parent_entry:
            params |= collect_params(parent_entry["key"], parent_entry["file"], seen)
    return params


def gen_file(java_path, rust_rel):
    """为单个 Java 文件生成 Rust 文件；纯数据类/枚举才生成。返回 (content, structs) 或 None。"""
    (
        order, classes, flags, type_params, enums, enum_order, top_enum,
        nested, top_level,
    ) = parse_java(java_path)
    if flags["is_enum"]:
        if not order and top_enum in enums:
            body = gen_enum(top_enum, enums[top_enum])
            java_class = f"{JAVA_PKG}.{rust_rel.replace('/', '.')}"
            return (f"""//! 对应 Java `{java_class}`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 枚举生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

{body}
""", re.findall(r"pub enum (\w+)", body))
        return None
    if not order:
        return None
    top = order[0]
    if (
        not classes[top]["fields"]
        and classes[top].get("parent_java") in (None, "Object", "Serializable")
        and len(order) == 1
        and not enums
    ):
        # 无字段、无父类的空壳类无数据可迁移
        return None
    enum_names = set(enums.keys())
    body_parts = []
    for key in order:
        fields = flatten_fields(key, rust_rel)
        params = collect_params(key, rust_rel) | type_params
        body_parts.append(
            gen_struct(key, fields, classes, nested, top_level, params, enum_names, rust_rel)
        )
    for ename in enum_order:
        body_parts.append(gen_enum(ename, enums[ename]))
    body = "\n\n".join(body_parts)
    structs = re.findall(r"pub (?:struct|enum) (\w+)", body)
    java_class = f"{JAVA_PKG}.{rust_rel.replace('/', '.')}"
    return (f"""//! 对应 Java `{java_class}`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

{body}
""", structs)


# ---------------------------------------------------------------------------
# 手写版本（Gson/XStream adapter 线格式权威 / 消息类，生成器禁止覆盖）
# ---------------------------------------------------------------------------
HAND_WRITTEN = {
    # 消息域（Java bean/message 子包）：XML 入站/出站消息与手动 toJson 的 JSON
    # 消息类，人工迁移至 src/bean/message/（生成器仅登记已有 Rust 文件）
    'message/WxCpXmlMessage.java',
    'message/WxCpTpXmlMessage.java',
    'message/WxCpXmlApprovalInfo.java',
    'message/WxCpXmlOutMessage.java',
    'message/WxCpXmlOutEventMessage.java',
    'message/WxCpXmlOutImageMessage.java',
    'message/WxCpXmlOutNewsMessage.java',
    'message/WxCpXmlOutTaskCardMessage.java',
    'message/WxCpXmlOutTextMessage.java',
    'message/WxCpXmlOutUpdateBtnMessage.java',
    'message/WxCpXmlOutVideoMessage.java',
    'message/WxCpXmlOutVoiceMessage.java',
    'message/WxCpMessage.java',
    'message/WxCpAppChatMessage.java',
    'message/WxCpGroupRobotMessage.java',
    'message/WxCpLinkedCorpMessage.java',
    'message/WxCpSchoolContactMessage.java',
    # 服务商推送 XML 包裹（XStream fromXml 解析，人工迁移至 bean 根）
    'WxCpTpXmlPackage.java',
    # 任务卡片按钮：Boolean 可空语义（is_bold 显式省略/输出），人工迁移
    'taskcard/TaskCardButton.java',
    # 模板卡片选项：Boolean 可空语义（is_checked 显式省略/输出），人工迁移
    'templatecard/CheckboxOption.java',
}

# Wave 2 人工迁移（Gson adapter 线格式权威在 util/json 的 adapter）：
# 本波次不产出 Rust 文件；若磁盘存在旧占位/陈迹文件则删除，防止误注册。
DEFERRED = {
    'WxCpUser.java',
    'WxCpTag.java',
    'WxCpDepart.java',
    'WxCpChat.java',
    'WxCpTagGetResult.java',   # 引用 WxCpUser，随 adapter 类一并迁移
    'WxCpTpTagGetResult.java', # 继承 WxCpTagGetResult，一并迁移
    'kf/WxCpKfGetCorpStatisticResp.java',
}

# 整个子目录跳过（迁移至 src/message/：messagebuilder/outxmlbuilder）
HAND_WRITTEN_DIRS = {
    "messagebuilder",
    "outxmlbuilder",
}

# 生成后追加的辅助 impl（from_json/to_json 语义；自动检测 + 显式登记，幂等）
POST_PROCESS = {
}

# Wave 0 门面（WxCpService）引用 bean 的 `new` 构造器（参数顺序与门面调用
# 一致；门面签名冻结，构造器不得改参）。键：相对 bean 根的 rust 文件路径。
NEW_IMPLS = {
    "wx_cp_agent_jsapi_signature.rs": (
        "impl WxCpAgentJsapiSignature {\n"
        "    /// 构建应用 jsapi 签名结果（对应 Java `@Builder` 语义；参数顺序与\n"
        "    /// `WxCpService::create_agent_jsapi_signature` 调用一致：corpid、\n"
        "    /// agentid、nonce_str、timestamp、url、signature。agentid 为 None\n"
        "    /// 时按 0 处理（Java Integer 缺省 0 的构造语义）。\n"
        "    pub fn new(\n"
        "        corpid: impl Into<String>,\n"
        "        agentid: Option<i32>,\n"
        "        nonce_str: impl Into<String>,\n"
        "        timestamp: i64,\n"
        "        url: impl Into<String>,\n"
        "        signature: impl Into<String>,\n"
        "    ) -> Self {\n"
        "        Self {\n"
        "            corpid: corpid.into(),\n"
        "            agentid: agentid.unwrap_or(0),\n"
        "            nonce_str: nonce_str.into(),\n"
        "            timestamp,\n"
        "            url: url.into(),\n"
        "            signature: signature.into(),\n"
        "        }\n"
        "    }\n"
        "}\n"
    ),
    "wx_cp_ma_js_code2_session_result.rs": (
        "impl WxCpMaJsCode2SessionResult {\n"
        "    /// 构建登录凭证校验结果。\n"
        "    pub fn new(\n"
        "        session_key: Option<String>,\n"
        "        user_id: Option<String>,\n"
        "        corp_id: Option<String>,\n"
        "    ) -> Self {\n"
        "        Self {\n"
        "            session_key: session_key.unwrap_or_default(),\n"
        "            user_id: user_id.unwrap_or_default(),\n"
        "            corp_id: corp_id.unwrap_or_default(),\n"
        "        }\n"
        "    }\n"
        "}\n"
    ),
    "wx_cp_provider_token.rs": (
        "impl WxCpProviderToken {\n"
        "    /// 构建服务商凭证。\n"
        "    pub fn new(\n"
        "        provider_access_token: Option<String>,\n"
        "        expires_in: Option<i32>,\n"
        "    ) -> Self {\n"
        "        Self {\n"
        "            provider_access_token: provider_access_token.unwrap_or_default(),\n"
        "            expires_in: expires_in.unwrap_or_default(),\n"
        "        }\n"
        "    }\n"
        "}\n"
    ),
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


def to_json_block(struct: str) -> str:
    return (
        "impl " + struct + " {\n"
        "    /// 序列化为 JSON（对应 Java `toJson`）。\n"
        "    pub fn to_json(&self) -> Result<String, String> {\n"
        "        serde_json::to_string(self).map_err(|e| format!(\"" + struct + " 序列化失败: {e}\"))\n"
        "    }\n"
        "}\n"
    )


def from_json_list_block(struct: str, key: str) -> str:
    return (
        "impl " + struct + " {\n"
        "    /// 从 JSON 构建列表（对应 Java `fromJson`：取 `" + key + "` 数组）。\n"
        "    pub fn from_json(json: &str) -> Result<Vec<Self>, String> {\n"
        "        let value: serde_json::Value =\n"
        "            serde_json::from_str(json).map_err(|e| format!(\"" + struct + " 列表解析失败: {e}\"))?;\n"
        "        let list = value.get(\"" + key + "\").ok_or_else(|| \"缺少 " + key + " 字段\".to_string())?;\n"
        "        serde_json::from_value(list.clone()).map_err(|e| format!(\"" + struct + " 列表解析失败: {e}\"))\n"
        "    }\n"
        "}\n"
    )


def post_process(rdir: str, rust_name: str, java_flags: dict):
    """追加辅助 impl（NEW_IMPLS 构造器 + from_json_list/from_json/to_json，已存在则跳过）。"""
    rel = os.path.relpath(os.path.join(rdir, rust_name), RUST_BEAN)
    p = os.path.join(rdir, rust_name)
    cur = open(p, encoding="utf-8").read()
    m = re.search(r"pub struct (\w+)", cur)
    struct = m.group(1) if m else None
    if struct is None:
        return

    actions = []
    spec = POST_PROCESS.get(rel)
    if spec is not None and spec[0] == "from_json_list":
        actions.append(("from_json_list", spec[1]))
    else:
        if java_flags.get("has_from_json"):
            actions.append(("from_json", None))
        if java_flags.get("has_to_json"):
            actions.append(("to_json", None))

    new_block = NEW_IMPLS.get(rel)
    if new_block and "pub fn new" not in cur:
        cur = cur.rstrip() + "\n\n" + new_block + "\n"

    for kind, key in actions:
        probe = "pub fn from_json" if kind == "from_json_list" else f"pub fn {kind}"
        if probe in cur:
            continue
        if kind == "from_json_list":
            block = from_json_list_block(struct, key)
        elif kind == "from_json":
            block = from_json_block(struct)
        else:
            block = to_json_block(struct)
        cur = cur.rstrip() + "\n\n" + block + "\n"
    if actions or new_block:
        open(p, "w", encoding="utf-8").write(cur)


# ---------------------------------------------------------------------------
# 目录树生成
# ---------------------------------------------------------------------------
ALL_SKIPPED = []
ALL_GENERATED = []


def ancestor_imports(rel_dir: str) -> str:
    """多级子目录文件引用祖先包类型：逐层追加通配导入（带 allow 属性抑制未用告警）。"""
    if not rel_dir:
        return ""
    parts = rel_dir.split("/")
    lines = []
    acc = ""
    for part in parts[:-1]:  # 不含自身目录
        acc = f"{acc}::{part}" if acc else part
        lines.append(f"#[allow(unused_imports)]\nuse crate::bean::{acc}::*;")
    return "\n".join(lines)


def gen_dir(rel_dir: str):
    """生成一个目录：本目录文件 + 递归子目录，写本目录 mod.rs。

    返回 (generated, skipped, exports)：exports 为 (path_spec, name) 列表，
    供父目录以显式 `pub use path_spec;` 方式按名去重转发，避免同名类型
    （Java 跨文件/跨子包同名的内嵌类）经 glob 重导出产生歧义。
    """
    jdir = os.path.join(JAVA_BEAN, rel_dir)
    rdir = os.path.join(RUST_BEAN, rel_dir)
    if not os.path.isdir(jdir):
        return [], [], []
    os.makedirs(rdir, exist_ok=True)

    mod_lines = []
    exports = []
    generated = []
    skipped = []

    java_files = sorted(
        fn for fn in os.listdir(jdir)
        if fn.endswith(".java") and fn != "package-info.java"
    )
    for fn in java_files:
        java_path = os.path.join(jdir, fn)
        rel_java = f"{rel_dir}/{fn}" if rel_dir else fn
        hw_rust = snake(fn[:-5]) + ".rs"
        hw_path = os.path.join(rdir, hw_rust)
        if rel_java in DEFERRED:
            # Wave 2 迁移：删除陈旧 Rust 文件，本波次不登记
            skipped.append(rel_java + " (Wave 2)")
            if os.path.isfile(hw_path):
                os.remove(hw_path)
            continue
        if rel_java in HAND_WRITTEN:
            # 手写迁移文件：若目标 Rust 文件已存在则纳入本目录 mod.rs 声明
            skipped.append(rel_java)
            if os.path.isfile(hw_path):
                hw_src = open(hw_path, encoding="utf-8").read()
                hw_structs = re.findall(r"pub (?:struct|enum) (\w+)", hw_src)
                mod_name = hw_rust[:-3]
                if mod_name in ("abstract", "final", "type"):
                    mod_name = f"r#{mod_name}"
                mod_lines.append(f"pub mod {mod_name};")
                for s in hw_structs:
                    exports.append((f"{mod_name}::{s}", s))
            continue
        content = gen_file(java_path, rel_java)
        if content is None:
            skipped.append(rel_java)
            continue
        content, structs = content
        # 子目录文件引用祖先包类型：`use super::*;` 之外逐层追加父包通配导入
        imports = ancestor_imports(rel_dir)
        if imports:
            content = content.replace("use super::*;", f"use super::*;\n{imports}")
        rust_name = snake(fn[:-5]) + ".rs"
        with open(os.path.join(rdir, rust_name), "w", encoding="utf-8") as f:
            f.write(content)
        parsed = parse_java(java_path)
        post_process(rdir, rust_name, parsed[2])
        mod_name = rust_name[:-3]
        if mod_name in ("abstract", "final", "type"):
            mod_name = f"r#{mod_name}"
        mod_lines.append(f"pub mod {mod_name};")
        for s in structs:
            exports.append((f"{mod_name}::{s}", s))
        generated.append(f"{rel_dir}/{rust_name}" if rel_dir else rust_name)

    # 递归子目录（跳过整体迁移目录）
    subdirs = sorted(
        d for d in os.listdir(jdir)
        if os.path.isdir(os.path.join(jdir, d))
    )
    for sub in subdirs:
        if sub in HAND_WRITTEN_DIRS:
            skipped.append(f"{rel_dir}/{sub}/" if rel_dir else f"{sub}/")
            continue
        sub_rel = f"{rel_dir}/{sub}" if rel_dir else sub
        sub_generated, sub_skipped, sub_exports = gen_dir(sub_rel)
        generated.extend(sub_generated)
        skipped.extend(sub_skipped)
        mod_lines.append(f"pub mod {sub};")
        for path_spec, name in sub_exports:
            exports.append((f"{sub}::{path_spec}", name))

    # 同名类型去重：保留首个（Java 语义下同包引用取同文件/同子包优先，已由
    # 生成阶段按文件/作用域解析保证；此处保证模块级导出无歧义）
    seen = set()
    deduped = []
    for path_spec, name in exports:
        if name in seen:
            continue
        seen.add(name)
        deduped.append((path_spec, name))

    if mod_lines:
        pkg_path = f"{JAVA_PKG}.{rel_dir}" if rel_dir else JAVA_PKG
        header = f"//! 对应 Java `{pkg_path}` 包（生成）。\n"
        uses = "\n".join(f"pub use {path_spec};" for path_spec, _ in deduped)
        with open(os.path.join(rdir, "mod.rs"), "w", encoding="utf-8") as f:
            f.write(header + "\n" + "\n".join(mod_lines) + "\n\n" + uses + "\n")
    return generated, skipped, deduped


def main():
    if not os.path.isdir(JAVA_BEAN):
        print(f"Java bean 目录不存在: {JAVA_BEAN}")
        sys.exit(1)
    build_global_cache()
    generated, skipped, root_exports = gen_dir("")
    ALL_GENERATED.extend(generated)
    ALL_SKIPPED.extend(skipped)

    # 根 bean/mod.rs 汇总声明（子目录 + 根文件，显式按名去重转发避免 glob 歧义）
    root_mod_lines = ["//! 企业微信 bean。", "//!", f"//! 对应 Java `{JAVA_PKG}` 包（生成）。", ""]
    rdir = RUST_BEAN
    subdirs = sorted(
        d for d in os.listdir(rdir)
        if os.path.isdir(os.path.join(rdir, d))
    )
    root_files = sorted(fn for fn in os.listdir(rdir) if fn.endswith(".rs") and fn != "mod.rs")
    # 手写根 bean 文件（不在本次生成集合中）补充导出
    generated_root = set(g for g in generated if "/" not in g)
    for fn in root_files:
        if fn in generated_root:
            continue
        src = open(os.path.join(rdir, fn), encoding="utf-8").read()
        for s in re.findall(r"pub (?:struct|enum) (\w+)", src):
            root_exports.append((f"{fn[:-3]}::{s}", s))
    # 同名去重（保留先到者）
    seen = set()
    root_exports = [
        e for e in root_exports if not (e[1] in seen or seen.add(e[1]))
    ]
    root_mod_lines.extend(f"pub mod {d};" for d in subdirs)
    root_mod_lines.extend(f"pub mod {fn[:-3]};" for fn in root_files)
    with open(os.path.join(rdir, "mod.rs"), "w", encoding="utf-8") as f:
        f.write("\n".join(root_mod_lines) + "\n\n" + "\n".join(
            f"pub use {path_spec};" for path_spec, _ in root_exports
        ) + "\n")

    print(f"generated: {len(ALL_GENERATED)}")
    print(f"skipped: {len(ALL_SKIPPED)}")
    for s in ALL_SKIPPED:
        print("  skip:", s)


if __name__ == "__main__":
    main()
