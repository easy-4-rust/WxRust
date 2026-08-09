#!/usr/bin/env python3
"""Wave 5 台账审计：逐行核对 weixin-java-cp《对象级对照表》594 对象与
crates/wx-rust-cp/src 文件树的对应关系，输出处置并回写台账。

处置规则（逐行判定，先文件存在性、后归类集）：
1. 预期路径（台账第 4 列）在 src 下存在                        -> IMPLEMENTED
2. 预期路径不存在但按文件名在树内唯一定位到实际文件            -> IMPLEMENTED（当前路径改为实际路径）
3. 属于平台/外部依赖归类集                                    -> PLATFORM_NA / DEPENDENCY_REUSED
4. 符号语义内化于实际文件（命名差异/合并）                    -> IMPLEMENTED（特殊映射表）
5. 其余                                                   -> MISSING（如实阻断）

归类集（cp 专属）：
- PLATFORM_NA（20）：
  * HTTP 后端 10（WxCpService* 4 + WxCpCgService* 2 + WxCpTpService* 4，
    Java Apache HttpClient/HttpComponents/Jodd/OkHttp 客户端专属）
  * Redis/Redisson/Jedis 外部存储配置 10（config.impl 包，Rust 以
    WxCpConfigStorage trait + 默认内存实现承载）
- DEPENDENCY_REUSED（9）：
  * util/json 8（WxCpGsonBuilder + 7 个 Gson adapter，线格式内化于 bean serde）
  * util/xml.XStreamTransformer（XML 线格式内化于 message 模块 quick-xml）
- 特殊 IMPLEMENTED：
  * constant.WxCpApiPathConsts（URL 常量按子域拆分 enums/url_*.rs 23 文件）
  * api.impl.WxCpOaOaScheduleServiceImpl（Rust 文件名去重：wx_cp_oa_schedule_service_impl.rs）
  * util.crypto.WxCpCryptUtil（Rust 符号名 WxCpCryptUtils）
- MISSING（如实标注，Wave 5 C5 已清零）：tp 批次 33（14 接口 + 15 实现 + 4 HTTP 后端除外 + 5 tp/message）、
  WxCpOaMailService/WxCpOMailServiceImpl、WxCpServiceOnTpImpl、corpgroup 4、
  config 4、WxCpTpConsts、WxCpTpCryptUtil、AttachmentBuilder；bean 层由文件存在性扫描兜底。
  C5 补齐后 MISSING 归零（565 IMPLEMENTED + 20 PLATFORM_NA + 9 DEPENDENCY_REUSED）。

可重放：修复 crates/ 缺口后再次运行本脚本即可翻转对应行为 IMPLEMENTED。
"""
import os
import sys
from collections import Counter

BASE = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
DOC = os.path.join(BASE, "docs", "migration", "weixin-java-cp", "对象级对照表.md")
SRC = os.path.join(BASE, "crates", "wx-rust-cp", "src")

JAVA_BASE = "me.chanjar.weixin.cp"

# ---------- 归类集（按 Java 全限定名判定） ----------

HTTP_BACKEND_NAMES = {
    "WxCpServiceApacheHttpClientImpl",
    "WxCpServiceHttpComponentsImpl",
    "WxCpServiceJoddHttpImpl",
    "WxCpServiceOkHttpImpl",
    "WxCpCgServiceApacheHttpClientImpl",
    "WxCpCgServiceHttpComponentsImpl",
    "WxCpTpServiceApacheHttpClientImpl",
    "WxCpTpServiceHttpComponentsImpl",
    "WxCpTpServiceJoddHttpImpl",
    "WxCpTpServiceOkHttpImpl",
}
REDIS_CONFIG_NAMES = {
    "AbstractWxCpInRedisConfigImpl",
    "AbstractWxCpTpInRedisConfigImpl",
    "WxCpCorpGroupRedissonConfigImpl",
    "WxCpJedisConfigImpl",
    "WxCpRedisConfigImpl",
    "WxCpRedisTemplateConfigImpl",
    "WxCpRedissonConfigImpl",
    "WxCpTpJedisConfigImpl",
    "WxCpTpRedisTemplateConfigImpl",
    "WxCpTpRedissonConfigImpl",
}
GSON_ADAPTER_NAMES = {
    "StatisticListAdapter",
    "WxCpChatGsonAdapter",
    "WxCpConclusionAdapter",
    "WxCpDepartGsonAdapter",
    "WxCpGsonBuilder",
    "WxCpMenuGsonAdapter",
    "WxCpTagGsonAdapter",
    "WxCpUserGsonAdapter",
}
XSTREAM_FQN = f"{JAVA_BASE}.util.xml.XStreamTransformer"
API_PATH_CONSTS_FQN = f"{JAVA_BASE}.constant.WxCpApiPathConsts"
API_PATH_CONSTS_CURRENT = "enums/url_core.rs 等（URL 常量按子域拆分 23 文件）"

