#!/usr/bin/env python3
"""weixin-java-channel 数据 bean 批量生成器（视频号小店）。

由 `scripts/gen_miniapp_bean_structs.py` 复制改造而来，服务于
crates/wx-rust-channel（对应 Java `me.chanjar.weixin.channel.bean` 包）。

遍历 channel bean 目录树（address/after/audit/base/brand/category/compass/
complaint/cooperation/coupon/delivery/freight/fund/home/image/lead/league/
limit/live/order/product/sharer/shop/token/vip/warehouse/window 及多级子目录
如 compass/finder、fund/bank、home/tree、lead/component/request、
league/product、window/request 等，目录自动发现，不硬编码），对纯数据类
（@Data 或仅字段的类）生成 Rust struct + serde 派生，写入
crates/wx-rust-channel/src/bean/<dir>/。

保留 miniapp 生成器全部机制：
- 继承扁平化（flatten_fields，跨文件/跨包父类经全局 cache 递归合并，父类字段在前）
- 内嵌类（static class → 同文件多 struct）
- `use super::*;` + 祖先包通配导入（多级子目录逐层导入）
- 每目录 mod.rs（pub mod + pub use），根 bean/mod.rs 汇总声明
- HAND_WRITTEN 保护集（Gson/Jackson adapter 线格式权威，禁止覆盖）
- POST_PROCESS（from_json / from_json_list / to_json 辅助，幂等）
- `#[allow(unused_imports)]`、`#[serde(rename = ...)]`、`alternate` → alias
- 陈旧占位清理：Wave 0 根级占位 .rs（如 address.rs）在目录化后自动删除

与 miniapp 生成器的差异（channel 适配点）：
- 包路径：me.chanjar.weixin.channel.bean（Java 实际包名）
- 注解：Jackson `@JsonProperty("x")`（channel 全部 bean 用 Jackson，
  无 Gson @SerializedName；1922 处 @JsonProperty、无 value= 形式）
- 字段可见性：`private` 与 `protected` 都是数据字段（Jackson 走 getter，
  如 WxChannelBaseResponse.errCode / OrderInfo.orderId 等 protected 字段）
- 方法签名行（`(` + `{`）重置待定注解：@JsonProperty/@JsonIgnore 可落在
  setter 上（如 AllConditionFreeDetail.addDetail），避免污染下一个字段
- HAND_WRITTEN：bean/message/**（消息子系统 bean，父类 WxChannelMessage
  在 bean 树外，含 XML 注解与 unpack 逻辑，留待消息批次）与
  warehouse/WarehouseIdsResponse、warehouse/WarehouseStockResponse
  （Java 以 `@JsonProperty("data")` setter 展开嵌套对象，需手写 serde 镜像）

排除：枚举（bean 树内无枚举）、含复杂逻辑的类（上述 HAND_WRITTEN）。

Date -> String（Jackson 输出 ISO 8601 字符串，线格式原样保留）。
"""
import os
import re
import sys

BASE = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
JAVA_BEAN = "/Users/wandl/workspaces/workspace-github/WxJava/weixin-java-channel/src/main/java/me/chanjar/weixin/channel/bean"
RUST_BEAN = os.path.join(BASE, "crates", "wx-rust-channel", "src", "bean")
JAVA_PKG = "me.chanjar.weixin.channel.bean"

# channel 适配：Jackson 数据字段可为 private 或 protected（WxChannelBaseResponse、
# OrderInfo、VipInfo 等均用 protected + Lombok getter）
FIELD_RE = re.compile(
    r"^\s*(?:private|protected)\s+(?!static\s+final\s+)([\w<>\[\],. ]+?)\s+(\w+)\s*(?:=\s*[^;]*)?;"
)
# 类声明行：捕获类名、泛型参数与可选 extends 父类（同一行声明，如
# `public class X<T> extends Y implements Serializable {` / `public class X extends Y {`）
CLASS_RE = re.compile(
    r"^\s*(?:public\s+|private\s+)?(?:abstract\s+|final\s+|static\s+)*class\s+(\w+)"
    r"(?:<([^>]*)>)?(?:\s+extends\s+([\w<>,. ]+?))?(?:\s+implements\s+[\w<>,. ]+)?\s*\{"
)

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
    "Date": "String",  # Jackson/Gson 输出 ISO 8601 字符串
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
}


