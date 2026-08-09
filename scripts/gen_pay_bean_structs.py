#!/usr/bin/env python3
"""weixin-java-pay 数据 bean 批量生成器（WxRust 迁移 Wave 1 Agent P1）。

由 `scripts/gen_miniapp_bean_structs.py` 复制改造而来，服务于
crates/wx-rust-pay（对应 Java `com.github.binarywang.wxpay.bean` 包）。

保留 miniapp 生成器全部机制：
- 继承扁平化（flatten_fields，跨文件/跨包父类经全局 cache 递归合并，父类字段在前）
- 内嵌类（static class → 同文件多 struct）
- `use super::*;` + 祖先包通配导入（多级子目录逐层导入）
- 每目录 mod.rs（pub mod + pub use 按名去重转发），根 bean/mod.rs 汇总声明
- HAND_WRITTEN 保护集（Wave 0 已定型对象，禁止覆盖）
- POST_PROCESS（from_json / to_json 辅助，幂等）
- `#[allow(unused_imports)]`、`#[serde(rename = ...)]`、alternate → alias

pay 特有扩展：
1. **XML 线格式**：v2 请求/结果（extends BaseWxPayRequest/BaseWxPayResult）与
   部分通知（WxPayNotifyResponse）走 XML（Java XStream）。Rust 以 quick-xml
   serde 支持（`quick_xml::de::from_str` / `quick_xml::se::to_string`），
   元素名 = Java 字段名（@XStreamAlias 值），根元素统一 `#[serde(rename = "xml")]`；
   Option 字段 `skip_serializing_if = "Option::is_none"`（XStream/Gson 均省略 null）。
   所有 XML 类自动附加 `from_xml`/`to_xml` 辅助（BaseWxPayResult 子类保留
   `xml_string` 原始报文，对应 Java `setXmlString`）。
2. **动态组合字段**（XML 中 `coupon_0/coupon_1`、`refund_*_0` 等索引式字段，
   Java 在 `fromXML` 后由 `composeXxx()` 从原始 XML 组装）：见 `XML_COMPOSE`
   手写 impl 模板，幂等追加（含 `to_map()` 供签名校验，对应 Java `BaseWxPayResult.toMap`）。
3. **跨目录类型引用**：自动解析字段类型所属 Java 文件，追加
   `use crate::bean::<dir>::<Type>;`（如 transfer 包引用 notify 包的
   `OriginNotifyResponse`）。
4. **Java 枚举字段 → String**：pay 的 bean/enums 枚举类不生成（迁移至
   enums/ 或线格式字符串），字段按 Gson/@SerializedName 语义映射为 String。
5. **abstract 顶层类**（BaseWxPayRequest/BaseWxPayResult/BaseWxPayV3Result）
   不产出独立 struct（字段仍经继承扁平化合并进子类）。

排除：枚举类（is_enum）。Wave 0 已定型对象（WxPayApiData、SignatureHeader）
进入 HAND_WRITTEN（不覆盖，注册既有手写文件）。

v3 通知解密（AES-GCM）、v2 退款通知 req_info 解密（AES-256-ECB）留 Wave 2：
生成的 `from_xml` 只做结构解析，解密函数以注释标注 TODO。
"""
import os
import re
import sys

BASE = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
JAVA_BEAN = "/Users/wandl/workspaces/workspace-github/WxJava/weixin-java-pay/src/main/java/com/github/binarywang/wxpay/bean"
RUST_BEAN = os.path.join(BASE, "crates", "wx-rust-pay", "src", "bean")
JAVA_PKG = "com.github.binarywang.wxpay.bean"

# 字段声明：private/protected/public（pay 的基类与个别 v3 类用 protected 修饰字段，
# 如 BaseWxPayRequest、WxPayOrderReverseV3Request/Result；mipay 的
# MedInsOrdersRequest/Result 全用 public 字段）
FIELD_RE = re.compile(r"^\s*(?:private|protected|public)\s+(?!static\s+final\s+)([\w<>\[\],. ]+?)\s+(\w+)\s*(?:=\s*[^;]*)?;")

# Java `transient` 字段标记（Wave 2 修复）：transient 字段在 Java 类中保留
# （getter/setter 可用，供门面构造 URL 等），但 Gson/XStream 序列化均跳过。
# 生成器以该哨兵记录字段，gen_struct 输出 `#[serde(skip)]` 字段（线格式不含）。
# 此前（Wave 1）transient 字段被整体丢弃，导致 WxPayOrderCloseV3Request /
# WxPayPartnerOrderCloseV3Request / CombineCloseRequest / WxPayOrderReverseV3Request
# 的 out_trade_no（combine_out_trade_no）缺失，门面 9 方法无法构造 URL。
TRANS_SKIP = "@serde-skip"
# 类声明行：捕获类名、泛型参数与可选 extends 父类（同一行声明；implements
# 列表可跨行（`implements Serializable,\n X {...}`），故结尾允许无 `{`，
# 以行尾锚定保证 extends 惰性匹配延伸至整行（否则父类会被截断）。
CLASS_RE = re.compile(
    r"^\s*(?:public\s+|private\s+)?(?:abstract\s+|final\s+|static\s+)*class\s+(\w+)"
    r"(?:<([^>]*)>)?"
    r"(?:\s+extends\s+([\w<>,.]+?))?"
    r"(?:\s+implements\s+[\w<>,. ]+,?)?"
    r"\s*\{?\s*$"
)

# pay Java 枚举类型：不生成 Rust 枚举，字段映射为线格式字符串
# （Java Gson 序列化枚举为 @SerializedName 值；v2 XML 无枚举字段）。
PAY_ENUM_TYPES = {
    # applyment/enums
    "AccountTypeEnum", "ApplymentStateEnum", "BankAccountTypeEnum", "CertTypeEnum",
    "FinanceTypeEnum", "IdTypeEnum", "MicroBizTypeEnum", "SalesScenesTypeEnum",
    "SettlementVerifyResultEnum", "SettlementVerifyStateEnum", "SubjectTypeEnum",
    # applyconfirm/enums
    "ApplySubjectStateEnum", "AuthorizeStateEnum",
    # ecommerce/enums
    "FundBillTypeEnum", "SpAccountTypeEnum",
    # marketing/enums
    "BackgroundColorEnum", "JumpTargetEnum", "StockTypeEnum",
    # marketing/enums/TradeTypeEnum（与 result/enums/TradeTypeEnum 同名，此处为营销枚举）
    "TradeTypeEnum",
    # mipay/enums
    "CashAddTypeEnum", "CashReduceTypeEnum", "MedInsPayStatusEnum", "MixPayStatusEnum",
    "MixPayTypeEnum", "OrderTypeEnum", "SelfPayStatusEnum", "UserCardTypeEnum",
    # payscore/enums
    "SignPlanServiceOrderPlanDetailStateEnum", "SignPlanServiceOrderStateEnum",
    "UserSignPlanCancelSignTypeEnum",
}