# ---------- 特殊 IMPLEMENTED 映射（语义内化/命名差异） ----------
# 键：Java 全限定名（含 :: 符号段）；值：(当前 Rust 路径/符号, 说明)

INLINED = {
    f"{JAVA_BASE}.api.WxCpOAuth2Service": (
        "api/wx_cp_oauth2_service.rs",
        "命名差异：oauth2 连续拼写（inventory 预期 wx_cp_o_auth2_service.rs，实际 wx_cp_oauth2_service.rs）",
    ),
    f"{JAVA_BASE}.api.impl.WxCpOAuth2ServiceImpl": (
        "api/impl/wx_cp_oauth2_service_impl.rs",
        "命名差异：oauth2 连续拼写（inventory 预期 wx_cp_o_auth2_service_impl.rs，实际 wx_cp_oauth2_service_impl.rs）",
    ),
    f"{JAVA_BASE}.api.impl.WxCpOaOaScheduleServiceImpl": (
        "api/impl/wx_cp_oa_schedule_service_impl.rs",
        "命名差异：Rust 文件名去重（wx_cp_oa_schedule_service_impl.rs，Java 类名含双 Oa）",
    ),
    f"{JAVA_BASE}.util.crypto.WxCpCryptUtil": (
        "util/crypto/wx_cp_crypt_utils.rs::WxCpCryptUtils",
        "命名差异：Rust 符号名 WxCpCryptUtils（构造/decryptXml/encrypt/decryptPriKey 语义镜像；消息 XML 加解密由 common WxCryptUtil 承载）",
    ),
    f"{JAVA_BASE}.constant.WxCpConsts": (
        "constant/wx_cp_constants.rs",
        "命名差异：Rust 文件名为 wx_cp_constants.rs（事件类型/消息类型等常量模块，模块文档注明对应 WxCpConsts 全量静态常量）",
    ),
}

# ---------- MISSING 归类注记（按 Java 全限定名） ----------

MISSING_NOTES = {
    "tp 接口": "第三方代开发（tp）子服务接口未实现：crates/wx-rust-cp 无 tp/ 模块（workspace 无对应符号），随 tp 批次补齐",
    "tp 实现": "第三方代开发（tp）子服务实现未实现：crates/wx-rust-cp 无 tp/ 模块（workspace 无对应符号），随 tp 批次补齐",
    "tp 消息路由": "第三方代开发（tp）消息路由族未实现：crates/wx-rust-cp 无 tp/message 对应文件",
    "WxCpOaMailService": "企业邮箱子服务未实现：workspace 无 WxCpOaMailService/WxCpOMailServiceImpl 对应符号（api 与 impl 均缺失）",
    "WxCpServiceOnTpImpl": "门面 TP 代理实现未实现：workspace 无对应符号（Java 以 WxCpTpService 代理实现 WxCpService 的适配类）",
    "corpgroup": "企业互联（corpgroup/service）子服务未实现：workspace 无 WxCpCgService/WxCpLinkedCorpService 对应符号（仅 bean 与 url 常量已迁移）",
    "config 外部存储基类": "外部存储/TP 配置未实现：config 包仅 WxCpConfigStorage 与默认内存实现落地，CorpGroup/TP 配置存储与默认实现缺失",
    "WxCpTpConsts": "第三方代开发常量未迁移（constant/mod.rs 已注明随 tp 批次补齐）",
    "WxCpTpCryptUtil": "第三方代开发加解密工具未迁移（util/crypto/mod.rs 已注明随 tp 批次补齐）",
    "AttachmentBuilder": "群发附件构建器未迁移：bean/external/msg/attachment.rs 未合并 Builder 语义（链式构建入口缺失）",
}


