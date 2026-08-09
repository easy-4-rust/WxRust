#!/usr/bin/env python3
"""weixin-java-open 数据 bean 批量生成器。

由 `scripts/gen_miniapp_bean_structs.py` 复制改造而来，服务于
crates/wx-rust-open（对应 Java `me.chanjar.weixin.open.bean` 包）。

遍历 open bean 目录树（auth/authandicp/icp/ma/minishop/minishopgoods/mp/
result/shoppingOrders/tcb/tcbComponent 及多级子目录如 ma/privacy、
minishop/goods、minishop/coupon、minishop/limitdiscount 等，目录自动发现，
不硬编码），对纯数据类（@Data 或仅字段的类）生成 Rust struct + serde 派生，
写入 crates/wx-rust-open/src/bean/<dir>/。

保留 miniapp 生成器全部机制：
- 继承扁平化（flatten_fields，跨文件/跨包父类经全局 cache 递归合并，父类字段在前）
- 内嵌类（static class → 同文件多 struct）
- `use super::*;` + 祖先包通配导入（多级子目录逐层导入）
- 每目录 mod.rs（pub mod + pub use），根 bean/mod.rs 汇总声明
- HAND_WRITTEN 保护集（Gson adapter 线格式权威 / Wave 0 手写 / 消息类，
  禁止覆盖；已存在的手写 rust 文件自动纳入目录 mod.rs 声明）
- POST_PROCESS（from_json / to_json 辅助，幂等）
- `#[allow(unused_imports)]`、`#[serde(rename = ...)]`、`alternate` → alias

相对 miniapp 生成器的适配点：
1. 路径/包名换为 open；
2. EXTRA_TYPE_MAP：跨包类型显式映射（common 的 CommonUploadParam/
   CommonUploadData/WxError → wx_rust_common；根包 CommonUploadMultiParam
   → crate::bean）；
3. 内嵌枚举支持（minishop/MinishopDeliveryTemplate.ValuationType）：枚举
   常量 → Rust 枚举变体，`#[serde(rename = "常量名")]` 保留 Gson name() 线
   格式，首变体 `#[default]`（父 struct 派生 Default 需要）；
4. 跨目录显式导入（cross_dir_imports）：单层子目录文件引用其他子目录类型
   （result→ma、icp→auth 等）时按「类型名唯一命中」补 `use crate::bean::<dir>::<Type>;`
   （miniapp 生成器仅依赖父包 glob，未覆盖跨顶层目录引用）；
5. HAND_WRITTEN 增补：adapter 驱动解析的 auth/result 6 bean
   （WxOpenAuthorizationInfo/WxOpenAuthorizerInfo/WxOpenQueryAuthResult/
   WxOpenAuthorizerInfoResult/WxOpenAuthorizerListResult/
   WxOpenAuthorizerOptionResult，WxOpenGsonBuilder 注册 TypeAdapter 的类）、
   auth/WxOpenMiniProgramInfo（commons Pair 字段）、message/ 3 消息类
   （XML 手工迁移 + 跨 crate 引用）、ma/privacy/PrivacyKeyEnum（纯枚举）。

Date -> String（Java Gson 输出 ISO 8601 字符串，线格式原样保留）。
"""
import os
import re
import sys

BASE = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
JAVA_BEAN = "/Users/wandl/workspaces/workspace-github/WxJava/weixin-java-open/src/main/java/me/chanjar/weixin/open/bean"
RUST_BEAN = os.path.join(BASE, "crates", "wx-rust-open", "src", "bean")
JAVA_PKG = "me.chanjar.weixin.open.bean"