# 类型映射遵循 Java 可空语义：引用类型（String/Integer/Long/...）→ Option<...>，
# 原始类型（int/long/boolean/...）→ 非 Option。Java Gson/XStream 序列化均省略
# null 字段（Rust `skip_serializing_if = "Option::is_none"`），缺失字段解析为
# None（Java null），与线格式一致。
TYPE_MAP = {
    "String": "Option<String>",
    "int": "i32", "Integer": "Option<i32>",
    "long": "i64", "Long": "Option<i64>",
    "boolean": "bool", "Boolean": "Option<bool>",
    "double": "f64", "Double": "Option<f64>",
    "float": "f32", "Float": "Option<f32>",
    "short": "i16", "Short": "Option<i16>",
    "byte": "u8", "Byte": "Option<u8>",
    "char": "char", "Character": "Option<char>",
    "Object": "Option<serde_json::Value>",
    "Date": "Option<String>",  # Java Gson 输出 ISO 8601 字符串，线格式原样保留
    "LocalDate": "Option<String>",
    "LocalDateTime": "Option<String>",
    "BigDecimal": "Option<String>",  # 金额精度保留原串
    "BigInteger": "Option<String>",
    "Long[]": "Vec<i64>",
    "String[]": "Vec<String>",
    "Integer[]": "Vec<i32>",
    "Map<String, Object>": "std::collections::HashMap<String, serde_json::Value>",
    "Map<String, String>": "std::collections::HashMap<String, String>",
    "Map<String, Integer>": "std::collections::HashMap<String, i32>",
    "Map<Integer, String>": "std::collections::HashMap<i32, String>",
    "Map<String, Long>": "std::collections::HashMap<String, i64>",
    "Map<Integer, Integer>": "std::collections::HashMap<i32, i32>",
    "Map<String, List<String>>": "std::collections::HashMap<String, Vec<String>>",
    "JsonArray": "serde_json::Value",
    "JsonObject": "serde_json::Value",
    "Collection": "serde_json::Value",  # payscore/WxPayScoreResult.collection（原始集合）
}
for _e in PAY_ENUM_TYPES:
    TYPE_MAP.setdefault(_e, "String")  # Java 枚举 → 线格式字符串（@SerializedName 值）

# v2 XML 基类（extends 链上命中即 XML 类）
XML_BASE_PARENTS = {"BaseWxPayRequest", "BaseWxPayResult"}
# 不继承基类但走 XML 的类（@XStreamAlias 检测的补充）
XML_EXPLICIT = {
    "notify/WxPayNotifyResponse.java",
}