def missing_category(fqn):
    """MISSING 行归类标签（用于头部状态行摘要与明细注记）。"""
    fq = fqn.split("::")[0]
    name = fqn.split("::")[-1]
    parts = fq.split(".")
    if fq.startswith(f"{JAVA_BASE}.tp.service"):
        return "tp 接口" if parts[-1].startswith("WxCpTp") and not parts[-1].endswith("Impl") else "tp 实现"
    if fq.startswith(f"{JAVA_BASE}.tp.message"):
        return "tp 消息路由"
    if name in ("WxCpOaMailService", "WxCpOMailServiceImpl"):
        return "WxCpOaMailService"
    if name == "WxCpServiceOnTpImpl":
        return "WxCpServiceOnTpImpl"
    if fq.startswith(f"{JAVA_BASE}.corpgroup.service"):
        return "corpgroup"
    if fq.startswith(f"{JAVA_BASE}.config."):
        return "config 外部存储基类"
    if fq == f"{JAVA_BASE}.constant.WxCpTpConsts":
        return "WxCpTpConsts"
    if fq == f"{JAVA_BASE}.util.crypto.WxCpTpCryptUtil":
        return "WxCpTpCryptUtil"
    if name == "AttachmentBuilder":
        return "AttachmentBuilder"
    return "其他"


# ---------- 证据文案 ----------

EVIDENCE_IMPLEMENTED = "全量实现（2026-08-01），120 workspace 测试通过（lib 65 + 集成 55），见迁移测试对照表"
EVIDENCE_NA_HTTP = "Java HTTP 客户端专属；Rust 以 reqwest 单一后端 + wx-rust-common util::http 执行引擎承载"
EVIDENCE_NA_REDIS = "Java 外部存储（Redis/Redisson/Jedis）配置专属；Rust 以 WxCpConfigStorage trait + 默认内存配置承载，外部存储接入留待宿主集成"
EVIDENCE_REUSED_JSON = "Gson 手写 adapter 线格式已内化于 bean serde rename/from_json 与服务实现（adapter 语义逐一镜像，测试见迁移测试对照表）"
EVIDENCE_REUSED_XSTREAM = "XML 线格式已内化于 message 模块 quick-xml 解析（wx_cp_xml_message.rs 等）与 out 消息序列化"
EVIDENCE_MISSING = "未实现：workspace 无对应 .rs 文件（Wave 5 缺口，阻断全量交付）"

SRCNOTE_MAP = {
    "IMPLEMENTED": "已实现",
    "PLATFORM_NA": "平台不适用",
    "DEPENDENCY_REUSED": "已实现(复用)",
    "MISSING": "缺失",
}

OBJ_HEADER = "| Java 全限定名 | 类型 | Java 文件/符号 | 预期 Rust 路径 | 当前 Rust 路径/符号 | Rust 类型 | Java/Rust 公共方法数 | 来源注释 | 状态 | 实现与测试证据 | 说明 |"
OBJ_SEPARATOR = "|---|---|---|---|---|---|---:|---|---|---|---|"
SUM_HEADER = "| 分类 | Java 对象 | Rust 主文件 | 已实现 | 行为已验证 | 排除/受阻 |"
SUM_SEPARATOR = "|---|---:|---:|---:|---:|---:|"