# 适配点 6：open bean 有 protected 字段（WxOpenResult 的 errcode/errmsg 等），
# 需与 private 一并捕获（miniapp 生成器仅匹配 private）
FIELD_RE = re.compile(r"^\s*(?:private|protected)\s+(?!static\s+final\s+)([\w<>\[\],. ]+?)\s+(\w+)\s*(?:=\s*[^;]*)?;")
# 适配点 7：open bean 有包私有（无修饰符）字段（如
# WxOpenMaHistoryVersionResult.versions），仅在类体括号深度 1 处匹配；
# 行内出现 `(`（方法声明）或 `=`（带初始化）不匹配
BARE_FIELD_RE = re.compile(r"^\s+([\w<>\[\],. ]+?)\s+(\w+)\s*;$")
# 类声明行：捕获类名、泛型参数与可选 extends 父类（同一行声明，如
# `public class X<T> extends Y implements Serializable {` / `public class X extends Y {`）
CLASS_RE = re.compile(
    r"^\s*(?:public\s+|private\s+)?(?:abstract\s+|final\s+|static\s+)*class\s+(\w+)"
    r"(?:<([^>]*)>)?(?:\s+extends\s+([\w<>,. ]+?))?(?:\s+implements\s+[\w<>,. ]+)?\s*\{"
)
# 内嵌/顶层枚举声明（顶层枚举 = 文件级，整文件跳过；内嵌枚举 → Rust 枚举变体）
ENUM_RE = re.compile(r"^\s*(?:public\s+|private\s+)?(?:static\s+)?enum\s+(\w+)\s*\{")

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
    "Map<String, Long>": "std::collections::HashMap<String, i64>",
    "Map<Integer, Integer>": "std::collections::HashMap<i32, i32>",
    "JsonArray": "serde_json::Value",
    "JsonObject": "serde_json::Value",
    "Pair<String, String>": "(String, String)",  # commons Pair，见 HAND_WRITTEN WxOpenMiniProgramInfo
}

# 适配点 2：跨包类型显式映射（先于类名解析命中）
EXTRA_TYPE_MAP = {
    "CommonUploadParam": "wx_rust_common::bean::CommonUploadParam",
    "CommonUploadData": "wx_rust_common::bean::CommonUploadData",
    "WxError": "wx_rust_common::error::WxError",
    "CommonUploadMultiParam": "crate::bean::CommonUploadMultiParam",
}


def map_type(t: str, classes: set, type_params: set) -> str:
    t = t.strip()
    if t.startswith("final "):
        t = t[len("final "):]
    if t in EXTRA_TYPE_MAP:
        return EXTRA_TYPE_MAP[t]
    m = re.match(r"(?:List|Collection)<(.+)>", t)
    if m:
        return f"Vec<{map_type(m.group(1), classes, type_params)}>"
    m = re.match(r"Map<([^,]+),\s*(.+)>", t)
    if m:
        k = map_type(m.group(1), classes, type_params)
        v = map_type(m.group(2), classes, type_params)
        return f"std::collections::HashMap<{k}, {v}>"
    if t.endswith("[]"):
        return f"Vec<{map_type(t[:-2], classes, type_params)}>"
    if t in TYPE_MAP:
        return TYPE_MAP[t]
    if t in type_params:
        return "serde_json::Value"  # 泛型类型参数（如 Result<T> 的 T）
    if "." in t:
        # 跨文件内嵌类引用（如 WxOpenApplyIcpFilingResult.Hint）：内嵌类在生成时
        # 是各自模块的顶层 struct，经 mod.rs pub use 按简单名可达，故取末段
        return t.split(".")[-1].strip()
    return t  # 类名引用（同文件嵌套或模块内）


def snake(name: str) -> str:
    s = re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", name)
    s = re.sub(r"([A-Z]+)([A-Z][a-z])", r"\1_\2", s)
    return s.lower()


def upper_camel(name: str) -> str:
    """枚举常量名 → Rust 变体名（UPPER_SNAKE -> UpperCamel）。"""
    return "".join(p[:1].upper() + p[1:].lower() for p in name.split("_") if p)