def map_type(t: str, classes: set, type_params: set) -> str:
    t = t.strip()
    if t.startswith("final "):
        t = t[len("final "):]
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
        return "serde_json::Value"  # 泛型类型参数
    if "." in t:
        # 跨文件内嵌类引用（如 ReservationTransferNotifyResult.DecryptNotifyResult）：
        # 内嵌类在生成时是各自模块的顶层 struct，经 mod.rs pub use 按简单名可达
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
    - flags：dict(is_enum, is_abstract, has_from_json, has_to_json)
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
    is_abstract = False
    # Java 类作用域栈（括号深度追踪，Wave 2 修复）：内嵌类声明后，其后的字段
    # 应归属回外层类。此前（Wave 1）不追踪括号深度，内嵌类之后的字段被误归入
    # 内嵌类（如 WxPayApplyFundFlowBillV3Request / WxPayDownloadFundFlowRequest
    # 的 bill_date/account_type/tar_type 被生成到常量类 AccountType，
    # WxPayRefundV3Request 的 sub_mchid 被生成到 GoodsDetail）。
    scope_stack = []  # (类名, 类体括号深度)
    depth = 0
    with open(path, encoding="utf-8") as f:
        for line in f:
            if "public enum" in line or re.search(r"^\s*(?:public\s+)?enum\s+", line):
                is_enum = True
            # 允许类声明行带前置注解（如 `@Data public class X {`）
            mline = re.sub(r"^\s*(?:@[^\s@]+\s*)+", "", line)
            cm = CLASS_RE.match(mline)
            if cm:
                name = cm.group(1)
                if "abstract" in line:
                    is_abstract = True
                if cm.group(2):
                    for p in re.split(r"\s*,\s*", cm.group(2)):
                        p = p.strip()
                        if p:
                            type_params.add(re.split(r"\s+", p)[0])
                parent = cm.group(3)
                if parent and "Serializable" not in parent and "Object" not in parent:
                    parent = re.split(r"[<,]", parent)[0].strip()
                    parents[name] = parent
                depth += line.count("{") - line.count("}")
                scope_stack.append((name, depth))
                cur = name
                if name not in classes:
                    classes[name] = []
                    order.append(name)
                continue
            depth += line.count("{") - line.count("}")
            # 括号深度低于类体深度 → 该类已结束，字段归属退回外层类
            while scope_stack and depth < scope_stack[-1][1]:
                scope_stack.pop()
            cur = scope_stack[-1][0] if scope_stack else None
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
            sm = re.search(r'@SerializedName\((?:value\s*=\s*)?"([^"]+)"', line)
            if sm:
                pending_serialized = sm.group(1)
                alt = re.search(r'alternate\s*=\s*"([^"]+)"', line)
                if alt:
                    pending_serialized = pending_serialized + "|" + alt.group(1)
                continue
            # v2 XML 类字段名覆盖（Java XStream `@XStreamAlias("wire_name")`，
            # 值即 XML 元素名，如 `@XStreamAlias("mch_id")`）
            xm = re.search(r'@XStreamAlias\(\s*(?:value\s*=\s*)?"([^"]+)"', line)
            if xm:
                pending_serialized = xm.group(1)
                continue
            fm = FIELD_RE.match(line)
            if fm:
                ftype, fname = fm.group(1).strip(), fm.group(2)
                if pending_serialized == "!skip":
                    pending_serialized = None
                    continue
                if "transient" in line:
                    # Java transient：类字段保留、线格式跳过 → #[serde(skip)]。
                    # 类型须剥离修饰符（`private transient String x;` 的
                    # FIELD_RE 捕获类型为 "transient String"）。
                    classes[cur].append((ftype.replace("transient", " ").strip(), fname, TRANS_SKIP))
                    pending_serialized = None
                    continue
                classes[cur].append((ftype, fname, pending_serialized))
                pending_serialized = None
    flags = {
        "is_enum": is_enum,
        "is_abstract": is_abstract,
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
    if name == "self":
        return "self_"  # `self` 不允许 raw identifier（r#self 非法）
    return f"r#{name}" if name in keywords else name


def java_type_is_class(ftype: str) -> bool:
    """Java 字段类型是否为直接类引用（需要 Option 包装，Java 引用类型可空）。

    List/Map/数组/基本类型/标量映射（TYPE_MAP）返回 False（其可空语义已
    在 TYPE_MAP 中表达；List/Map 元素恒在，不做包装）。
    """
    t = ftype.strip()
    if t.startswith("final "):
        t = t[len("final "):]
    if t in TYPE_MAP or re.match(r"(?:List|Collection|Map)<", t) or t.endswith("[]"):
        return False
    base = t.split(".")[-1].strip()
    if base in ("int", "long", "boolean", "double", "float", "short", "byte", "char"):
        return False
    return True


def gen_struct(name, fields, classes, type_params, xml=False, indent=""):
    lines = []
    lines.append(f"{indent}#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]")
    if xml:
        # 根元素名（Java XStream `@XStreamAlias("xml")`），quick-xml 序列化使用。
        # 注意：serde 容器属性须位于 derive 之后（derive helper 顺序约束）。
        lines.append(f"{indent}#[serde(rename = \"xml\")]")
    lines.append(f"{indent}pub struct {name} {{")
    used_names = set()
    for ftype, fname, serialized in fields:
        rt = map_type(ftype, set(classes.keys()), type_params)
        if java_type_is_class(ftype) and not rt.startswith("Option<"):
            rt = f"Option<{rt}>"  # Java 引用类型可空 → Option（缺失/省略语义与线格式一致）
        if serialized == TRANS_SKIP:
            # Java transient 字段：类字段保留（供门面构造 URL 等），
            # Gson/XStream 序列化均跳过 → #[serde(skip)]（线格式不含）。
            lines.append(
                f"{indent}    /// Java `transient` 字段：类字段保留，但 Gson/XStream"
                f" 线格式跳过（不含此键）。"
            )
            lines.append(f"{indent}    #[serde(skip)]")
            lines.append(f"{indent}    pub {rust_ident(snake(fname))}: {rt},")
            continue
        json_name = serialized.split("|")[0] if serialized else fname
        if json_name in used_names:
            # 同名线格式键冲突（Java 中组合字段与线字段同名，如
            # ProfitSharingQueryResult 的 `receivers`（JSON 串）与 `receivers`
            # （List，compose 组装，不在线格式中））：后者 serde(skip)，
            # 线格式键归前者（XStream 同名字段组合字段不参与线格式解析）。
            lines.append(
                f"{indent}    /// 组合字段（线格式键与上文字段同名，serde(skip)，"
                f"由 compose 逻辑组装）。"
            )
            lines.append(f"{indent}    #[serde(skip)]")
            lines.append(f"{indent}    pub {rust_ident(snake(fname))}: {rt},")
            continue
        used_names.add(json_name)
        opts = ["default"]
        if rt.startswith("Option<"):
            # Java XStream/Gson 序列化均省略 null 字段
            opts.append('skip_serializing_if = "Option::is_none"')
        if serialized and "|" in serialized:
            primary, alternate = serialized.split("|", 1)
            opts.append(f'rename = "{primary}"')
            lines.append(f"{indent}    #[serde({', '.join(opts)}, alias = \"{alternate}\")]")
        else:
            opts.append(f'rename = "{json_name}"')
            lines.append(f"{indent}    #[serde({', '.join(opts)})]")
        lines.append(f"{indent}    pub {rust_ident(snake(fname))}: {rt},")
    lines.append(f"{indent}}}")
    return "\n".join(lines)


# ---------------------------------------------------------------------------
# 全局类缓存：整个 bean 目录树的类（含跨文件/跨包父类），用于继承扁平化。
# 按「文件名 + 类名」定位（Java 内嵌类简单名可能跨文件/跨包重复）。
# ---------------------------------------------------------------------------
GLOBAL_CACHE = {}
FILE_CLASSES = {}


def build_global_cache():
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
    """递归合并父类字段（父类在前，对应 XStream/Gson 反射序列化层级顺序）。

    子类重声明父类字段（Java 字段遮蔽，如 entpay 各请求/结果重声明
    `mchId`）时**后声明者胜**（Java 语义：子类字段遮蔽父类，序列化只输出一份）。
    """
    seen = seen or set()
    if name in seen:
        return []
    seen.add(name)
    info = resolve_class(name, cur_file)
    if not info:
        return []
    parent = info["parent"]
    if not parent or parent in ("Object", "Serializable"):
        fields = list(info["fields"])
    else:
        fields = flatten_fields(parent, cur_file, seen) + list(info["fields"])
    deduped = {}
    for f in fields:
        deduped[snake(f[1])] = f  # 后声明者覆盖（遮蔽语义）
    return list(deduped.values())


def parent_chain(name, cur_file):
    """返回从 name 到根的全部祖先类名（含自身），用于 XML 类判定。"""
    chain = []
    seen = set()
    while name and name not in seen:
        seen.add(name)
        chain.append(name)
        info = resolve_class(name, cur_file)
        if not info:
            break
        name = info["parent"]
    return chain


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


# 字段类型标识符提取（去掉泛型/数组/包前缀），用于跨目录引用解析
_IDENT_RE = re.compile(r"[A-Za-z_][A-Za-z0-9_]*")


def type_idents(ftype: str):
    """从 Java 字段类型提取顶层标识符集合（`List<X.Y>` → {List, X}）。"""
    return set(_IDENT_RE.findall(ftype))


def cross_imports(cur_rel, body):
    """计算本文件需要的跨目录类型导入（Java 类型在其它目录/根包定义的）。

    同类名引用在本地模块（同文件内嵌类）或经 `use super::*` 可达（同目录
    mod.rs 转发）；仅当解析出的定义文件与本文件不在同一目录时才需显式导入
    `use crate::bean::<dir>::<Type>;`（经由目标目录 mod.rs 的 pub use 转发）。
    """
    imports = set()
    cur_dir = os.path.dirname(cur_rel)
    # 扫描生成体里出现的所有标识符，仅保留可能是 Java 类名的
    for ident in _IDENT_RE.findall(body):
        if ident in TYPE_MAP:
            continue
        cand = GLOBAL_CACHE.get(ident)
        if not cand:
            continue
        # 本文件/本目录内定义的类不需要导入（本地或 super::* 可达）
        same_file = any(e["file"] == cur_rel for e in cand)
        same_dir = any(os.path.dirname(e["file"]) == cur_dir for e in cand)
        if same_file or same_dir:
            continue
        # 定义文件所在目录 → crate::bean::<dir>::<Type>（目录路径以 :: 连接）
        tgt_dir = os.path.dirname(cand[0]["file"])
        if tgt_dir:
            imports.add(f"use crate::bean::{tgt_dir.replace('/', '::')}::{ident};")
        else:
            imports.add(f"use crate::bean::{ident};")
    return sorted(imports)


def gen_file(java_path, rust_rel):
    """为单个 Java 文件生成 Rust 文件；纯数据类才生成。返回 (content, structs, is_xml) 或 None。"""
    order, classes, parents, flags, type_params = parse_java(java_path)
    if flags["is_enum"] or not order:
        return None
    top = order[0]
    top_is_abstract = flags["is_abstract"] and not parents  # 仅顶层抽象类（无父类）
    # 空壳类（无字段、无父类、无内嵌类）无数据可迁移
    if (
        not classes.get(top)
        and not resolve_class(top, rust_rel).get("parent")
        and len(order) == 1
    ):
        return None
    with open(java_path, encoding="utf-8") as f:
        java_text = f.read()
    is_xml = (
        "XStreamAlias" in java_text
        or rust_rel in XML_EXPLICIT
        or any(
            p in XML_BASE_PARENTS
            for p in parent_chain(top, rust_rel)
        )
    )
    body_parts = []
    if not top_is_abstract:
        top_fields = flatten_fields(top, rust_rel)
        top_params = collect_params(top, rust_rel) | type_params
        body_parts.append(gen_struct(top, top_fields, classes, top_params, xml=is_xml))
    for inner in order[1:]:
        inner_fields = flatten_fields(inner, rust_rel)
        inner_params = collect_params(inner, rust_rel) | type_params
        body_parts.append(gen_struct(inner, inner_fields, classes, inner_params, xml=False))
    if not body_parts:
        return None
    body = "\n\n".join(body_parts)
    structs = re.findall(r"pub struct (\w+)", body)
    java_class = f"{JAVA_PKG}.{rust_rel.replace('/', '.')}"
    imports = cross_imports(rust_rel, body)
    import_lines = "\n".join(f"#[allow(unused_imports)]\n{i}" for i in imports)
    content = f"""//! 对应 Java `{java_class}`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;
{import_lines}
{body}
"""
    return content, structs, is_xml


# ---------------------------------------------------------------------------
# 手写版本（Wave 0 已定型 / 复杂逻辑，生成器禁止覆盖）
# ---------------------------------------------------------------------------
HAND_WRITTEN = {
    # Wave 0 已定型对象（bean 根与 notify 模块，手写文件保留并注册进 mod.rs）
    'WxPayApiData.java',
    'notify/SignatureHeader.java',
}

# 生成后追加的辅助 impl（from_json/to_json 语义；自动检测 + 显式登记，幂等）
# 键：相对 bean 根的 rust 文件路径；值：("from_json_list", "list键") 或 ("from_json",) 等
POST_PROCESS = {
    "media/image_upload_result.rs": ("from_json",),
    "media/marketing_image_upload_result.rs": ("from_json",),
    "media/video_upload_result.rs": ("from_json",),
    "payscore/partner_user_sign_plan_entity.rs": ("from_json",),
    "payscore/wx_partner_pay_score_result.rs": ("from_json",),
    "payscore/wx_partner_pay_score_sign_plan_result.rs": ("from_json",),
    "payscore/wx_partner_pay_score_user_sign_plan_result.rs": ("from_json",),
    "payscore/wx_pay_score_result.rs": ("from_json",),
}

# 需要 Java 组合逻辑的 XML 类：手写 impl 模板（幂等追加；对应 Java
# composeXxx()/toMap()/fromXML 的精确语义）。键为生成文件路径。
XML_COMPOSE = {
    # ---- 支付结果通知：coupon_*_i 索引字段 → coupon_list（Java composeCoupons）----
    "notify/wx_pay_order_notify_result.rs": '''
/// 支付结果通知的组合逻辑（对应 Java `WxPayOrderNotifyResult` 的组合方法）。
impl WxPayOrderNotifyResult {
    /// 从 XML 解析（对应 Java `fromXML`）：结构字段 + 原始报文 + 代金券列表组装。
    pub fn from_xml(xml: &str) -> Result<Self, String> {
        let mut v: Self = quick_xml::de::from_str(xml)
            .map_err(|e| format!("WxPayOrderNotifyResult 解析失败: {e}"))?;
        v.xml_string = Some(xml.to_string());
        v.compose_coupons();
        Ok(v)
    }

    /// 通过原始 XML 组装 `coupon_list`（对应 Java `composeCoupons`：
    /// `xml/coupon_id_{i}`/`coupon_type_{i}`/`coupon_fee_{i}`）。
    pub fn compose_coupons(&mut self) {
        let count = self.coupon_count.unwrap_or(0);
        if count <= 0 {
            return;
        }
        let Some(xml) = self.xml_string.as_deref() else { return };
        let map = match crate::bean::xml::root_children_map(xml) {
            Ok(m) => m,
            Err(_) => return,
        };
        self.coupon_list = (0..count)
            .map(|i| WxPayOrderNotifyCoupon {
                coupon_id: map.get(&format!("coupon_id_{i}")).cloned(),
                coupon_type: map.get(&format!("coupon_type_{i}")).cloned(),
                coupon_fee: map.get(&format!("coupon_fee_{i}")).and_then(|s| s.trim().parse().ok()),
            })
            .collect();
    }

    /// 将原始 XML 全部字段转为 map（对应 Java `toMap`：`/xml/*` + 代金券字段），
    /// 供签名校验使用。
    pub fn to_map(&self) -> Result<std::collections::HashMap<String, String>, String> {
        let xml = self
            .xml_string
            .as_deref()
            .ok_or_else(|| "xml 数据有问题，请核实！".to_string())?;
        let mut m = crate::bean::xml::root_children_map(xml)?;
        for (i, coupon) in self.coupon_list.iter().enumerate() {
            m.extend(coupon.to_map(i));
        }
        Ok(m)
    }
}
''',
    # ---- 代金券：toMap(index)（Java WxPayOrderNotifyCoupon.toMap）----
    "notify/wx_pay_order_notify_coupon.rs": '''
/// 代金券组合辅助（对应 Java `WxPayOrderNotifyCoupon.toMap(int index)`）。
impl WxPayOrderNotifyCoupon {
    /// 以 `coupon_id_{index}`/`coupon_type_{index}`/`coupon_fee_{index}` 键导出。
    pub fn to_map(&self, index: usize) -> std::collections::HashMap<String, String> {
        let mut map = std::collections::HashMap::new();
        if let Some(v) = self.coupon_id.as_deref() {
            map.insert(format!("coupon_id_{index}"), v.to_string());
        }
        if let Some(v) = self.coupon_type.as_deref() {
            map.insert(format!("coupon_type_{index}"), v.to_string());
        }
        map.insert(format!("coupon_fee_{index}"), format!("{}", self.coupon_fee.unwrap_or_default()));
        map
    }
}
''',
    # ---- 退款结果通知：req_info 解密留 Wave 2 ----
    "notify/wx_pay_refund_notify_result.rs": '''
/// 退款结果通知（对应 Java `WxPayRefundNotifyResult`）。
impl WxPayRefundNotifyResult {
    /// 从 XML 解析（对应 Java `fromXML`）：仅结构解析，不涉及 req_info 解密。
    pub fn from_xml(xml: &str) -> Result<Self, String> {
        let mut v: Self = quick_xml::de::from_str(xml)
            .map_err(|e| format!("WxPayRefundNotifyResult 解析失败: {e}"))?;
        v.xml_string = Some(xml.to_string());
        Ok(v)
    }

    /// 解密并解析 `req_info`（对应 Java `decryptReqInfo(String mchKey)`：
    /// `md5(mchKey)` 作为 AES-256-ECB 密钥解密 `req_info` 的 Base64 内容）。
    ///
    /// **Wave 2 实现**：AES-256-ECB 解密 + `ReqInfo::from_xml`；当前返回未实现错误。
    pub fn decrypt_req_info(&mut self, mch_key: &str) -> Result<(), String> {
        let _ = mch_key;
        Err("WxPayRefundNotifyResult::decrypt_req_info 未实现（Wave 2：AES-256-ECB 解密 req_info）".to_string())
    }
}
''',
    # ---- 查询订单：coupon 组合（Java composeCoupons）----
    "result/wx_pay_order_query_result.rs": '''
/// 查询订单结果的组合逻辑（对应 Java `WxPayOrderQueryResult`）。
impl WxPayOrderQueryResult {
    /// 从 XML 解析（对应 Java `fromXML`）。
    pub fn from_xml(xml: &str) -> Result<Self, String> {
        let mut v: Self = quick_xml::de::from_str(xml)
            .map_err(|e| format!("WxPayOrderQueryResult 解析失败: {e}"))?;
        v.xml_string = Some(xml.to_string());
        v.compose_coupons();
        Ok(v)
    }

    /// 组装 `coupons`（对应 Java `composeCoupons`：`xml/coupon_type_{i}` 等）。
    pub fn compose_coupons(&mut self) {
        let count = self.coupon_count.unwrap_or(0);
        if count <= 0 {
            return;
        }
        let Some(xml) = self.xml_string.as_deref() else { return };
        let map = match crate::bean::xml::root_children_map(xml) {
            Ok(m) => m,
            Err(_) => return,
        };
        self.coupons = (0..count)
            .map(|i| Coupon {
                coupon_type: map.get(&format!("coupon_type_{i}")).cloned(),
                coupon_id: map.get(&format!("coupon_id_{i}")).cloned(),
                coupon_fee: map.get(&format!("coupon_fee_{i}")).and_then(|s| s.trim().parse().ok()),
            })
            .collect();
    }
}
''',
    # ---- 查询退款：refund 记录 + 营销详情组合 ----
    "result/wx_pay_refund_query_result.rs": '''
/// 查询退款结果的组合逻辑（对应 Java `WxPayRefundQueryResult`）。
impl WxPayRefundQueryResult {
    /// 从 XML 解析（对应 Java `fromXML`）。
    pub fn from_xml(xml: &str) -> Result<Self, String> {
        let mut v: Self = quick_xml::de::from_str(xml)
            .map_err(|e| format!("WxPayRefundQueryResult 解析失败: {e}"))?;
        v.xml_string = Some(xml.to_string());
        v.compose_refund_records();
        v.compose_promotion_details();
        Ok(v)
    }

    /// 组装 `refund_records`（对应 Java `composeRefundRecords`：
    /// `xml/refund_*_{i}` 与嵌套 `xml/coupon_refund_*_{i}_{j}` 索引字段）。
    pub fn compose_refund_records(&mut self) {
        let count = self.refund_count.unwrap_or(0);
        if count <= 0 {
            return;
        }
        let Some(xml) = self.xml_string.as_deref() else { return };
        let map = match crate::bean::xml::root_children_map(xml) {
            Ok(m) => m,
            Err(_) => return,
        };
        let mut records = Vec::with_capacity(count as usize);
        for i in 0..count {
            let coupon_refund_count =
                map.get(&format!("coupon_refund_count_{i}")).and_then(|s| s.trim().parse::<i32>().ok());
            let mut coupons = Vec::new();
            if let Some(c) = coupon_refund_count {
                if c > 0 {
                    for j in 0..c {
                        coupons.push(WxPayRefundCouponInfo {
                            coupon_refund_id: map.get(&format!("coupon_refund_id_{i}_{j}")).cloned(),
                            coupon_refund_fee: map
                                .get(&format!("coupon_refund_fee_{i}_{j}"))
                                .and_then(|s| s.trim().parse().ok()),
                            coupon_type: map.get(&format!("coupon_type_{i}_{j}")).cloned(),
                        });
                    }
                }
            }
            records.push(RefundRecord {
                out_refund_no: map.get(&format!("out_refund_no_{i}")).cloned(),
                refund_id: map.get(&format!("refund_id_{i}")).cloned(),
                refund_channel: map.get(&format!("refund_channel_{i}")).cloned(),
                refund_fee: map.get(&format!("refund_fee_{i}")).and_then(|s| s.trim().parse().ok()),
                settlement_refund_fee: map
                    .get(&format!("settlement_refund_fee_{i}"))
                    .and_then(|s| s.trim().parse().ok()),
                coupon_refund_fee: map
                    .get(&format!("coupon_refund_fee_{i}"))
                    .and_then(|s| s.trim().parse().ok()),
                coupon_refund_count,
                refund_account: map.get(&format!("refund_account_{i}")).cloned(),
                refund_status: map.get(&format!("refund_status_{i}")).cloned(),
                refund_recv_account: map.get(&format!("refund_recv_accout_{i}")).cloned(),
                refund_success_time: map.get(&format!("refund_success_time_{i}")).cloned(),
                refund_coupons: coupons,
            });
        }
        self.refund_records = records;
    }

    /// 解析 `promotion_detail`（XML 元素内嵌 JSON 串，对应 Java `composePromotionDetails`：
    /// 取 JSON 的 `promotion_detail` 数组）。
    pub fn compose_promotion_details(&mut self) {
        let Some(s) = self.promotion_detail_string.as_deref() else { return };
        if s.is_empty() {
            return;
        }
        if let Ok(v) = serde_json::from_str::<serde_json::Value>(s) {
            if let Some(arr) = v.get("promotion_detail") {
                if let Ok(list) = serde_json::from_value::<Vec<WxPayRefundPromotionDetail>>(arr.clone()) {
                    self.promotion_details = list;
                }
            }
        }
    }
}
''',
    # ---- 退款结果：营销详情 + 退款代金券组合 ----
    "result/wx_pay_refund_result.rs": '''
/// 退款结果的组合逻辑（对应 Java `WxPayRefundResult`）。
impl WxPayRefundResult {
    /// 从 XML 解析（对应 Java `fromXML`）。
    pub fn from_xml(xml: &str) -> Result<Self, String> {
        let mut v: Self = quick_xml::de::from_str(xml)
            .map_err(|e| format!("WxPayRefundResult 解析失败: {e}"))?;
        v.xml_string = Some(xml.to_string());
        v.compose_promotion_details();
        v.compose_refund_coupons();
        Ok(v)
    }

    /// 解析 `promotion_detail`（内嵌 JSON 串，对应 Java `composePromotionDetails`）。
    pub fn compose_promotion_details(&mut self) {
        let Some(s) = self.promotion_detail_string.as_deref() else { return };
        if s.is_empty() {
            return;
        }
        if let Ok(v) = serde_json::from_str::<serde_json::Value>(s) {
            if let Some(arr) = v.get("promotion_detail") {
                if let Ok(list) = serde_json::from_value::<Vec<WxPayRefundPromotionDetail>>(arr.clone()) {
                    self.promotion_details = list;
                }
            }
        }
    }

    /// 组装 `refund_coupons`（对应 Java `composeRefundCoupons`：
    /// `xml/coupon_refund_id_{i}`/`coupon_refund_fee_{i}`/`coupon_type_{i}`）。
    pub fn compose_refund_coupons(&mut self) {
        let count = self.coupon_refund_count.unwrap_or(0);
        if count <= 0 {
            return;
        }
        let Some(xml) = self.xml_string.as_deref() else { return };
        let map = match crate::bean::xml::root_children_map(xml) {
            Ok(m) => m,
            Err(_) => return,
        };
        self.refund_coupons = (0..count)
            .map(|i| WxPayRefundCouponInfo {
                coupon_refund_id: map.get(&format!("coupon_refund_id_{i}")).cloned(),
                coupon_refund_fee: map.get(&format!("coupon_refund_fee_{i}")).and_then(|s| s.trim().parse().ok()),
                coupon_type: map.get(&format!("coupon_type_{i}")).cloned(),
            })
            .collect();
    }
}
''',
    # ---- 委托代扣签约通知（XML）：coupon 组合 ----
    "result/wx_sign_status_notify_result.rs": '''
/// 签约状态变更通知（XML，对应 Java `WxSignStatusNotifyResult`）。
impl WxSignStatusNotifyResult {
    /// 从 XML 解析（对应 Java `fromXML`）。
    pub fn from_xml(xml: &str) -> Result<Self, String> {
        let mut v: Self = quick_xml::de::from_str(xml)
            .map_err(|e| format!("WxSignStatusNotifyResult 解析失败: {e}"))?;
        v.xml_string = Some(xml.to_string());
        Ok(v)
    }
}
''',
    # ---- 委托代扣支付结果通知（XML）：coupon 组合 ----
    "result/wx_withhold_notify_result.rs": '''
/// 委托代扣支付结果通知（XML，对应 Java `WxWithholdNotifyResult`）。
impl WxWithholdNotifyResult {
    /// 从 XML 解析（对应 Java `fromXML`：结构字段 + 原始报文 + 代金券列表组装）。
    pub fn from_xml(xml: &str) -> Result<Self, String> {
        let mut v: Self = quick_xml::de::from_str(xml)
            .map_err(|e| format!("WxWithholdNotifyResult 解析失败: {e}"))?;
        v.xml_string = Some(xml.to_string());
        v.compose_coupons();
        Ok(v)
    }

    /// 组装 `coupon_list`（对应 Java `composeCoupons`）。
    pub fn compose_coupons(&mut self) {
        let count = self.coupon_count.unwrap_or(0);
        if count <= 0 {
            return;
        }
        let Some(xml) = self.xml_string.as_deref() else { return };
        let map = match crate::bean::xml::root_children_map(xml) {
            Ok(m) => m,
            Err(_) => return,
        };
        self.coupon_list = (0..count)
            .map(|i| WxPayOrderNotifyCoupon {
                coupon_id: map.get(&format!("coupon_id_{i}")).cloned(),
                coupon_type: map.get(&format!("coupon_type_{i}")).cloned(),
                coupon_fee: map.get(&format!("coupon_fee_{i}")).and_then(|s| s.trim().parse().ok()),
            })
            .collect();
    }
}
''',
    # ---- 委托代扣查询订单结果（XML）：coupon 组合 ----
    "result/wx_withhold_order_query_result.rs": '''
/// 委托代扣查询订单结果（XML，对应 Java `WxWithholdOrderQueryResult`）。
impl WxWithholdOrderQueryResult {
    /// 从 XML 解析（对应 Java `fromXML`）。
    pub fn from_xml(xml: &str) -> Result<Self, String> {
        let mut v: Self = quick_xml::de::from_str(xml)
            .map_err(|e| format!("WxWithholdOrderQueryResult 解析失败: {e}"))?;
        v.xml_string = Some(xml.to_string());
        v.compose_coupons();
        Ok(v)
    }

    /// 组装 `coupon_list`（对应 Java `composeCoupons`）。
    pub fn compose_coupons(&mut self) {
        let count = self.coupon_count.unwrap_or(0);
        if count <= 0 {
            return;
        }
        let Some(xml) = self.xml_string.as_deref() else { return };
        let map = match crate::bean::xml::root_children_map(xml) {
            Ok(m) => m,
            Err(_) => return,
        };
        self.coupon_list = (0..count)
            .map(|i| WxPayOrderNotifyCoupon {
                coupon_id: map.get(&format!("coupon_id_{i}")).cloned(),
                coupon_type: map.get(&format!("coupon_type_{i}")).cloned(),
                coupon_fee: map.get(&format!("coupon_fee_{i}")).and_then(|s| s.trim().parse().ok()),
            })
            .collect();
    }
}
''',
    # ---- v3 通知响应（JSON）：SUCCESS/FAIL 构造（Java WxPayNotifyV3Response）----
    "notify/wx_pay_notify_v3_response.rs": '''
/// v3 通知响应构造（对应 Java `WxPayNotifyV3Response` 静态方法，JSON）。
impl WxPayNotifyV3Response {
    /// 成功响应 JSON（对应 Java `success(String msg)`）。
    pub fn success(msg: &str) -> String {
        serde_json::json!({ "code": "SUCCESS", "message": msg }).to_string()
    }

    /// 失败响应 JSON（对应 Java `fail(String msg)`）。
    pub fn fail(msg: &str) -> String {
        serde_json::json!({ "code": "FAIL", "message": msg }).to_string()
    }
}
''',
    # ---- 通知响应（XML）：SUCCESS/FAIL 常量与构造（Java generateXml，单行 CDATA）----
    "notify/wx_pay_notify_response.rs": '''/// 通知响应构造辅助（对应 Java `WxPayNotifyResponse` 静态方法）。
impl WxPayNotifyResponse {
    /// 成功响应（对应 Java `WxPayNotifyResponse.successResp(String msg)`，
    /// 无参时 msg 为 "OK"）。
    pub fn success() -> String {
        Self::success_resp("OK")
    }

    /// 失败响应（对应 Java `WxPayNotifyResponse.failResp(String msg)`）。
    pub fn fail(msg: &str) -> String {
        Self::fail_resp(msg)
    }

    /// 成功响应（对应 Java `WxPayNotifyResponse.successResp`）。
    pub fn success_resp(msg: &str) -> String {
        Self::generate_xml("SUCCESS", msg)
    }

    /// 失败响应（对应 Java `WxPayNotifyResponse.failResp`）。
    pub fn fail_resp(msg: &str) -> String {
        Self::generate_xml("FAIL", msg)
    }

    /// 生成响应 XML（对应 Java `generateXml`，单行 CDATA 格式）。
    pub fn generate_xml(code: &str, msg: &str) -> String {
        format!("<xml><return_code><![CDATA[{code}]]></return_code><return_msg><![CDATA[{msg}]]></return_msg></xml>")
    }
}
''',
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


def xml_impl_block(struct: str, keep_xml_string: bool) -> str:
    """普通 XML 类的 from_xml/to_xml 辅助（quick-xml serde 模式）。

    `keep_xml_string`：struct 含 `xml_string` 字段（BaseWxPayResult 子类）时
    在解析后回填原始报文（对应 Java `setXmlString`）。
    """
    if keep_xml_string:
        body = (
            "        let mut v: Self = quick_xml::de::from_str(xml)\n"
            f"            .map_err(|e| format!(\"{struct} 解析失败: {{e}}\"))?;\n"
            "        v.xml_string = Some(xml.to_string());\n"
            "        Ok(v)"
        )
    else:
        body = (
            "        quick_xml::de::from_str(xml)\n"
            f"            .map_err(|e| format!(\"{struct} 解析失败: {{e}}\"))"
        )
    return (
        "impl " + struct + " {\n"
        "    /// 从 XML 解析（对应 Java `fromXML`，XStream 语义：未知元素忽略、缺失字段默认）。\n"
        "    pub fn from_xml(xml: &str) -> Result<Self, String> {\n"
        + body + "\n"
        "    }\n"
        "\n"
        "    /// 序列化为 XML（根元素 `<xml>`，对应 Java `toXML`）。\n"
        "    ///\n"
        "    /// 注意：quick-xml 以转义文本代替 Java 的 CDATA、空元素输出 `<x/>`\n"
        "    /// （`expand_empty_elements` 归一为 `<x></x>`）——解析语义等价，\n"
        "    /// 逐字节格式化差异记录于 Wave 2。\n"
        "    pub fn to_xml(&self) -> Result<String, String> {\n"
        "        let out = quick_xml::se::to_string(self)\n"
        "            .map_err(|e| format!(\"" + struct + " 序列化失败: {e}\"))?;\n"
        "        Ok(crate::bean::xml::expand_empty_elements(&out))\n"
        "    }\n"
        "}\n"
    )


def post_process(rdir: str, rust_name: str, java_flags: dict, is_xml: bool):
    """追加辅助 impl（XML from_xml/to_xml / 自定义 compose / from_json / to_json）。"""
    rel = os.path.relpath(os.path.join(rdir, rust_name), RUST_BEAN)
    p = os.path.join(rdir, rust_name)
    cur = open(p, encoding="utf-8").read()
    m = re.search(r"pub struct (\w+)", cur)
    struct = m.group(1) if m else None
    if struct is None:
        return

    # 1) 自定义组合逻辑（XML_COMPOSE）优先于通用 XML 辅助
    custom = XML_COMPOSE.get(rel)
    if custom is not None:
        probe = "pub fn compose_" if "compose_" in custom else "pub fn from_xml"
        if probe not in cur:
            cur = cur.rstrip() + "\n" + custom + "\n"
            open(p, "w", encoding="utf-8").write(cur)
        return

    # 2) 通用 XML 辅助（仅顶层 struct；保持 xml_string 的为 BaseWxPayResult 子类）
    if is_xml:
        keep_xml = "xml_string" in cur
        probe = "pub fn from_xml"
        if probe not in cur:
            block = xml_impl_block(struct, keep_xml)
            cur = cur.rstrip() + "\n\n" + block + "\n"
            open(p, "w", encoding="utf-8").write(cur)
        return

    # 3) JSON 辅助（Java fromJson/toJson 检测 + 显式登记）
    actions = []
    spec = POST_PROCESS.get(rel)
    if spec is not None and spec[0] == "from_json_list":
        actions.append(("from_json_list", spec[1]))
    else:
        if spec is not None or java_flags.get("has_from_json"):
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


# 目录树生成
# ---------------------------------------------------------------------------
ALL_SKIPPED = []
ALL_GENERATED = []

# 小样调试：`PAY_GEN_ONLY=request,result` 时仅生成指定目录（env 过滤，全量置空）
ONLY_DIRS = {d for d in os.environ.get("PAY_GEN_ONLY", "").split(",") if d}


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
    供父目录以显式 `pub use path_spec;` 方式按名去重转发，避免同名类型经
    glob 重导出产生歧义。
    """
    jdir = os.path.join(JAVA_BEAN, rel_dir)
    rdir = os.path.join(RUST_BEAN, rel_dir)
    if not os.path.isdir(jdir):
        return [], [], []
    if ONLY_DIRS and rel_dir and rel_dir not in ONLY_DIRS:
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
        result = gen_file(java_path, rel_java)
        if result is None:
            skipped.append(rel_java)
            continue
        content, structs, is_xml = result
        # 跨目录类型引用由 gen_file 的 cross_imports 显式导入（逐类型、无歧义）；
        # 不再使用 miniapp 的祖先包通配导入（同名类型跨子包 glob 会引发 E0659 歧义）。
        rust_name = snake(fn[:-5]) + ".rs"
        with open(os.path.join(rdir, rust_name), "w", encoding="utf-8") as f:
            f.write(content)
        _, _, _, java_flags, _ = parse_java(java_path)
        post_process(rdir, rust_name, java_flags, is_xml)
        mod_name = rust_name[:-3]
        if mod_name in ("abstract", "final", "type"):
            mod_name = f"r#{mod_name}"
        mod_lines.append(f"pub mod {mod_name};")
        for s in structs:
            exports.append((f"{mod_name}::{s}", s))
        generated.append(f"{rel_dir}/{rust_name}" if rel_dir else rust_name)

    subdirs = sorted(
        d for d in os.listdir(jdir)
        if os.path.isdir(os.path.join(jdir, d))
    )
    for sub in subdirs:
        sub_rel = f"{rel_dir}/{sub}" if rel_dir else sub
        sub_generated, sub_skipped, sub_exports = gen_dir(sub_rel)
        generated.extend(sub_generated)
        skipped.extend(sub_skipped)
        # 子目录无任何产出（如纯枚举目录）不注册 `pub mod`，避免空模块报错
        if not os.path.isfile(os.path.join(rdir, sub, "mod.rs")):
            continue
        mod_lines.append(f"pub mod {sub};")
        for path_spec, name in sub_exports:
            exports.append((f"{sub}::{path_spec}", name))

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
        # Wave 2 修复：局部生成（PAY_GEN_ONLY）时根 mod.rs 不在此重建——
        # 未生成目录的导出集会缺失（其它目录提前 return），重建会丢掉
        # 未生成目录的全部导出（如 `crate::bean::WxPayBillResult`）。
        # 根 mod.rs 的权威重建在 main() 全量模式分支。
        if not (ONLY_DIRS and not rel_dir):
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

    # 根 bean/mod.rs 汇总声明（子目录 + 根文件，显式按名去重转发避免 glob 歧义）。
    # Wave 2 修复：仅全量生成（未设置 PAY_GEN_ONLY）时重建根 mod.rs——局部生成
    # （PAY_GEN_ONLY）时子目录导出集会缺失（其它目录提前 return），重建会丢掉
    # 未生成目录的全部导出（如 `crate::bean::WxPayBillResult` 解析失败）。
    rdir = RUST_BEAN
    if not ONLY_DIRS:
        root_mod_lines = ["//! 微信支付 bean。", "//!", f"//! 对应 Java `{JAVA_PKG}` 包（生成）。", ""]
        subdirs = sorted(
            d for d in os.listdir(rdir)
            if os.path.isdir(os.path.join(rdir, d))
        )
        root_files = sorted(fn for fn in os.listdir(rdir) if fn.endswith(".rs") and fn != "mod.rs")
        generated_root = set(g for g in generated if "/" not in g)
        for fn in root_files:
            if fn in generated_root:
                continue
            src = open(os.path.join(rdir, fn), encoding="utf-8").read()
            for s in re.findall(r"pub (?:struct|enum) (\w+)", src):
                root_exports.append((f"{fn[:-3]}::{s}", s))
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
    # 根模块附加声明：枚举重导出（Java bean/result/enums 包，Rust 在 enums/ 模块）
    root_mod_src = open(os.path.join(rdir, "mod.rs"), encoding="utf-8").read()
    extra = (
        "\n// TradeTypeEnum/GlobalTradeTypeEnum 在 Java 位于 bean/result/enums 包，\n"
        "// Wave 0 实现于 enums/ 模块（含 URL 方法），此处重导出以对齐门面的\n"
        "// `crate::bean::` 引用路径。\n"
        "pub use crate::enums::{GlobalTradeTypeEnum, TradeTypeEnum};\n"
    )
    if "pub use crate::enums" not in root_mod_src:
        root_mod_src = root_mod_src.rstrip() + "\n\n" + extra
        open(os.path.join(rdir, "mod.rs"), "w", encoding="utf-8").write(root_mod_src)

    # notify 模块补充说明（v3 通知 AES-GCM 解密留 Wave 2，幂等）
    notify_mod = os.path.join(rdir, "notify", "mod.rs")
    if os.path.isfile(notify_mod):
        note = (
            "\n// v3 通知解密（AEAD_AES_256_GCM，`OriginNotifyResponse.resource.ciphertext`）\n"
            "// 与 v2 退款通知 req_info 解密（AES-256-ECB）在 **Wave 2** 实现：\n"
            "// 相关 bean 已提供 `from_xml`/`from_json` 结构解析，解密 + 验签在服务层接线。\n"
        )
        src = open(notify_mod, encoding="utf-8").read()
        if "Wave 2" not in src:
            open(notify_mod, "w", encoding="utf-8").write(src.rstrip() + "\n" + note)
    print(f"generated: {len(ALL_GENERATED)}")
    print(f"skipped: {len(ALL_SKIPPED)}")
    for s in ALL_SKIPPED:
        print("  skip:", s)


if __name__ == "__main__":
    main()
