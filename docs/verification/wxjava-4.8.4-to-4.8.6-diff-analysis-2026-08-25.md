# WxJava 4.8.4→4.8.6 差异分析与 WxRust 追补建议

日期：2026-08-25
分析者：codegraph（基于 WxJava HEAD `25423da4` + v4.8.4..v4.8.6 diff）
报告路径：本文档

## 一、版本对比（59 commits / 238 new + 0 delete / 111 modified）

```
v4.8.4..v4.8.5：29 commits
v4.8.5..v4.8.6：30 commits
```

### 4.8.5 主要功能性 commit
- `#4070` 微信支付：新增 V3 服务商电子发票（邀约/抬头/开票/冲红/文件上传/下载）
- `#4031` 视频号：新增微信小店**代发管理**模块（Supplier 关联、自动分配、Dropship 单据）
- `#4030` 视频号：新增微信小店**赠品与买赠活动**模块
- `#4039` 视频号：新增微信小店**质检管理**模块
- `#4044` 微信支付：新增商家转账**用户授权免确认**接口
- `#4055/4056/4057` common 修复：日志脱敏、GsonHelper 健壮性、FileUtils Base64 截断
- `#4060/4069` 修复：open-multi starter 硬编码 HttpClient4；miniapp JSON 验签解密
- `#4050/4058` 修复：pay codepay 202 状态；spring-data-redis BOM

### 4.8.6 主要功能性 commit
- `#4107` 微信支付：新增**服务商点金计划** + patchV3 修复缺失 HTTP 头
- `#4109` 视频号：微信小店**商品赠品/限时抢购/库存/商品辅助服务**
- `#4102` 视频号：支持**微信小店电子面单**服务
- `#4106` 视频号：支持**售后保障单**
- `#4100` 企业微信：支持**智能机器人 API 模式**
- `#4092` 企业微信：增加**待办 API**（`WxCpTodoService`）
- `#4079/4078/4074` 修复：企业微信长整型 AppID、小程序手机号 openid 校验、pay 小程序纯签约版本字段
- `#4071` 新增 WxJava 用户技能(SKILL)指南

## 二、WxJava 主源新增统计

| 模块 | 新增 .java 主源 | 占比 | 主要新增 |
|---|---|---|---|
| weixin-java-channel | 170 | 85% | 电子面单/收藏/赠品/客服/限时折扣/商品辅助/库存/质检/供货商/带货人（10 个 service + 160 bean） |
| weixin-java-pay | 50 | 25% | V3 服务商电子发票、商家转账用户授权、点金计划 |
| weixin-java-cp | 9 | 4.5% | 智能机器人 API 模式、待办 API |
| weixin-java-miniapp | 5 | 2.5% | 一站式 NFC scheme |
| weixin-java-open | 3 | 1.5% | 微信小店商家客服 |
| weixin-java-common | 1 | 0.5% | OCR menu result |
| weixin-java-aispeech | 0 | 0% | — |
| weixin-java-qidian | 0 | 0% | — |
| weixin-java-mp | 0 | 0% | — |

**结论：4.8.4→4.8.6 的功能新增集中在 channel（视频号小店）**，与 4.8.4 当时 cp/pay/miniapp 并重的模式不同。

## 三、WxRust 追补缺口（真实缺失约 196/199 个主源文件）

通过采样验证（18 个抽样，15 MISS / 3 OK），WxRust 在以下新功能上完全未跟进：
- ❌ 视频号 channel 10 个新 service（ewaybill/favorite/gift/kf/limited_discount/product_assistant/product_stock/qic/supplier/talent）
- ❌ 视频号 channel 160 个新 bean（after/ewaybill/limit/product/qic/shop/supplier/talent 等分类）
- ❌ 微信支付 V3 服务商电子发票（goldplan/invoice/partner_invoice 等 6 个 service + 数十个 bean）
- ❌ 微信支付商家转账用户授权（pre_transfer / transfer_bills_after_authorization / user_confirm_authorization 等）
- ❌ 微信支付服务商点金计划
- ❌ 企业微信智能机器人 API 模式（WxCpIntelligentRobotService 增强）
- ❌ 企业微信待办 API（WxCpTodoService）
- ❌ 小程序一站式 NFC scheme（已有 service，4.8.5 增强实现未跟进）
- ❌ 微信开放平台商家客服（已有 service，实现细节未跟进）
- ❌ common OCR menu result bean