def parse_java(path):
    """返回 (order, classes, parents, flags, type_params, enums)。

    - order：文件内类声明顺序（首个为顶层类）
    - classes：name -> [(type, name, serialized)]
    - parents：name -> 父类名（仅记录非 Serializable/Object 的 extends）
    - flags：dict(is_enum, has_from_json, has_to_json)
    - type_params：文件内声明的泛型类型参数集合（如 Result<T> 的 T）
    - enums：name -> [常量名, ...]（内嵌枚举；顶层枚举整文件跳过）
    """
    classes = {}
    order = []
    cur = None
    pending_serialized = None
    parents = {}
    type_params = set()
    has_from_json = False
    has_to_json = False
    is_enum = False
    enums = {}
    in_enum = None  # 正在收集常量的内嵌枚举类名
    enum_consts = []
    # 适配点 8：括号深度栈修正内嵌类归属——Java 内嵌类闭合后其后的字段属于
    # 外层类（如 MinishopBrand.brandInfo），miniapp 生成器按行 cur 记名会
    # 错挂到内嵌类上
    class_stack = []  # (类名, 该行 } 计入后的深度)
    depth = 0
    with open(path, encoding="utf-8") as f:
        for line in f:
            # ---- 括号深度跟踪（先于各处理器）----
            net = line.count("{") - line.count("}")
            depth += net
            cm = CLASS_RE.match(line)
            if cm:
                name = cm.group(1)
                if cm.group(2):
                    for p in re.split(r"\s*,\s*", cm.group(2)):
                        p = p.strip()
                        if p:
                            # 只取类型参数名（如 `<T extends X.Y>` -> T）
                            type_params.add(re.split(r"\s+", p)[0])
                parent = cm.group(3)
                if parent and "Serializable" not in parent and "Object" not in parent:
                    # 泛型父类（如 Base<T>）去掉类型参数
                    parent = re.split(r"[<,]", parent)[0].strip()
                    parents[name] = parent
                cur = name
                class_stack.append((name, depth))
                if name not in classes:
                    classes[name] = []
                    order.append(name)
                continue
            while class_stack and depth < class_stack[-1][1]:
                class_stack.pop()
            cur = class_stack[-1][0] if class_stack else None

            em = ENUM_RE.match(line)
            if em:
                if cur is None:
                    # 顶层枚举（文件级）：整文件跳过（同 miniapp PayMode）
                    is_enum = True
                    continue
                in_enum = em.group(1)
                enum_consts = []
                # 内嵌枚举登记为文件内类（生成 Rust 枚举；适配点 3）
                if in_enum not in classes:
                    classes[in_enum] = []
                    order.append(in_enum)
                continue
            if in_enum is not None:
                s = line.strip()
                if s.startswith("}"):
                    enums[in_enum] = list(enum_consts)
                    in_enum = None
                    continue
                # 收集常量：标识符后跟 ( 或 , 或 ; 或行尾；跳过注解/修饰符/方法行
                m = re.match(r"([A-Za-z_][A-Za-z0-9_]*)\s*(?:\(|,|;)?\s*$", s)
                if m and not re.match(
                    r"(?:public|private|protected|static|final|@|get[A-Z]|set[A-Z]|is[A-Z])",
                    s,
                ):
                    enum_consts.append(m.group(1))
                continue
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
            bfm = None
            class_depth = class_stack[-1][1] if class_stack else 0
            if not fm and depth == class_depth and "(" not in line and "=" not in line:
                # 包私有字段（无修饰符）；深度恰为当前类体深度（非方法/块内）
                bfm = BARE_FIELD_RE.match(line)
            if fm or bfm:
                ftype, fname = (fm or bfm).group(1).strip(), (fm or bfm).group(2)
                if pending_serialized == "!skip":
                    pending_serialized = None
                else:
                    classes[cur].append((ftype, fname, pending_serialized))
                    pending_serialized = None
    flags = {
        "is_enum": is_enum,
        "has_from_json": has_from_json,
        "has_to_json": has_to_json,
    }
    return order, classes, parents, flags, type_params, enums


def rust_ident(name: str) -> str:
    keywords = {
        "type", "match", "move", "ref", "loop", "self", "super", "crate",
        "fn", "impl", "let", "mut", "pub", "struct", "enum", "trait",
        "where", "use", "as", "in", "for", "if", "else", "while", "return",
        "break", "continue", "true", "false", "static", "const", "mod",
        "async", "await", "dyn", "box",
    }
    return f"r#{name}" if name in keywords else name


def gen_enum(name, consts, indent=""):
    lines = [
        f"{indent}#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]",
        f"{indent}pub enum {name} {{",
    ]
    for i, c in enumerate(consts):
        if i == 0:
            lines.append(f"{indent}    #[default]")
        lines.append(f"{indent}    #[serde(rename = \"{c}\")]")
        lines.append(f"{indent}    {upper_camel(c)},")
    lines.append(f"{indent}}}")
    return "\n".join(lines)


