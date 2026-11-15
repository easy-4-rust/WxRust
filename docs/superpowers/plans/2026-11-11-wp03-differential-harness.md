# WP-03 双运行时差分 Implementation Plan

> 执行：superpowers:executing-plans；测试先行，未执行不计通过。

**Goal:** 在同一组 JSONL fixtures 上实际调用固定版本 WxJava 与当前 WxRust，持久化两侧输出与差分报告。

**Architecture:** Java Adapter 只调用固定源码编译产物，不复制 Oracle 业务逻辑。Rust Adapter 复用 wx-rust-pay example 及现有 common/pay 依赖，不新增 workspace crate。Python 标准库负责构建、协议校验、比较和证据摘要。

**Spec:** `openspec/changes/wxjava-1to1-migration-closure/design.md`，T016–T019。

## Constraints / rulings

- Oracle 为 a49d6e1461752c06b752d2afd8aeeb7e6e78cefe；校验 HEAD 和受版本控制文件的状态。
- 保留当前 feature/wxjava-1to1-phase1 未提交改动，不新建丢失改动的源码快照，不推送、不发布。
- Java 17 使用明确 JDK 路径，不能误用 PATH 上 Java 8。
- 首批执行 Common WxError.fromJson/fromJson(type) 与 Pay SignUtils.createSign；CP Finance Native 缺少 SDK，保持 UNSUPPORTED，不能用替身获得通过。
- JSON key 顺序可忽略；数组顺序、字段缺失/null/空串、布尔与数字必须区分。
- XML 比较只允许显式选用；忽略属性顺序，不忽略文本/子节点顺序，拒绝 DTD/实体声明。
- 行为差异、构建/运行/协议失败、未支持必须分开；空样本不能通过。
- 只证明具体样本涉及的可观察字段，不将方法矩阵批量升级 VERIFIED。
- Ruling: 首批 Common 比较 errorCode/errorMsg/errorMsgEn，不比较 Gson 与 serde 的原始报文格式化字符串；协议文档明确未覆盖部分。
- Ruling: 历史 1298 数字来自被截断日志的自动摘要，本轮不作为完整测试总数复用。

## Tasks

- [x] A. comparator 测试 RED → 实现 GREEN，覆盖缺失输出、重复 ID、空样本、JSON 类型、XML、unsupported。最终 comparator/runner 共18项自测通过。
- [x] B. 创建27个共享 fixtures 与两个薄 Adapter，运行真实 Oracle/目标输出；自测能检测人工篡改，且核验实际 Java class 来源。
- [x] C. 首轮8项差异；Pay UTF-16排序两项 RED→GREEN，最终6项Common差异保留未修复。
- [x] D. 全工作区5007 passed、0 failed、1 ignored；fmt/check/严格Clippy通过，完整日志及哈希已保留。
- [x] E. 首批协议、命令、机器证据、报告和OpenSpec状态已落地；完整WP-03及P0/P1/P2仍进行中。

## File map

- tools/compat/test_compare.py：比较器回归。
- tools/compat/compare.py：严格样本协议与结果比较。
- tools/compat/fixtures.jsonl：两侧共用输入。
- tools/compat/java/Phase1Oracle.java：真实 Java SDK 调用。
- crates/wx-rust-pay/examples/phase1_oracle.rs：真实 Rust SDK 调用。
- tools/compat/run_diff.py：构建、运行、哈希与退出码。
- tools/compat/README.md：协议与复现方式。
- docs/verification/wp03-differential-2026-11-11.md：证据及边界。