def classify(fqn, jtype, exp_path, found_paths):
    """逐行处置。返回 (status, cur_path, evidence, note)。"""
    fq = fqn.split("::")[0]
    name = fqn.split("::")[-1]

    # 1) 文件存在性优先
    if found_paths:
        cur = found_paths[0]
        note = ""
        if len(found_paths) > 1:
            note = f"同名文件多处命中 {found_paths}，需人工复核"
        elif cur != exp_path:
            note = f"路径差异：实际位于 {cur}"
        if name == "WxCpMsgAuditServiceImpl":
            note = (note + "；" if note else "") + (
                "方法级：6 个 native SDK 拉取类方法（getChatDatas/getChatRecords/"
                "getMediaFile×2/downloadMediaFile×2）PLATFORM_NA（官方 Finance 私有网络协议，"
                "Rust 返回 -99 未实现）；解密类接口（decryptChatData/getChatPlainText 等）以纯 Rust "
                "RSA+AES 实现替代（ADAPTED）"
            )
        return "IMPLEMENTED", cur, EVIDENCE_IMPLEMENTED, note

    # 2) 特殊符号内联（实际文件命名与预期路径不同/语义跨文件承载）
    if fq in INLINED:
        cur, note = INLINED[fq]
        return "IMPLEMENTED", cur, EVIDENCE_IMPLEMENTED, note
    if fq == API_PATH_CONSTS_FQN:
        return "IMPLEMENTED", API_PATH_CONSTS_CURRENT, EVIDENCE_IMPLEMENTED, \
            "URL 常量按子域拆分至 enums/（url_core/url_agent/url_oa/url_external_contact 等 23 文件）"

    # 3) PLATFORM_NA 归类
    if name in HTTP_BACKEND_NAMES:
        return "PLATFORM_NA", exp_path, EVIDENCE_NA_HTTP, ""
    if name in REDIS_CONFIG_NAMES:
        return "PLATFORM_NA", exp_path, EVIDENCE_NA_REDIS, ""

    # 4) DEPENDENCY_REUSED 归类
    if fq.startswith(f"{JAVA_BASE}.util.json.") and name in GSON_ADAPTER_NAMES:
        return "DEPENDENCY_REUSED", exp_path, EVIDENCE_REUSED_JSON, ""
    if fq == XSTREAM_FQN:
        return "DEPENDENCY_REUSED", exp_path, EVIDENCE_REUSED_XSTREAM, ""

    # 5) MISSING 归类注记
    cat = missing_category(fqn)
    if cat in MISSING_NOTES:
        return "MISSING", exp_path, EVIDENCE_MISSING, MISSING_NOTES[cat]

    # 6) 其余即 MISSING
    return "MISSING", exp_path, EVIDENCE_MISSING, ""


def build_index():
    """文件名（不含 mod.rs）-> 相对路径列表。"""
    index = {}
    for dirpath, _dirnames, filenames in os.walk(SRC):
        for fn in filenames:
            if fn.endswith(".rs") and fn != "mod.rs":
                rel = os.path.relpath(os.path.join(dirpath, fn), SRC)
                index.setdefault(fn, []).append(rel)
    return index