def gen_struct(name, fields, classes, type_params, indent=""):
    lines = [f"{indent}#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]"]
    lines.append(f"{indent}pub struct {name} {{")
    for ftype, fname, serialized in fields:
        rt = map_type(ftype, set(classes.keys()), type_params)
        json_name = serialized.split("|")[0] if serialized else fname
        if serialized and "|" in serialized:
            primary, alternate = serialized.split("|", 1)
            lines.append(f"{indent}    #[serde(rename = \"{primary}\", alias = \"{alternate}\", default)]")
        else:
            lines.append(f"{indent}    #[serde(rename = \"{json_name}\", default)]")
        lines.append(f"{indent}    pub {rust_ident(snake(fname))}: {rt},")
    lines.append(f"{indent}}}")
    return "\n".join(lines)


# ---------------------------------------------------------------------------
# 全局类缓存：整个 bean 目录树的类（含跨文件/跨包父类），用于继承扁平化。
#
# Java 内嵌类简单名可能跨文件/跨包重复（如 authandicp 与 icp 都有
# WxOpenSubmitAuthAndIcpParam、icp 内多文件同名 AppletsBaseInfo），故按
# 「文件名 + 类名」定位：
# - GLOBAL_CACHE: 简单类名 -> [entry, ...]（entry = {file, fields, parent, params}）
# - FILE_CLASSES: (file 相对路径) -> {类名 -> entry}
# - DIR_NAME_INDEX: (目录) -> {类名 -> [file, ...]}（跨目录导入用）
# 解析父类时优先当前文件，其次唯一命中；`Outer.Inner` 形式先找 Outer 所在文件。
# ---------------------------------------------------------------------------
GLOBAL_CACHE = {}
FILE_CLASSES = {}
DIR_NAME_INDEX = {}


def build_global_cache():
    """扫描 JAVA_BEAN 下全部 .java，填充 GLOBAL_CACHE / FILE_CLASSES / DIR_NAME_INDEX。"""
    for root, _dirs, files in os.walk(JAVA_BEAN):
        for fn in sorted(files):
            if not fn.endswith(".java") or fn == "package-info.java":
                continue
            path = os.path.join(root, fn)
            rel = os.path.relpath(path, JAVA_BEAN)
            order, classes, parents, _, type_params, _ = parse_java(path)
            file_cls = {}
            for name in order:
                entry = {
                    "file": rel,
                    "fields": classes.get(name, []),
                    "parent": parents.get(name),
                    "params": type_params,
                }
                GLOBAL_CACHE.setdefault(name, []).append(entry)
                file_cls[name] = entry
            FILE_CLASSES[rel] = file_cls
            jdir = os.path.dirname(rel)
            for name in order:
                DIR_NAME_INDEX.setdefault(jdir, {}).setdefault(name, []).append(rel)


def resolve_class(name, cur_file):
    """按名称解析类条目：优先当前文件，其次唯一命中；支持 `Outer.Inner` 形式。"""
    if "." in name:
        outer, inner = name.rsplit(".", 1)
        if outer in FILE_CLASSES.get(cur_file, {}):
            inner_entry = FILE_CLASSES[cur_file].get(inner)
            if inner_entry:
                return inner_entry
        for f, cls in FILE_CLASSES.items():
            if outer in cls and inner in cls:
                return cls[inner]
        return None
    cand = GLOBAL_CACHE.get(name, [])
    if len(cand) == 1:
        return cand[0]
    for e in cand:
        if e["file"] == cur_file:
            return e
    return cand[0] if cand else None


def flatten_fields(name, cur_file, seen=None):
    """递归合并父类字段（父类在前，对应 Gson 反射序列化层级顺序），跨文件生效。"""
    seen = seen or set()
    if name in seen:
        return []
    seen.add(name)
    info = resolve_class(name, cur_file)
    if not info:
        return []
    parent = info["parent"]
    if not parent or parent in ("Object", "Serializable"):
        return list(info["fields"])
    return flatten_fields(parent, cur_file, seen) + list(info["fields"])


def collect_params(name, cur_file, seen=None):
    """沿父类链收集泛型类型参数（子类引用父类泛型参数时按 Value 展开）。"""
    seen = seen or set()
    if name in seen:
        return set()
    seen.add(name)
    info = resolve_class(name, cur_file)
    if not info:
        return set()
    params = set(info["params"])
    parent = info["parent"]
    if parent and parent not in ("Object", "Serializable"):
        params |= collect_params(parent, cur_file, seen)
    return params