def map_type(t: str, classes: set, type_params: set, string_types: set = None) -> str:
    t = t.strip()
    if t.startswith("final "):
        t = t[len("final "):]
    m = re.match(r"(?:List|Collection)<(.+)>", t)
    if m:
        return f"Vec<{map_type(m.group(1), classes, type_params, string_types)}>"
    m = re.match(r"Map<([^,]+),\s*(.+)>", t)
    if m:
        k = map_type(m.group(1), classes, type_params, string_types)
        v = map_type(m.group(2), classes, type_params, string_types)
        return f"std::collections::HashMap<{k}, {v}>"
    if t.endswith("[]"):
        return f"Vec<{map_type(t[:-2], classes, type_params, string_types)}>"
    if t in TYPE_MAP:
        return TYPE_MAP[t]
    if t in type_params:
        return "serde_json::Value"  # 泛型类型参数
    if string_types and t in string_types:
        # 枚举类型（bean 树外 me.chanjar.weixin.channel.enums.*，如
        # PackageAuditItemType）：Jackson 默认按枚举名序列化为字符串，Wave 1 以
        # String 镜像线格式，枚举迁移随 enums 批次补齐
        return "String"
    if "." in t:
        # 跨文件内嵌类引用（如 WxCloudDatabaseQueryResult.Pager）：内嵌类在生成时
        # 是各自模块的顶层 struct，经 mod.rs pub use 按简单名可达，故取末段
        return t.split(".")[-1].strip()
    return t  # 类名引用（同文件嵌套或模块内）


def snake(name: str) -> str:
    s = re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", name)
    s = re.sub(r"([A-Z]+)([A-Z][a-z])", r"\1_\2", s)
    return s.lower()