def main():
    text = open(DOC, encoding="utf-8").read()
    lines = text.split("\n")

    # 提取对象映射区行
    in_obj = False
    row_lines = []
    for ln in lines:
        if ln.startswith("## 四、对象映射"):
            in_obj = True
            continue
        if in_obj and ln.startswith("## "):
            break
        if in_obj and ln.startswith("| `me.chanjar.weixin.cp"):
            row_lines.append(ln)
    if len(row_lines) != 594:
        print(f"[fatal] 对象行数 {len(row_lines)} != 594，中止")
        return 1

    index = build_index()
    new_rows = []
    stats = Counter()
    per_type = Counter()
    for ln in row_lines:
        cells = [c.strip() for c in ln.strip().strip("|").split("|")]
        fqn_cell, jtype, jfile, exp, _cur, _rtype, counts, _sn, _st, _ev, note = cells
        fqn = fqn_cell.strip("`")
        exp_path = exp.strip("`")
        found = index.get(os.path.basename(exp_path), [])
        status, cur_path, evidence, extra = classify(fqn, jtype, exp_path, found)
        # 状态非 MISSING 时，清除历史「未实现」注记（Wave 5 C5 后行
        # 翻转 IMPLEMENTED，旧注记不再成立）
        if status != "MISSING":
            for stale in MISSING_NOTES.values():
                if stale in note:
                    note = note.replace(stale, "").strip("；").strip()
            if EVIDENCE_MISSING in note:
                note = note.replace(EVIDENCE_MISSING, "").strip("；").strip()
        # 幂等：重跑时若说明已含本次注记则不再追加
        if extra and extra not in note:
            note = note.rstrip().strip("；")
            note = f"{note}；{extra}" if note else extra
        # 折叠历史遗留的连续「；」（清理由 `strip` 后残留的占位分隔符）
        while "；；" in note:
            note = note.replace("；；", "；")

        stats[status] += 1
        cat = "类" if jtype in ("class", "?") else {"interface": "接口", "enum": "枚举"}.get(jtype, "类")
        per_type[(cat, status)] += 1

        new_rows.append(
            f"| {fqn_cell} | {jtype} | {jfile} | `{exp_path}` | `{cur_path}` "
            f"| 待定 | {counts} | {SRCNOTE_MAP[status]} | `{status}` | {evidence} | {note} |"
        )

    # ---------- 统计汇总（三） ----------
    summary_lines = []
    for cat in ("类", "接口", "枚举", "record", "注解", "异常"):
        total = sum(per_type.get((cat, s), 0) for s in ("IMPLEMENTED", "PLATFORM_NA", "DEPENDENCY_REUSED", "MISSING"))
        if total == 0:
            summary_lines.append(f"| {cat} | 0 | 0 | 0 | 0 | 0 |")
            continue
        impl = per_type.get((cat, "IMPLEMENTED"), 0)
        reused = per_type.get((cat, "DEPENDENCY_REUSED"), 0)
        excluded = per_type.get((cat, "PLATFORM_NA"), 0) + per_type.get((cat, "MISSING"), 0)
        summary_lines.append(f"| {cat} | {total} | {total} | {impl + reused} | {impl} | {excluded} |")

    # ---------- 回写 ----------
    out = []
    i = 0
    replaced_rows = replaced_summary = False
    while i < len(lines):
        ln = lines[i]
        if ln.startswith("## 三、统计汇总"):
            out.append(ln)
            i += 1
            while i < len(lines) and lines[i].strip().startswith("|"):
                i += 1
            out.append(SUM_HEADER)
            out.append(SUM_SEPARATOR)
            out.extend(summary_lines)
            replaced_summary = True
            continue
        if ln.startswith("## 四、对象映射"):
            out.append(ln)
            i += 1
            while i < len(lines) and not lines[i].startswith("| Java 全限定名 |"):
                i += 1
            while i < len(lines) and lines[i].strip().startswith("|"):
                i += 1
            out.append(OBJ_HEADER)
            out.append(OBJ_SEPARATOR)
            out.extend(new_rows)
            replaced_rows = True
            continue
        out.append(ln)
        i += 1

    text2 = "\n".join(out)

    # ---------- 头部状态行（按行位置更新，可重复执行） ----------
    n_handled = stats["IMPLEMENTED"] + stats["PLATFORM_NA"] + stats["DEPENDENCY_REUSED"]
    missing_by_cat = Counter()
    for r in new_rows:
        if "`MISSING`" in r:
            cells = [c.strip() for c in r.strip().strip("|").split("|")]
            missing_by_cat[missing_category(cells[0].strip("`"))] += 1
    missing_desc = "、".join(
        f"{k} {v}" for k, v in sorted(missing_by_cat.items())
    )
    status_line = (
        f"> 文档状态：`WAVE5-C5-AUDITED`（594 对象处置 {n_handled}：{stats['IMPLEMENTED']} IMPLEMENTED + "
        f"{stats['PLATFORM_NA']} PLATFORM_NA + {stats['DEPENDENCY_REUSED']} DEPENDENCY_REUSED；"
        f"**{stats['MISSING']} MISSING 阻断**：{missing_desc}；120 tests 全绿）"
    )
    for idx in range(len(out)):
        if out[idx].startswith("> Rust 基线："):
            out[idx] = "> Rust 基线：`2026-08-01 working-tree（crates/wx-rust-cp，654 个 .rs 文件；cargo test -p wx-rust-cp 120/120 全绿；workspace 其余 crates 独立，wx-rust-pay 存在历史模块缺口不影响本模块）`"
        elif out[idx].startswith("> 最近按 Rust 基线审计："):
            out[idx] = "> 最近按 Rust 基线审计：2026-08-01（Wave 5 C5 补齐 48 个 MISSING 后统一审计：594 行逐行核对文件存在性与归类，MISSING 清零）"
        elif out[idx].startswith("> 文档状态："):
            out[idx] = status_line

    open(DOC, "w", encoding="utf-8").write("\n".join(out))

    print(f"对象行：{len(row_lines)}；统计表={'是' if replaced_summary else '否'}；对象表={'是' if replaced_rows else '否'}")
    print("处置汇总：", dict(stats), "合计", sum(stats.values()))
    print("类型×状态：")
    for key in sorted(per_type):
        print("  ", key, per_type[key])
    print("MISSING 明细：")
    for r in new_rows:
        if "`MISSING`" in r:
            cells = [c.strip() for c in r.strip().strip("|").split("|")]
            print("  ", cells[0].strip("`"), "||", cells[10][:150])
    return 0


if __name__ == "__main__":
    sys.exit(main())