def dedupe_fields(fields):
    """按 JSON 键去重（适配点 9）：子类与父类声明同名 JSON 键时子类覆盖父类。

    例：`WxFastMaCanSetCategoryResult extends WxOpenResult` 且自带
    `@SerializedName("errcode") private int errCode;` —— 线格式只有一个
    `errcode` 键，Gson 按字段注册序后者（子类）覆盖；Rust struct 亦只保留
    子类字段，避免重复 `#[serde(rename)]` 字段（unreachable pattern）。
    """
    seen = {}
    out = []
    for f in fields:
        ftype, fname, serialized = f
        key = serialized.split("|")[0] if serialized else fname
        if key in seen:
            out[seen[key]] = f
        else:
            seen[key] = len(out)
            out.append(f)
    return out


def gen_file(java_path, rust_rel):
    """为单个 Java 文件生成 Rust 文件；纯数据类才生成。

    返回 (content, structs, ref_types) 或 None。
    ref_types：文件字段引用的原始类型名集合（跨目录导入用）。
    """
    order, classes, parents, flags, type_params, enums = parse_java(java_path)
    if flags["is_enum"] or not order:
        return None
    top = order[0]
    if (
        not classes.get(top)
        and not (resolve_class(top, rust_rel) or {}).get("parent")
        and len(order) == 1
    ):
        # 无字段、无父类的空壳类无数据可迁移
        return None
    top_fields = dedupe_fields(flatten_fields(top, rust_rel))
    top_params = collect_params(top, rust_rel) | type_params
    body = gen_struct(top, top_fields, classes, top_params) if top not in enums \
        else gen_enum(top, enums[top])
    for inner in order[1:]:
        inner_fields = dedupe_fields(flatten_fields(inner, rust_rel))
        inner_params = collect_params(inner, rust_rel) | type_params
        if inner in enums:
            body += "\n\n" + gen_enum(inner, enums[inner])
        else:
            body += "\n\n" + gen_struct(inner, inner_fields, classes, inner_params)
    structs = re.findall(r"pub (?:struct|enum) (\w+)", body)
    # 收集全部字段原始类型（跨目录导入判定；不含父类链——父类字段已扁平化进本文件）
    ref_types = set()
    for name in order:
        for ftype, _fname, _ser in classes.get(name, []):
            ref_types.add(ftype.strip())
    java_class = f"{JAVA_PKG}.{rust_rel.replace('/', '.')}"
    return (f"""//! 对应 Java `{java_class}`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

{body}
""", structs, ref_types)


def type_names(t: str) -> set:
    """提取类型串中的内层类名集合（`List<X>` / `Map<K, V>` / `X[]` 递归拆解）。"""
    t = t.strip()
    if t.startswith("final "):
        t = t[len("final "):]
    m = re.match(r"(?:List|Collection)<(.+)>", t)
    if m:
        return type_names(m.group(1))
    if t.startswith("Map<"):
        inner = t[4:-1]
        # 顶层逗号分割（最后一个逗号前无嵌套 `>`）
        k, _, v = inner.rpartition(",")
        return type_names(k) | type_names(v)
    if t.endswith("[]"):
        return type_names(t[:-2])
    if t.startswith(("java.", "javax.")):
        return set()
    return {t}


def cross_dir_imports(rel_dir: str, ref_types: set) -> str:
    """适配点 4：单层子目录文件引用其他子目录类型时补显式导入。

    类型名在当前目录无定义、且在整个 bean 树中唯一命中另一目录时，
    追加 `#[allow(unused_imports)] use crate::bean::<dir>::<Type>;`。
    （miniapp 生成器仅靠父包 glob，未覆盖跨顶层目录引用。）
    """
    if not rel_dir:
        return ""
    lines = []
    for t in sorted(ref_types):
        for name in sorted(type_names(t)):
            if not name or name in TYPE_MAP or name in EXTRA_TYPE_MAP:
                continue
            name = name.split(".")[-1].strip()
            if not name:
                continue
            if name in DIR_NAME_INDEX.get(rel_dir, {}):
                continue  # 本目录有定义，经 use super::* 可达
            cand_dirs = [d for d, m in DIR_NAME_INDEX.items() if name in m]
            if len(cand_dirs) == 1 and cand_dirs[0]:
                lines.append(f"#[allow(unused_imports)]\nuse crate::bean::{cand_dirs[0]}::{name};")
    return "\n".join(lines)


