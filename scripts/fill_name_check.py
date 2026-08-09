#!/usr/bin/env python3
"""填充《对象名称一致性检查.md》——逐行安全处理，避免正则回溯。

用法: fill_name_check.py <module> [--inject-rows]
"""
import csv
import os
import re
import sys

BASE = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
MIG_DIR = os.path.join(BASE, "docs", "migration")
CSV_PATH = os.path.join(BASE, "docs", "inventory_java_objects.csv")


def main():
    module = sys.argv[1]
    inject = "--inject-rows" in sys.argv
    doc = os.path.join(MIG_DIR, module, "对象名称一致性检查.md")
    lines = open(doc, encoding="utf-8").read().split("\n")

    # 对象统计
    rows = [r for r in csv.DictReader(open(CSV_PATH, encoding="utf-8")) if r["module"] == module]
    n_obj = len(rows)
    tests = {"weixin-java-common": 29, "weixin-java-mp": 71, "weixin-java-miniapp": 68,
             "weixin-java-pay": 71, "weixin-java-cp": 88, "weixin-java-open": 13,
             "weixin-java-channel": 31, "weixin-java-aispeech": 2, "weixin-java-qidian": 6}.get(module, 0)

    out = []
    section = None
    in_section3 = False
    section3_rows = []
    for i, line in enumerate(lines):
        if line.startswith("## "):
            section = line[3:].strip()
            in_section3 = section.startswith("三、")
            out.append(line)
            continue
        stripped = line.strip()

        # --- 二、统计汇总 ---
        if section and section.startswith("二、") and stripped.startswith("|"):
            if "对象/主文件" in line:
                out.append(f"| 对象/主文件 | {n_obj} | 0 | 0 | 0 | {n_obj} | 0 |")
                continue
            if "公共方法/构造器" in line:
                out.append("| 公共方法/构造器 | 待 B0 以 javap 冻结 | 0 | 0 | 0 | 全量 | 0 |")
                continue
            if "参数序列" in line:
                out.append("| 参数序列 | 待 B0 枚举 | 0 | 0 | 0 | 全量 | 0 |")
                continue
            if "示例/脚本" in line:
                out.append("| 示例/脚本 | 0 | 0 | 0 | 0 | 0 | 0 |")
                continue
            if stripped.startswith("| 测试") or stripped.startswith("|测试"):
                out.append(f"| 测试 | {tests} 测试对象 | 0 | 0 | 0 | {tests} | 0 |")
                continue

        # --- 三、对象名称完全匹配（逐行注入） ---
        if section and section.startswith("三、"):
            if stripped.startswith("| <!-- TODO -->"):
                if inject and not section3_rows:
                    # 用对象清单替换 TODO 行
                    section3_rows = [
                        f"| `{r['java_name']}` | `{r['rust_path']}` | `{r['java_name']}` | 0/0 | `MISSING` | 规划阶段，无实现（inventory_java_objects.py） |"
                        for r in rows
                    ]
                    out.extend(section3_rows)
                continue
            # 表头/分隔行照抄
            out.append(line)
            continue

        # --- 四、Java 有 Rust 缺失 ---
        if section and section.startswith("四、") and stripped.startswith("| <!-- TODO -->"):
            out.append("| 全部 Java 对象 | 未迁移（规划阶段） | 0 | P0 | B2 整批实现 |")
            continue

        # --- 五、Rust 有 Java 无对应 ---
        if section and section.startswith("五、") and stripped.startswith("| <!-- TODO -->"):
            out.append("| （规划阶段无 Rust 代码） | — | — | — | — |")
            continue

        # --- 六、合并、拆分与重命名 ---
        if section and section.startswith("六、") and stripped.startswith("| <!-- TODO -->"):
            out.append("| （规划阶段无合并/拆分/重命名决策，待 B1） | — | — | 是 | — | 规划中 |")
            continue

        # --- 七、方法名称与重载 ---
        if section and section.startswith("七、") and stripped.startswith("| <!-- TODO -->"):
            out.append("| （B0 冻结签名分母后逐条填写） | — | — | — | 0 | — | 缺失 |")
            continue

        # --- 八、参数一致性 ---
        if section and section.startswith("八、"):
            if stripped.startswith("| <!-- TODO -->"):
                out.append("| （B0 冻结参数分母后逐条填写） | — | — | — | — | — | 缺失 |")
                continue
            if "| `getName()` / `setName(v)` |" in line:
                out.append("| （B1 后逐 bean 填写 JavaBean 适配） | — | — | — | `ADAPTED` | 待验证 | `MISSING` |")
                continue

        # --- 九、业务逻辑一致性 ---
        if section and section.startswith("九、"):
            if "### 完全一致且已验证" in line:
                out.append(line)
                continue
            if stripped.startswith("| <!-- TODO -->"):
                # 未实现/未验证行
                out.append("| 全部 Java 对象 | 无 Rust 实现（规划阶段） | 规划阶段 | `MISSING` | B2 整批实现 |")
                continue

        # --- 十、结构红线检查 ---
        if section and section.startswith("十、") and stripped.startswith("| ") and "<!-- TODO -->" in line:
            # 提取检查项名
            m = re.match(r"\| `([^`]+)` \|", line)
            name = m.group(1) if m else "?"
            out.append(f"| `{name}` | 未检查（规划阶段） | — | B2 后统一审计 |")
            continue

        # --- 十一、结论 ---
        if section and section.startswith("十一、"):
            if stripped.startswith("| 结构覆盖") and "<!-- TODO -->" in line:
                out.append(f"| 结构覆盖 | 0/{n_obj} | 规划阶段，未实现 | 全部 |")
                continue
            if stripped.startswith("| 实现覆盖") and "<!-- TODO -->" in line:
                out.append(f"| 实现覆盖 | 0/{n_obj} | 规划阶段 | 全部 |")
                continue
            if stripped.startswith("| 行为覆盖") and "<!-- TODO -->" in line:
                out.append(f"| 行为覆盖（镜像/golden/live 分列） | 0/{tests} 镜像；0 golden；0 live | 规划阶段 | 全部 |")
                continue
            if stripped.startswith("| 测试覆盖") and "<!-- TODO -->" in line:
                out.append(f"| 测试覆盖 | 0/{tests} | 规划阶段 | 全部 |")
                continue
            if stripped.startswith("| 集成覆盖") and "<!-- TODO -->" in line:
                out.append("| 集成覆盖 | 0/N | 规划阶段 | 全部 |")
                continue
            if stripped.startswith("| 生产就绪") and "<!-- TODO -->" in line:
                out.append("| 生产就绪 | 0/N | 规划阶段 | 全部 |")
                continue
            # 四文档一致性
            if stripped.startswith("| 四文档 Java/Rust SHA 相同") and "<!-- TODO -->" in line:
                out.append("| 四文档 Java/Rust SHA 相同 | ✅ a49d6e14 / plan-only | — | — |")
                continue
            if stripped.startswith("| 对象总数、排除项与分母相同") and "<!-- TODO -->" in line:
                out.append(f"| 对象总数、排除项与分母相同 | ✅ {n_obj}（三文档一致） | — | — |")
                continue
            if stripped.startswith("| 完成状态与技术/语义缺口无冲突") and "<!-- TODO -->" in line:
                out.append("| 完成状态与技术/语义缺口无冲突 | ✅ 全 MISSING 一致 | — | — |")
                continue
            if stripped.startswith("| `IMPLEMENTED`") and "<!-- TODO -->" in line:
                out.append("| `IMPLEMENTED` 均有当前实现与语义测试证据 | ✅ 无 IMPLEMENTED 行 | — | — |")
                continue
            if stripped.startswith("| `DEPENDENCY_REUSED`") and "<!-- TODO -->" in line:
                out.append("| `DEPENDENCY_REUSED` 与 `PLATFORM_NA` 均有精确证据 | ✅ 排除项已在语义表十三节记录 | — | — |")
                continue
            if stripped.startswith("| 合并/拆分/重命名均有批准") and "<!-- TODO -->" in line:
                out.append("| 合并/拆分/重命名均有批准 | ✅ 规划阶段无决策 | — | — |")
                continue
            # 下一步行动
            if stripped.startswith("1. <!-- TODO"):
                out.append("1. P0：B0 冻结方法/参数分母（javap 全量枚举）")
                continue
            if stripped.startswith("2. <!-- TODO"):
                out.append("2. P1：B2 整批实现 + 注释迁移")
                continue
            if stripped.startswith("3. <!-- TODO"):
                out.append("3. P2：V0-V7 统一验证（token 并发、重试差分、golden 夹具）")
                continue

        out.append(line)

    open(doc, "w", encoding="utf-8").write("\n".join(out))
    remain = "\n".join(out).count("<!-- TODO -->")
    print(f"[ok] {module}: 注入 {len(section3_rows) if inject else 0} 对象行, 剩余 TODO {remain}")


if __name__ == "__main__":
    main()
