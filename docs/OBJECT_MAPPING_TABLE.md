# WxRust 对象级对照表（总览）

- Java 基线：WxJava `4.8.4.B`
- Rust 基线：`easy-4-rust/WxRust`
- 总分母：main 对象 `3287`

本文为对象级对照的总览索引。每个模块的逐对象权威台账在：
- `docs/migration/<module>/对象级对照表.md`

## 模块对象分布（来自 docs/migration/README.md）

| Java 模块 | Rust crate | 对象数 | 文档目录 |
|---|---|---:|---|
| weixin-java-common | wx-rust-common | 174 | [docs/migration/weixin-java-common](migration/weixin-java-common/) |
| weixin-java-mp | wx-rust-mp | 428 | [docs/migration/weixin-java-mp](migration/weixin-java-mp/) |
| weixin-java-miniapp | wx-rust-miniapp | 611 | [docs/migration/weixin-java-miniapp](migration/weixin-java-miniapp/) |
| weixin-java-pay | wx-rust-pay | 570 | [docs/migration/weixin-java-pay](migration/weixin-java-pay/) |
| weixin-java-cp | wx-rust-cp | 594 | [docs/migration/weixin-java-cp](migration/weixin-java-cp/) |
| weixin-java-open | wx-rust-open | 240 | [docs/migration/weixin-java-open](migration/weixin-java-open/) |
| weixin-java-channel | wx-rust-channel | 618 | [docs/migration/weixin-java-channel](migration/weixin-java-channel/) |
| weixin-java-aispeech | wx-rust-aispeech | 25 | [docs/migration/weixin-java-aispeech](migration/weixin-java-aispeech/) |
| weixin-java-qidian | wx-rust-qidian | 27 | [docs/migration/weixin-java-qidian](migration/weixin-java-qidian/) |

## 状态口径

- `IMPLEMENTED / DEPENDENCY_REUSED / PLATFORM_NA` 计入已处置
- `MISSING / MISPLACED / STUB / PARTIAL / UNVERIFIED` 为迁移阻断项

## 当前结论

根据 `docs/migration/README.md`，9 个模块当前均记录为 `0 MISSING`。