# ---------------------------------------------------------------------------
# 手写版本（Gson adapter 线格式权威 / Wave 0 手写 / 消息类，生成器禁止覆盖）
# ---------------------------------------------------------------------------
HAND_WRITTEN = {
    # Wave 0 手写（勿覆盖）：token 响应体（带 from_json 语义与 getter）
    'WxOpenAuthorizerAccessToken.java',
    'WxOpenComponentAccessToken.java',
    # adapter 驱动解析（WxOpenGsonBuilder 注册 TypeAdapter，snake_case 键 +
    # 嵌套/扁平映射与 @SerializedName 不同），人工迁移
    'auth/WxOpenAuthorizationInfo.java',      # func_info 数组扁平为 List<Integer>
    'auth/WxOpenAuthorizerInfo.java',         # service/verify_type_info {"id":n} 扁平
    'result/WxOpenQueryAuthResult.java',
    'result/WxOpenAuthorizerInfoResult.java',
    'result/WxOpenAuthorizerListResult.java', # list -> List<Map<String,String>>
    'result/WxOpenAuthorizerOptionResult.java',
    # commons Pair 字段（无 Gson adapter，Gson 反射输出 ImmutablePair left/right）
    'auth/WxOpenMiniProgramInfo.java',
    # 字段类型无 serde 派生（CommonUploadParam/CommonUploadData 仅 Debug, Clone），
    # 手工迁移（multipart 载体不经过 JSON 线格式）
    'CommonUploadMultiParam.java',
    'icp/WxOpenUploadIcpMediaParam.java',
    # wx_rust_common::error::WxError 无 Default，wxError 字段手工迁移为 Option
    'minishop/MinishopAuditStatus.java',
    # 消息类：迁往 bean/message/（XML 手工迁移 / 跨 miniapp crate 引用）
    'message/WxOpenXmlMessage.java',
    'message/WxOpenMaSubmitAuditMessage.java',
    'message/WxOpenMaVerifyBetaWeappMessage.java',
    # 纯枚举（无字段线格式需求，同 miniapp PayMode 先例）
    'ma/privacy/PrivacyKeyEnum.java',
}

# 生成后追加的辅助 impl（from_json/to_json 语义；自动检测 + 显式登记，幂等）
POST_PROCESS = {}


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


def post_process(rdir: str, rust_name: str, java_flags: dict):
    """追加辅助 impl（from_json/to_json，已存在则跳过）。"""
    p = os.path.join(rdir, rust_name)
    cur = open(p, encoding="utf-8").read()
    m = re.search(r"pub struct (\w+)", cur)
    struct = m.group(1) if m else None
    if struct is None:
        return

    actions = []
    spec = POST_PROCESS.get(os.path.relpath(p, RUST_BEAN))
    if spec is not None and spec[0] == "from_json_list":
        actions.append(("from_json_list", spec[1]))
    else:
        if java_flags.get("has_from_json"):
            actions.append(("from_json", None))
        if java_flags.get("has_to_json"):
            actions.append(("to_json", None))

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
    if actions:
        open(p, "w", encoding="utf-8").write(cur)


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


