# Task A3 Report: 边界覆盖测试

## Status: DONE

## Commit
- Hash: `0d43d59f2bbd6d433f37de3534ac1015f4811b58`
- Message: `test(common+mp): A3 边界覆盖——crypto/config/menu/template/kefu`

## New Files
| File | Lines | Tests |
|------|-------|-------|
| `crates/wx-rust-common/tests/cov_common_edge.rs` | 508 | 51 |
| `crates/wx-rust-mp/tests/cov_mp_edge.rs` | 687 | 52 |
| **Total** | **1195** | **103** |

## Coverage Breakdown

### cov_common_edge.rs (51 tests)
- **config/mod.rs** (11 tests): token 未设置/永不过期(expires_at=None)/精确到期秒数(now==expires_at)/将过期边界(now+1s)/推进到精确过期/expire_access_token 清除/access_token_lock 共享 Arc/默认标志/set_clock 双调用失败/ticket 默认实现/TicketType 值
- **crypto/sha1** (6 tests): 空数组/含空字符串/单字符/超长>64B 块边界/digest_with_amp 空参/deterministic sorting
- **crypto/pkcs7_encoder** (7 tests): 块对齐填充 32B/差一字节填充 1B/零字节填充/decode 空/decode 合法填充/decode 非法 pad=0/decode 非法 pad>32
- **util/xml_utils** (6 tests): 正常解析/空根/空字符串/未闭合标签/CDATA/同名元素 last wins
- **util/crypto/byte_group** (2 tests): 空组/多次拼接
- **error** (11 tests): WxError Display/错误码 0/负数码/JSON 解析成功/非法 JSON 回退/Display 含 json/WxErrorException from_code/IO 变体无 code/IO 变体无 wx_error/WxRuntimeError Display/Serde 变体 Display
- **util/sign_utils** (1 tests): HmacSHA256 确定性
- **util/data_utils** (2 tests): 脱敏替换/无 secret 不变
- **bean/menu** (2 tests): 空菜单往返/含按钮往返
- **crypto/wx_crypt_util** (3 tests): aesKey 长度错误/正常构造/加解密往返

### cov_mp_edge.rs (52 tests)
- **menu button 类型** (14 tests): view/click/scancode_push/scancode_waitmsg/pic_sysphoto/pic_photo_or_album/pic_weixin/location_select/media_id/view_limited/article_id/article_view_limited/miniprogram/delete 语义
- **WxMpMenu JSON** (3 tests): 含子按钮往返/个性化菜单带规则/menu_id 数字转字符串
- **template message** (10 tests): 基本构造/带 miniprogram/to_json 含 miniprogram/截断 thing*/截断 character_string*/截断 phone_number*/截断 car_number*/截断 const*/短值不截断/with_color
- **template industry** (4 tests): JSON 往返/find_by_class/find_by_code/ALL 共 41 项
- **template bean** (1 test): WxMpTemplate JSON 往返
- **kefu CRUD 请求** (4 tests): account add/update/invite/session create
- **kefu result JSON** (6 tests): KfInfo/KfList/KfOnlineList/SessionGetResult/SessionList/SessionWaitCaseList/KfMsgList
- **kefu 消息类型** (7 tests): text/image/miniprogrampage/msgmenu/mpnewsarticle/非法类型错误/kf_account 路由/空 kf_account 省略

## Quality Gates
- `cargo test -p wx-rust-common`: 284 tests pass (51 new)
- `cargo test -p wx-rust-mp`: 401 tests pass (52 new)
- `cargo clippy -D warnings`: clean
- `cargo fmt --check`: clean