**抽样中 3 个 OK 的解释**：
- `transfer_bills_request.rs`（4.8.4 即有，4.8.5 增强）
- `wx_ma_scheme_service.rs`（4.8.4 即有，4.8.5 增强实现）
- `wx_open_ma_privacy_service.rs`（4.8.4 即有，4.8.6 增强实现）

## 四、追补优先级建议（基于重要度 × 工作量）

### P0（**立即**追补，1-2 周，4.8.4→4.8.6 新增 **关键商业能力**）
1. **微信支付点金计划**（`#4107`）— 1 service + 5-10 bean
2. **微信支付商家转账用户授权**（`#4044`）— 3 service + 10 bean
3. **微信支付 V3 服务商电子发票**（`#4070`）— 6 service + 30 bean

### P1（**重要**，2-3 周，channel 新功能补齐）
4. **视频号客服/赠品/质检/收藏**（`#4030/#4031/#4039`）— 5 service + 50 bean
5. **视频号商品辅助/库存**（`#4109`）— 2 service + 30 bean
6. **视频号售后保障单**（`#4106`）— ~10 bean
7. **企业微信智能机器人 API 模式增强**（`#4100`）— impl 层补全
8. **企业微信待办 API**（`#4092`）— 1 service + 5 bean

### P2（**后续**，1-2 周，长尾 bean/fix）
9. 视频号电子面单（`#4102`）— 1 service + 30 bean
10. 小程序 NFC scheme 增强（`WxMaSchemeService` impl）
11. 开放平台商家客服（`#4105` 增强 impl）
12. common OCR menu result bean
13. 微信支付 limit_discount 限价折扣 bean

### P3（**非阻塞**）
14. WxJava 文档/SKILL 同步（`#4071`）
15. spring-data-redis BOM 修复（`#4058`，仅 Java 侧）

## 五、追补策略建议

1. **批处理导入**——把 P0 三类作为一次"零碎增量同步"提交，每类一个 commit，沿用现有 V0 审计脚本（`scripts/audit_migration_layout.py`）实时验证 MISSING=0
2. **P1 拆两波**——第一波 channel service 骨架（5 个 service trait 默认 stub），第二波批量补 bean（用 V0 已建立的脚本生成器模式）
3. **P2 单 PR 合并**——长尾 bean 可一次性同步
4. **规范遵循**——每个新文件遵循"WxRust Rust 项目规范"：snake_case 文件名 + PascalCase 类型 + `对应 Java:<ClassName>` 注释 + 一个文件一个 Java 类型

## 六、关键风险

1. **测试缺口**——4.8.5/4.8.6 Java 测试文件 39 个新增（`#4070/#4031/#4105` 等含完整 Mock 链），WxRust 端零对应
2. **API 行为差异**——若新功能已上线给用户使用，缺接口会直接导致线上 404/500
3. **文档同步**——CHANGELOG 与 known-issues 需要追加 4.8.5/4.8.6 缺口条目
4. **覆盖率倒退**——新增 199 个主源不补测试，61.57% 覆盖率必下降

## 七、建议的下一步动作（按风险与工作量排）

```
Day 1-2  : P0 #1 点金计划（1 service 3-5 bean），验证 import & 单元测试
Day 3-5  : P0 #2-#3 转账授权 + V3电子发票（~50 bean，3-5 service）
Day 6-10 : P1 channel 新功能 10 service 骨架（trait 默认 stub）
Day 11-15: P1 channel 50 bean 批量同步
Day 16-20: P1 cp/miniapp 增强
Day 21+  : P2 长尾同步
```

## 八、不在此报告范围（已存在或暂不可行）

- 4.8.4 之前的迁移缺口 → V0 审计已确证 MISSING=0
- 4.8.5 本身的 29 commits 中文档/AGENTS/Star 等非功能性 commit → 已在开头汇总表中筛除
- 4.8.6 的 solon-plugins / spring-boot-starters 集成层 → WxRust 不涉及（无 Java starter 等价物）