def mod_decl(sub: str) -> str:
    """目录模块声明：camelCase 目录名（Java 原目录 shoppingOrders/tcbComponent）
    抑制 non_snake_case 告警（模块名须与磁盘目录名一致）。"""
    if sub != snake(sub):
        return f"#[allow(non_snake_case)]\npub mod {sub};"
    return f"pub mod {sub};"


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
        if rel_java in HAND_WRITTEN:
            # 手写迁移文件：若目标 Rust 文件已存在则纳入本目录 mod.rs 声明
            skipped.append(rel_java)
            hw_rust = snake(fn[:-5]) + ".rs"
            hw_path = os.path.join(rdir, hw_rust)
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
        content, structs, ref_types = content
        # 子目录文件引用祖先包类型：`use super::*;` 之外逐层追加父包通配导入；
        # 跨顶层目录引用（result→ma、icp→auth 等）追加显式导入（适配点 4）
        imports = ancestor_imports(rel_dir)
        cross = cross_dir_imports(rel_dir, ref_types)
        if imports or cross:
            combined = "\n".join(x for x in (imports, cross) if x)
            content = content.replace("use super::*;", f"use super::*;\n{combined}")
        rust_name = snake(fn[:-5]) + ".rs"
        with open(os.path.join(rdir, rust_name), "w", encoding="utf-8") as f:
            f.write(content)
        # 解析该文件再取一次 flags（gen_file 内部已解析，这里重新解析成本可接受）
        _, _, _, java_flags, _, _ = parse_java(java_path)
        post_process(rdir, rust_name, java_flags)
        mod_name = rust_name[:-3]
        if mod_name in ("abstract", "final", "type"):
            mod_name = f"r#{mod_name}"
        mod_lines.append(f"pub mod {mod_name};")
        for s in structs:
            exports.append((f"{mod_name}::{s}", s))
        generated.append(f"{rel_dir}/{rust_name}" if rel_dir else rust_name)

    # 递归子目录
    subdirs = sorted(
        d for d in os.listdir(jdir)
        if os.path.isdir(os.path.join(jdir, d))
    )
    for sub in subdirs:
        sub_rel = f"{rel_dir}/{sub}" if rel_dir else sub
        sub_generated, sub_skipped, sub_exports = gen_dir(sub_rel)
        generated.extend(sub_generated)
        skipped.extend(sub_skipped)
        mod_lines.append(mod_decl(sub))
        for path_spec, name in sub_exports:
            exports.append((f"{sub}::{path_spec}", name))

    # 同名类型去重：保留首个（Java 语义下同包引用取同文件/同子包优先，已由
    # 生成阶段 flatten 的按文件解析保证；此处保证模块级导出无歧义）
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


def count_java_classes():
    """统计 Java bean 类数（顶层 + 内嵌，剔除枚举与接口），用于对照生成量。"""
    total = 0
    enum_files = 0
    for root, _dirs, files in os.walk(JAVA_BEAN):
        for fn in files:
            if not fn.endswith(".java") or fn == "package-info.java":
                continue
            path = os.path.join(root, fn)
            with open(path, encoding="utf-8") as f:
                src = f.read()
            # 文件级枚举（行首无缩进）：整文件不产出 bean
            if re.search(r"(?m)^(?:public\s+)?enum\s+\w+\s*\{", src):
                enum_files += 1
                continue
            total += len(re.findall(r"(?m)^\s*(?:public\s+|private\s+)?(?:abstract\s+|final\s+|static\s+)*class\s+(\w+)", src))
            # 内嵌枚举也计为类（会产出 Rust 枚举）
            total += len(re.findall(r"(?m)^\s*(?:public\s+|private\s+)?(?:static\s+)?enum\s+(\w+)\s*\{", src))
    return total, enum_files


def main():
    if not os.path.isdir(JAVA_BEAN):
        print(f"Java bean 目录不存在: {JAVA_BEAN}")
        sys.exit(1)
    build_global_cache()
    generated, skipped, root_exports = gen_dir("")
    ALL_GENERATED.extend(generated)
    ALL_SKIPPED.extend(skipped)

    # 根 bean/mod.rs 汇总声明（子目录 + 根文件，显式按名去重转发避免 glob 歧义）
    root_mod_lines = ["//! 开放平台（第三方平台）数据对象。", "//!", f"//! 对应 Java `{JAVA_PKG}` 包（生成）。", ""]
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
    root_mod_lines.extend(mod_decl(d) for d in subdirs)
    root_mod_lines.extend(f"pub mod {fn[:-3]};" for fn in root_files)
    with open(os.path.join(rdir, "mod.rs"), "w", encoding="utf-8") as f:
        f.write("\n".join(root_mod_lines) + "\n\n" + "\n".join(
            f"pub use {path_spec};" for path_spec, _ in root_exports
        ) + "\n")

    java_classes, enum_files = count_java_classes()
    print(f"Java bean 类数（剔除文件级枚举）: {java_classes}（文件级枚举文件 {enum_files}）")
    print(f"generated: {len(ALL_GENERATED)}")
    print(f"skipped: {len(ALL_SKIPPED)}")
    for s in ALL_SKIPPED:
        print("  skip:", s)


if __name__ == "__main__":
    main()
