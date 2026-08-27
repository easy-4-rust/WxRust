# Batch-C 镜像率补测报告

日期：2026-08-27
Commit: `73395eeda426cace34d5a33e04f48d90651e36fb`

## 实测数字（git grep 可复算）

```
Before: 185 / 380 = 48.7%
After:  227 / 380 = 59.7%
Delta:  +42 unique Java test classes, +11.0pp
```

## 新增文件清单

| 文件路径 | 行数 | 镜像 Java 测试类数 |
|---|---|---|
| `crates/wx-rust-cp/tests/batch_c_cp_corpgroup_robot.rs` | 741 | 17 |
| `crates/wx-rust-pay/tests/batch_c_pay_payscore.rs` | 362 | 5 |
| `crates/wx-rust-mp/tests/batch_c_mp_shake_guide.rs` | 337 | 7 |
| `crates/wx-rust-miniapp/tests/batch_c_miniapp_beans.rs` | 263 | 4 |
| `crates/wx-rust-open/tests/batch_c_open_beans.rs` | 289 | 5 |
| `crates/wx-rust-channel/tests/batch_c_channel_order_beans.rs` | 266 | 5 |
| **合计** | **2258** | **43** |

## 模块镜像率变化

| 模块 | Before | After | Delta |
|---|---|---|---|
| cp | 43 | 60 | +17 |
| pay | 39 | 44 | +5 |
| miniapp | 35 | 39 | +4 |
| mp | 30 | 37 | +7 |
| channel | 19 | 24 | +5 |
| open | 16 | 20 | +4 |
| common | 3 | 3 | 0 |
| **合计** | **185** | **227** | **+42** |

## 镜像的 Java 测试类清单

### cp 模块 (17 类)
1. WxCpCorpGroupCorpTest
2. WxCpCorpGroupCorpTokenTest
3. WxCpCorpGroupCorpListAppShareInfoRespTest
4. WxCpCorpGroupCorpGetTokenReqTest
5. WxCpIntelligentRobotTest
6. WxCpIntelligentRobotChatRequestTest
7. WxCpIntelligentRobotChatResponseTest
8. WxCpIntelligentRobotCreateRequestTest
9. WxCpIntelligentRobotCreateResponseTest
10. WxCpIntelligentRobotSendMessageRequestTest
11. WxCpIntelligentRobotSendMessageResponseTest
12. WxCpIntelligentRobotUpdateRequestTest
13. WxCpIntelligentRobotMessageTest
14. WxCpLinkedCorpDepartmentTest
15. WxCpLinkedCorpUserTest
16. WxCpLinkedCorpAgentPermTest
17. WxCpMaTransferSessionTest

### pay 模块 (5 类)
1. WxPartnerPayScoreSignPlanResultTest
2. WxPartnerPayScoreUserSignPlanResultTest
3. PartnerUserSignPlanEntityTest
4. PayScorePlanDetailResultTest
5. PayScorePlanDetailTest

### mp 模块 (7 类)
1. WxMpShakeAroundPageAddResultTest
2. WxMpShakeAroundRelationSearchResultTest
3. WxMpShakeAroundPageAddQueryTest
4. WxMpShakeAroundDeviceBindPageQueryTest
5. WxMpShakeAroundRelationSearchQueryTest
6. WxMpDeviceIdentifierTest
7. WxMpGuideBuyerRespTest

### miniapp 模块 (4 类)
1. WxMaCode2VerifyInfoResultTest
2. WxMaAuditMediaUploadResultTest
3. WxMaMediaAsyncCheckResultTest
4. WxMaApiResponseTest

### open 模块 (5 类)
1. WxOpenAuthorizerAccessTokenTest
2. WxOpenComponentAccessTokenTest
3. WxOpenCreateResultTest
4. WxOpenGetResultTest
5. WxOpenMaCodeTemplateTest

### channel 模块 (5 类)
1. OrderInfoParamTest
2. OrderCouponInfoTest
3. DecodeSensitiveInfoResponseTest
4. OrderAddressInfoTest
5. OrderCustomInfoTest

## 测试结果

```
Baseline: 2912 tests passed
After:    3027 tests passed (+115 new tests)
Failures: 0
```

## 质量门禁

- `cargo clippy --workspace --all-targets -- -D warnings`: PASS
- `cargo fmt --all -- --check`: PASS
- `cargo test --workspace`: 3027 passed, 0 failed

## 证据链路

- Commit hash: `73395eeda426cace34d5a33e04f48d90651e36fb`
- `git show HEAD --stat`: 6 files changed, 2258 insertions(+)
- Mirror rate: `grep -r "对应 Java:" crates/*/tests/*.rs | sed 's/.*对应 Java: //' | grep -oP '^[A-Z][A-Za-z0-9]+Test' | sort -u | wc -l` = 227
