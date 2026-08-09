#!/usr/bin/env python3
"""填充 WxRust 各模块《对象级对照表》：注入对象映射 + 统计汇总（v2，安全版本）。

v2 修复：精确定位表格边界（表头行 -> 下一个 ## 或空行），不再使用宽泛 lookahead。
"""
import csv
import os
import re
import sys
from collections import Counter

BASE = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
CSV_PATH = os.path.join(BASE, "docs", "inventory_java_objects.csv")
MIG_DIR = os.path.join(BASE, "docs", "migration")

KIND_CN = {
    "class": "类",
    "interface": "接口",
    "enum": "枚举",
    "record": "record",
    "annotation": "注解",
}


def is_exception(kind, name):
    return kind == "class" and (name.endswith("Exception") or name.endswith("Error"))


def kind_category(kind, name):
    if kind == "class" and is_exception(kind, name):
        return "异常"
    return KIND_CN.get(kind, "类")


def replace_table(text, header_line, new_body):
    """替换以 header_line 开头的 markdown 表格体。返回 (new_text, replaced_bool)。"""
    lines = text.split("\n")
    out = []
    replaced = False
    i = 0
    while i < len(lines):
        line = lines[i]
        if line.strip().startswith("|") and header_line in line:
            # 找到表头 -> 表头分隔行 -> 表体（连续 | 行）
            j = i
            while j < len(lines) and lines[j].strip().startswith("|"):
                j += 1
            # j 现在是表体结束后的行；替换 i..j 为 header + 分隔 + new_body
            out.append(line)                     # 表头
            out.append(lines[i + 1])             # 分隔行
            out.extend(new_body)
            replaced = True
            i = j
            continue
        out.append(line)
        i += 1
    return "\n".join(out), replaced


def main():
    rows = list(csv.DictReader(open(CSV_PATH, encoding="utf-8")))
    by_module = {}
    for r in rows:
        by_module.setdefault(r["module"], []).append(r)

    for module, mrows in sorted(by_module.items()):
        doc = os.path.join(MIG_DIR, module, "对象级对照表.md")
        if not os.path.exists(doc):
            print(f"[skip] {module}: 无对象级对照表")
            continue

        cats = Counter(kind_category(r["kind"], r["java_name"]) for r in mrows)
        n_class = cats["类"]; n_iface = cats["接口"]; n_enum = cats["枚举"]
        n_rec = cats["record"]; n_ann = cats["注解"]; n_exc = cats["异常"]

        text = open(doc, encoding="utf-8").read()

        # 1) 统计汇总表（三、统计汇总）
        summary_rows = [
            f"| 类 | {n_class} | {n_class} | 0 | 0 | 0 |",
            f"| 接口 | {n_iface} | {n_iface} | 0 | 0 | 0 |",
            f"| 枚举 | {n_enum} | {n_enum} | 0 | 0 | 0 |",
            f"| record | {n_rec} | {n_rec} | 0 | 0 | 0 |",
            f"| 注解 | {n_ann} | {n_ann} | 0 | 0 | 0 |",
            f"| 异常 | {n_exc} | {n_exc} | 0 | 0 | 0 |",
        ]
        text, ok1 = replace_table(text, "| 分类 | Java 对象 |", summary_rows)

        # 2) 对象映射表（四、对象映射）
        obj_rows = []
        for r in mrows:
            fq = r["java_file"].replace("/", ".").replace(".java", "") + "::" + r["java_name"]
            obj_rows.append(
                f"| `{fq}` | {r['kind']} | `{r['java_file']}` | `{r['rust_path']}` | `{r['rust_path']}` "
                f"| 待定 | 0/0 | 缺失 | `MISSING` | 规划阶段，无实现 | 清点自 inventory_java_objects.py |"
            )
        text, ok2 = replace_table(text, "| Java 全限定名 | 类型 | Java 文件/符号 |", obj_rows)

        open(doc, "w", encoding="utf-8").write(text)
        print(f"[ok] {module}: 统计表={'是' if ok1 else '否'} 对象表={'是' if ok2 else '否'} 对象={len(mrows)}")


if __name__ == "__main__":
    sys.exit(main())