def parse_java(path):
    """返回 (order, classes, parents, flags, type_params)。

    - order：文件内类声明顺序（首个为顶层类）
    - classes：name -> [(type, name, serialized)]
    - parents：name -> 父类名（仅记录非 Serializable/Object 的 extends）
    - flags：dict(is_enum, has_from_json, has_to_json)
    - type_params：文件内声明的泛型类型参数集合
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
    with open(path, encoding="utf-8") as f:
        for line in f:
            if "public enum" in line or re.search(r"^\s*(?:public\s+)?enum\s+", line):
                is_enum = True
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
                if name not in classes:
                    classes[name] = []
                    order.append(name)
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
            # channel 适配：Jackson @JsonProperty（无 value= 形式）；alternate 语法
            # 一并兼容（Jackson 无此语法，保留分支仅为与 miniapp 机制一致）
            sm = re.search(r'@JsonProperty\((?:value\s*=\s*)?"([^"]+)"', line)
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
                continue
            # channel 适配：方法签名行（getter/setter/工具方法，形如 `xxx(...) {`）
            # 会消费掉上一行的 @JsonProperty/@JsonIgnore（Jackson 允许注解落在方法上，
            # 如 AllConditionFreeDetail.addDetail / WarehouseIdsResponse 的 data setter），
            # 必须重置待定注解，避免污染下一个字段
            if "(" in line and line.rstrip().endswith("{"):
                pending_serialized = None
    flags = {
        "is_enum": is_enum,
        "has_from_json": has_from_json,
        "has_to_json": has_to_json,
    }
    return order, classes, parents, flags, type_params


def rust_ident(name: str) -> str:
    keywords = {
        "type", "match", "move", "ref", "loop", "self", "super", "crate",
        "fn", "impl", "let", "mut", "pub", "struct", "enum", "trait",
        "where", "use", "as", "in", "for", "if", "else", "while", "return",
        "break", "continue", "true", "false", "static", "const", "mod",
        "async", "await", "dyn", "box",
    }
    return f"r#{name}" if name in keywords else name


def gen_struct(name, fields, classes, type_params, string_types=None, indent=""):
    lines = [f"{indent}#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]"]
    lines.append(f"{indent}pub struct {name} {{")
    for ftype, fname, serialized in fields:
        rt = map_type(ftype, set(classes.keys()), type_params, string_types)
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
# Java 内嵌类简单名可能跨文件/跨包重复，故按「文件名 + 类名」定位：
# - GLOBAL_CACHE: 简单类名 -> [entry, ...]（entry = {file, fields, parent, params}）
# - FILE_CLASSES: (file 相对路径) -> {类名 -> entry}
# 解析父类时优先当前文件，其次唯一命中；`Outer.Inner` 形式先找 Outer 所在文件。
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
            order, classes, parents, _, type_params = parse_java(path)
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
    """递归合并父类字段（父类在前，对应 Jackson 反射序列化层级顺序），跨文件生效。"""
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


def java_imports(path):
    """解析 Java 文件的 channel 包导入，返回 (use_lines, string_types)。

    - use_lines：bean 包类型导入对应的 rust `use` 行（精确命名导入，镜像 Java
      显式 import 语义；内嵌类导入如 `...bean.order.OrderInfo.OrderDetailInfo`
      扁平为包级导出 `crate::bean::order::OrderDetailInfo`）
    - string_types：bean 树外枚举类型名集合（me.chanjar.weixin.channel.enums.*，
      Wave 1 以 String 镜像 Jackson 枚举名字符串线格式）
    """
    use_lines = []
    string_types = set()
    with open(path, encoding="utf-8") as f:
        for line in f:
            m = re.match(r"\s*import\s+me\.chanjar\.weixin\.channel\.(.+?)\s*;", line)
            if not m:
                continue
            pkg_path = m.group(1)
            parts = pkg_path.split(".")
            if parts[0] == "bean":
                if len(parts) < 3 or parts[1] == "message":
                    # `bean` 根包无类型；message 子系统 bean（Wave 2 人工迁移）
                    continue
                segs = parts[1:]
                type_name = segs[-1]
                mod_segs = []
                for s in segs[:-1]:
                    if s[0].isupper():
                        break  # 内嵌类（Outer.Inner）：跳过类段，扁平到包级导出
                    mod_segs.append(snake(s))
                use_lines.append(f"use crate::bean::{'::'.join(mod_segs)}::{type_name};")
            elif parts[0] == "enums":
                string_types.add(parts[-1])
            # message / constant / util 等非 bean 导入：字段类型不会引用（父类
            # flatten 由 GLOBAL_CACHE 处理），忽略
    return sorted(set(use_lines)), string_types


def collect_chain_imports(order, rust_rel):
    """沿文件内全部类（含父类链）收集 bean 导入与枚举类型名。

    继承扁平化会把父类字段并入子类 struct，父类字段的类型可能来自父类所在
    Java 文件的 import（如 freight/NotSendArea extends AddressInfoList，
    AddressInfoList 导入 bean.base.AddressInfo），故必须沿父类链合并各文件导入。
    """
    use_lines = set()
    string_types = set()
    seen_files = set()
    for name in order:
        cur = name
        seen_names = set()
        while cur and cur not in seen_names:
            seen_names.add(cur)
            info = resolve_class(cur, rust_rel)
            if not info:
                break
            parent = info["parent"]
            f = info["file"]
            if f not in seen_files:
                seen_files.add(f)
                ul, st = java_imports(os.path.join(JAVA_BEAN, f))
                use_lines |= set(ul)
                string_types |= st
            if parent and parent not in ("Object", "Serializable"):
                cur = parent
            else:
                break
    return sorted(use_lines), string_types


def gen_file(java_path, rust_rel):
    """为单个 Java 文件生成 Rust 文件；纯数据类才生成。返回 (content, structs) 或 None。"""
    order, classes, parents, flags, type_params = parse_java(java_path)
    if flags["is_enum"] or not order:
        return None
    top = order[0]
    if (
        not classes.get(top)
        and not resolve_class(top, rust_rel).get("parent")
        and len(order) == 1
    ):
        # 无字段、无父类的空壳类无数据可迁移
        return None
    use_lines, string_types = collect_chain_imports(order, rust_rel)
    # 过滤：目标类型若定义于当前文件（本文件内嵌类/顶层类），导入会产生 E0255
    # 重复定义；同文件定义自身在作用域内可达，无需导入
    use_lines = [
        u for u in use_lines
        if u.rsplit("::", 1)[-1].rstrip(";") not in order
    ]
    top_fields = flatten_fields(top, rust_rel)
    top_params = collect_params(top, rust_rel) | type_params
    body = gen_struct(top, top_fields, classes, top_params, string_types)
    for inner in order[1:]:
        inner_fields = flatten_fields(inner, rust_rel)
        inner_params = collect_params(inner, rust_rel) | type_params
        body += "\n\n" + gen_struct(inner, inner_fields, classes, inner_params, string_types)
    structs = re.findall(r"pub struct (\w+)", body)
    java_class = f"{JAVA_PKG}.{rust_rel.replace('/', '.')}"
    imports_block = ""
    if use_lines:
        # 精确命名导入（镜像 Java import；allow 抑制未用告警）
        imports_block = "\n#[allow(unused_imports)]\n" + "\n#[allow(unused_imports)]\n".join(use_lines) + "\n"
    return (f"""//! 对应 Java `{java_class}`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;
{imports_block}
{body}
""", structs)


# ---------------------------------------------------------------------------
# 手写版本（Jackson adapter 线格式权威 / 消息类，生成器禁止覆盖）
# ---------------------------------------------------------------------------
HAND_WRITTEN = {
    # Jackson adapter 驱动：Java 以 `@JsonProperty("data")` setter 展开嵌套对象
    # （响应形如 {"data": {...}}），serde 需手写反序列化镜像，已手写
    'warehouse/WarehouseIdsResponse.java',
    'warehouse/WarehouseStockResponse.java',
}


def is_hand_written(rel_java: str) -> bool:
    """手写保护判定：显式登记，或 bean/message/** 消息子系统 bean。

    消息子系统 bean（50 个文件）父类 WxChannelMessage 在 bean 树外
    （me.chanjar.weixin.channel.message），含 @JacksonXml* XML 注解与
    unpack 嵌套对象逻辑，由消息批次人工迁移（对应 miniapp 消息类先例）。
    """
    if rel_java in HAND_WRITTEN:
        return True
    if rel_java.startswith("message/"):
        return True
    return False

# 生成后追加的辅助 impl（from_json/to_json 语义；自动检测 + 显式登记，幂等）
# channel bean 树内无 fromJson/toJson 方法，保持空表（机制保留）
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
    """追加辅助 impl（from_json_list/from_json/to_json，已存在则跳过）。"""
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


# ---------------------------------------------------------------------------
# 目录树生成
# ---------------------------------------------------------------------------
ALL_SKIPPED = []
ALL_GENERATED = []
ALL_HAND_WRITTEN_REGISTERED = []
WRITTEN = set()  # 本次运行写入的 .rs 文件（生成 + mod.rs），用于陈旧占位清理


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
        if is_hand_written(rel_java):
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
                ALL_HAND_WRITTEN_REGISTERED.append(rel_java)
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
        WRITTEN.add(os.path.join(rdir, rust_name))
        # 解析该文件再取一次 flags（gen_file 内部已解析，这里重新解析成本可接受）
        _, _, _, java_flags, _ = parse_java(java_path)
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
        # 仅当子目录实际产出内容（生成文件/手写登记文件）时声明 `pub mod`，
        # 避免 message 等整目录手写保护的空目录产出悬空模块声明（E0583）
        sub_rdir = os.path.join(rdir, sub)
        sub_has_content = bool(sub_generated) or bool(sub_exports) or any(
            fn.endswith(".rs")
            for _, _, fns in os.walk(sub_rdir)
            for fn in fns
        )
        if sub_has_content:
            mod_lines.append(f"pub mod {sub};")
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
        mod_rs = os.path.join(rdir, "mod.rs")
        with open(mod_rs, "w", encoding="utf-8") as f:
            f.write(header + "\n" + "\n".join(mod_lines) + "\n\n" + uses + "\n")
        WRITTEN.add(mod_rs)
    return generated, skipped, deduped


def collect_rs_files(root):
    out = []
    for r, _dirs, fns in os.walk(root):
        for fn in fns:
            if fn.endswith(".rs"):
                out.append(os.path.join(r, fn))
    return out


def hand_written_rust_targets():
    """HAND_WRITTEN Java 文件对应的 Rust 目标路径（仅这些文件受删除保护）。"""
    kept = set()
    for root, _dirs, fns in os.walk(JAVA_BEAN):
        for fn in fns:
            if not fn.endswith(".java") or fn == "package-info.java":
                continue
            rel_java = os.path.relpath(os.path.join(root, fn), JAVA_BEAN)
            if is_hand_written(rel_java):
                rel_dir = os.path.dirname(rel_java)
                rust_rel = os.path.join(rel_dir, snake(fn[:-5]) + ".rs")
                kept.add(os.path.join(RUST_BEAN, rust_rel))
    return kept


def cleanup_stale(before):
    """删除本次生成未覆盖的陈旧占位文件（Wave 0 根级 address.rs 等目录化后的旧文件），
    并移除空目录。手写保护文件（HAND_WRITTEN 且已存在）不会被删除。"""
    # 保留集：HAND_WRITTEN Java 对应的 Rust 目标文件（已存在的才保留）
    kept_hand = hand_written_rust_targets() & set(collect_rs_files(RUST_BEAN))
    stale = sorted(set(before) - WRITTEN - kept_hand)
    for f in stale:
        os.remove(f)
        print("  stale-removed:", os.path.relpath(f, RUST_BEAN))
    # 移除空目录（含仅剩空目录的嵌套）
    removed_dirs = True
    while removed_dirs:
        removed_dirs = False
        for root, dirs, fns in os.walk(RUST_BEAN, topdown=False):
            for d in dirs:
                dp = os.path.join(root, d)
                try:
                    os.rmdir(dp)
                    removed_dirs = True
                except OSError:
                    pass
    return len(stale)


def main():
    if not os.path.isdir(JAVA_BEAN):
        print(f"Java bean 目录不存在: {JAVA_BEAN}")
        sys.exit(1)
    before = set(collect_rs_files(RUST_BEAN))
    java_files_before = sum(
        len([f for f in fs if f.endswith(".java") and f != "package-info.java"])
        for _, _, fs in os.walk(JAVA_BEAN)
    )
    build_global_cache()
    generated, skipped, root_exports = gen_dir("")
    ALL_GENERATED.extend(generated)
    ALL_SKIPPED.extend(skipped)
    stale_count = cleanup_stale(before)

    # 根 bean/mod.rs 汇总声明（子目录 + 根文件，显式按名去重转发避免 glob 歧义）
    root_mod_lines = [
        "//! 视频号小店 bean。",
        "//!",
        f"//! 对应 Java `{JAVA_PKG}` 包（生成）。",
        "",
    ]
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

    # 统计
    struct_count = 0
    for f in collect_rs_files(rdir):
        if f.endswith("mod.rs"):
            continue
        src = open(f, encoding="utf-8").read()
        struct_count += len(re.findall(r"^pub struct (\w+)", src, re.M))
    hand_written = sorted(
        os.path.relpath(f, RUST_BEAN)
        for f in collect_rs_files(rdir)
        if os.path.basename(os.path.dirname(f)) == "warehouse"
        and "response" in os.path.basename(f)
        and open(f, encoding="utf-8").read().find("手写") != -1
    )
    print(f"java_files: {java_files_before}")
    print(f"generated rust files: {len(ALL_GENERATED)}")
    print(f"generated structs: {struct_count}")
    print(f"hand-written registered: {len(ALL_HAND_WRITTEN_REGISTERED)} -> {ALL_HAND_WRITTEN_REGISTERED}")
    print(f"stale placeholders removed: {stale_count}")
    print(f"skipped: {len(ALL_SKIPPED)}")
    for s in ALL_SKIPPED:
        print("  skip:", s)


if __name__ == "__main__":
    main()
