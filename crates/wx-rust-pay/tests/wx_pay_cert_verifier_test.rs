//! wx-rust-pay 平台证书自动更新与验签路由单元测试。
//!
//! 测试对象：`util/crypto/wx_pay_cert_verifier.rs`
//! （对应 Java `v3/auth/CertificatesVerifier` + `v3/auth/AutoUpdateCertificatesVerifier`）。
//!
//! Golden 来源标注：
//! - 下载响应 JSON 结构取自微信支付官方文档《获取平台证书列表》
//!   （https://pay.weixin.qq.com/doc/v3/merchant/401271645 ），`nonce`/
//!   `associated_data` 取官方示例值（`61a9c8685a6f` / `certificate`）；
//! - 加密的证书材料为 official-wechatpay-java SDK 测试资源（平台证书
//!   `wechat_pay_certificate.pem`，已按 P3 惯例重新签发、序列号保持官方
//!   `TestConfig.WECHAT_PAY_CERTIFICATE_SERIAL_NUMBER`；过期原件保留用于
//!   "跳过失效证书" 行为测试）；
//! - `ciphertext` 由 Node.js `crypto`（aes-256-gcm，与 JCE `AES/GCM/NoPadding`
//!   相同原语）独立生成——AES-GCM 原语本身已由官方 SDK 向量验证
//!   （见 `wx_pay_v3_crypto_test.rs::aes_gcm_decrypt_official_sdk_vector`），
//!   故本夹具可视为外部 golden；
//! - RSA-SHA256 验签使用 official-wechatpay-java 的商户/平台私钥（P3 已用
//!   openssl golden 验证签名原语正确性）。

use std::io::{Read, Write};
use std::net::{Shutdown, SocketAddr, TcpListener};
use std::sync::{Arc, Mutex};

use serde_json::json;

use wx_rust_pay::config::r#impl::WxPayDefaultConfig;
use wx_rust_pay::util::crypto::wx_pay_cert_utils::{
    load_certificate_from_pem, load_private_key_from_pem, load_public_key_from_pem,
};
use wx_rust_pay::util::crypto::wx_pay_cert_verifier::{
    CERT_DOWNLOAD_PATH, WxPayAutoUpdateCertificatesVerifier, WxPayCertificatesVerifier,
    deserialize_to_certs,
};
use wx_rust_pay::util::crypto::wx_pay_v3_crypto_utils::{
    AUTHORIZATION_SCHEMA, sign_sha256_rsa, verify_sha256_rsa,
};

// ---- 夹具：密钥/证书材料与 P3 测试（wx_pay_v3_crypto_test.rs）同源 ----

/// APIv3 密钥（official-wechatpay-java `TestConfig.API_V3_KEY`）。
const API_V3_KEY: &str = "a7cde1ZJB1kG2e7VfTs3jQzaWizur8Gb";

/// 商户号（official-wechatpay-java `TestConfig.MERCHANT_ID`）。
const MCH_ID: &str = "1234567891";

/// 商户证书序列号（official-wechatpay-java `TestConfig.MERCHANT_CERTIFICATE_SERIAL_NUMBER`）。
const MERCHANT_CERT_SERIAL_NO: &str = "5F1C72E2A8931B72A2E13AF8DEE92471EB397115";

/// 平台证书序列号（official-wechatpay-java `TestConfig.WECHAT_PAY_CERTIFICATE_SERIAL_NUMBER`）。
const WECHAT_PAY_CERT_SERIAL_NO: &str = "440024045C4A427599D09BB4E3DE0279F2E813FD";

/// 商户 API 私钥（PKCS#8，official-wechatpay-java `merchant_private_key.pem`）。
const MERCHANT_PRIVATE_KEY_PEM: &str = "-----BEGIN PRIVATE KEY-----
MIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQCaZzehUwzcxdLg
Gn/UNryHLdX0yBvCqD0p92/BdlCIBi0dmzZzrfc+FF0xK70AP9b2+Ry5q+zXUU+d
PucJmgwABiZ+Lte+4euMxqPCEkdEu9lyiphalpoaOVPbmDNatzq9k5a/P454QRWM
dkLUJZCeoL9bF2Gn/2+wWEw3sL8zFFcOM8Jr1PdOLmAx+h7pf/87jcmXXCm+SZqw
5MtILKQi9zHWujYdMA0IcYNeQaNl1h/NUnungdIHKaaU+17wCXqTcZsipAGoqfqr
Hx/sr30ZszOdHYOvFNiB+rhEldBGSLWwSYE6LFkbP9GdQWKIQCip3E5dLj5ZFkDe
Ov4Hekf9AgMBAAECggEAEsVsqnS90hNMzUj7dHHJHsgQRGeVlGc+tFzsHcGEDd1u
W7SUfKDQN6BjKgiuvBqGyFTFzL7dltnAS5YroWu0fMZCpMGOIhs2N1Go8/2j43PQ
/k9iMVUw2/JPQxmwWJ2BCy4nvA1+hRkohQCVpFQCzn4tdWYUzcdMrUw2y+h1fkCQ
5MJn7iw9QHKQSeFeCl1/xq2PvOtiK/r1LsckyKNSSNgFEfxyWYaKbnK9OH+5rFKQ
QuI+fnAgE6QiLvmW0NqqZUSfqkLKi/FSWI13ns0H6OxjqpLX8VQ6+Cw5qq8fCuv2
gzkVk8A85ZTCQL/q9qDilt9uAE0bE924WU+n2zkBoQKBgQDQvIufN6fKpm27k4yx
RNV23fj9nojewaVqGg/3yuyiAu6w/yFcTXkGMVOicTYraX2mliHTIoyP8ywKGqqa
XS/Kk3tGD1K04KriPiFwWXU54+DmOJEyYoJlmXOm4BoZ1lW0z5HECC9eO/VPSDY+
zQdRYSCTdSHEgYuOGQSLPPwdrQKBgQC9XTrvkUkIthayc+4IhV6m4kT8uwzuoc3f
uaJhFFcpLKqzcpQBH71TYCXfqkucnO0no0sGerBB4HJQoVRK+jsdhNZhcw1JKLau
E+YlCSRLZ62vyzBTzLw1fnFBp82z1VZBujCrMP+DdwXBTsnkRaUtmDEG0s6YsCwd
fayF0PB9kQKBgAHc/P4R2ByV+brH6WSXsbQa7SMObDhY0CovS18x34Tes9S+okSZ
qG/mttFnY01l5qo7AthIoaqTSBxa+pTgKhIL2PjaICnfK4dTeKbxFXvLzfEgJiOl
/3X6ta6Sp4j9gcxYYfu2+v1DWcA4a8uJtvwB+vF2BTQk1+MP1BuOEs4NAoGBAK3y
+HKdOUPBUPQ4vk4hhaMzcz/d67FB/UYo1lrrPm3aVCxnckHeECKIzgG6A58oIEor
HH4lMcgyD5C1wiLl3mvtXKlD8M5lkfoy2VToIukJomk783bnOXTCY/N12+X4cTYL
fS2k4vK24RiD8b25pFRP26ly+MkV/FBS46pBFsmhAoGAOjfl2vGyJo8CRQ/HBLsS
Bw2VQgRvZU7mom1qa2SKA2VlsFz/aiBCcT8XEsTJxkVYkvbdfbyx2Z9kCbI6Y2cz
Z+M/0rQSRv/eOiBW7anBImZMg6WcYNfRfumkuNq+6fcCKKtZuyrd7ZFc0jTcVasI
xNJ1TM6J6kRQdn5O6Ot4ERw=
-----END PRIVATE KEY-----";

/// 商户 API 证书（有效期 2026-08 ~ 2036-07，序列号保持官方值）。
const MERCHANT_CERT_PEM: &str = "-----BEGIN CERTIFICATE-----
MIIDXzCCAkegAwIBAgIUXxxy4qiTG3Ki4Tr43ukkces5cRUwDQYJKoZIhvcNAQEL
BQAwPzELMAkGA1UEBhMCQ04xITAfBgNVBAoMGEludGVybmV0IFdpZGdpdHMgUHR5
IEx0ZDENMAsGA1UEAwwEdGVzdDAeFw0yNjA4MDEwOTE4NDhaFw0zNjA3MjkwOTE4
NDhaMD8xCzAJBgNVBAYTAkNOMSEwHwYDVQQKDBhJbnRlcm5ldCBXaWRnaXRzIFB0
eSBMdGQxDTALBgNVBAMMBHRlc3QwggEiMA0GCSqGSIb3DQEBAQUAA4IBDwAwggEK
AoIBAQCaZzehUwzcxdLgGn/UNryHLdX0yBvCqD0p92/BdlCIBi0dmzZzrfc+FF0x
K70AP9b2+Ry5q+zXUU+dPucJmgwABiZ+Lte+4euMxqPCEkdEu9lyiphalpoaOVPb
mDNatzq9k5a/P454QRWMdkLUJZCeoL9bF2Gn/2+wWEw3sL8zFFcOM8Jr1PdOLmAx
+h7pf/87jcmXXCm+SZqw5MtILKQi9zHWujYdMA0IcYNeQaNl1h/NUnungdIHKaaU
+17wCXqTcZsipAGoqfqrHx/sr30ZszOdHYOvFNiB+rhEldBGSLWwSYE6LFkbP9Gd
QWKIQCip3E5dLj5ZFkDeOv4Hekf9AgMBAAGjUzBRMB0GA1UdDgQWBBQQicr3Vq+2
NG2ykH0nxQxsct1RtTAfBgNVHSMEGDAWgBQQicr3Vq+2NG2ykH0nxQxsct1RtTAP
BgNVHRMBAf8EBTADAQH/MA0GCSqGSIb3DQEBCwUAA4IBAQBTPKAzNuaP3Z3xBF9e
3WbyR97t3YfI77BwaUJgp6PUPYqTXHN4/cJUDsiOx/GS/TO4RhW0ogWqSOFKFndL
bJLDaweQaOQsQLLAf3eg1YpqVr3qTV7nUKiLbBF3/TSXHjbKa1obI6ju8Wnt8tdZ
+uH8gXdJnCaS6NJAvSQ4a2XVml9GV9b0EUi09YE4ZtSqblhXFoIOK9SqiOj1QMqn
z2G7OY+IrzTKFXue7deavDBy81S7RgObpwyoWh+Iryb3rwekqNwp0fYd01kSFdZ2
86Va/Ynhd3DkreG4BwqJp+BANAtaINW2DpT/odCplRpxX93ODKorvjGQHZmva3R1
ST+q
-----END CERTIFICATE-----";

/// 微信支付平台私钥（official-wechatpay-java `wechat_pay_private_key.pem`，
/// 测试中模拟平台侧签名）。
const WECHAT_PAY_PRIVATE_KEY_PEM: &str = "-----BEGIN PRIVATE KEY-----
MIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQDioFW6YBq3ehY9
1ZW5Tbh+RNUnuGfou+nqbh0Kr53+3Ef0ENoTbO1S24BhNv5Z66BWgNkqmGtBfm2K
yZCr9o3+um3K8FfzEIDOyZcWxhdYiyIb1x1TgUqTbbtqSnli7d3vhNdSMPKKPANq
alDYdlO7w7oJ7K65hjEdylbRWSRK1ogdfO2p8EZXwr0QzrtxtKYL6er4pj44ij2s
dlSQKdwuusF0d/n0iT8p1GwRTUTPNSo97l1VgOy6I/FZC9B/zxKAq7/7/XlgWKaW
kTJr092QJxNXTHL85IiKwgUd6TqP0iLmMu+qFXBpJr7nOSGutKrj9vXSRcuOHcg+
xMN6di3vAgMBAAECggEAZZ1hwxc7c4BU/7XIlBViv49KiixXvxWe/WQRNoznCXMo
+ikiNyqUwR210yEu7A/ITXEprr03/l5bbKsTaN+87HYYK7IXpV+FefMPAYGhzyxX
h2cteZdT4oYFzlOFWRPx+uc0CuDyp4PgPjNce6qWwe6xeKi88WfXlxV2VxkiukDe
JRiqvfOypDbs2oD8fRupBZdIrb30/88ABu1wDg/16/jabCf/fCEyLMfyoZTzrN4E
Dq0rLK7HFbAiVTa2Nc0mfV42rOz/BdFLbt6JfDML1A9NrfM9vNUjLu0rFGVIFXFt
Dc7d88j/xZyM463mCW/Ttmn+uR/UjttumOVaOm6jkQKBgQD3N2ni7vFmlBnzXuy3
wYAPH10GZDJwo5qpHsnp6Gs/VHQspnCZFu+FQMI7lADP1fhqyf8X5HRS7NVkdus3
SagrisFp/y1j1SKZBH7e7citsGsmEMLtrKTyh148FNS8VLTFCgbzdNwiPi/5UBD1
nfzSLGPFdnDBngzvIb91MgrfvwKBgQDqraQf/HTZIqkj3eMwyLS9GwZpQXZ3fVpj
XOw1crPpEXd/s098C5ahgSxdXjAsAVrh79Buz48ze4BWhM5t/Oh3eLDDgsW57q3y
WoMxX/39CRMe+DyFIl1RtCB/86iz0cjsYxeCnuF35FWyBHco8qsFgbDXtORp3I0v
94nqWPs90QKBgHi/Ph6zaoEZ0VsHgEN4ZZ39Sah3hBN0NXRpL2HqPYffLico3FJJ
24+QQ85pycO51O4D327CDPb76nZ5hwWoGlW6HiIxmu/qCE4I3hbwuVXsl0kuYqdH
7gDvkV1zJz0denXLIf4DlAm9qhzxuHbfRb86IzktUoGfOt4J88PAP4NtAoGBALnu
oV6GUr3GbGPyZRI9spUxWD8EkqvRMVAQS0V9f3VlVVxj4NHTDKEhCtleyqPS3wKr
UmS5JigP7Xqe9j7GZhhiEwqUDbxmkoK1m8P+3ekgy3l6C9kfhPg60pUNo9f0mlBP
okznPJQU4KCOUSqsOTu/qTD+LQqCp4odgbOelFBBAoGAJyhBwb2NN7Pia4CdN2Pd
ukN4ZYWH+ImhwHbuMB0ap633oKci8WpCBn1gEsvI8MiSQLoXGyY0NULt+4/zY6Qe
X0BhQOC+abKri6mKSJt61cKhmYeNZvQon/Y9lKAGTuOS/a4NiebKytmyArbHKZ3J
4iWXZYuCrKtcQEU4nAY/FgA=
-----END PRIVATE KEY-----";

/// 微信支付平台证书（有效期 2026-08 ~ 2036-07，序列号保持官方值）。
const WECHAT_PAY_CERT_PEM: &str = "-----BEGIN CERTIFICATE-----
MIIDNzCCAh+gAwIBAgIURAAkBFxKQnWZ0Ju0494CefLoE/0wDQYJKoZIhvcNAQEL
BQAwKzELMAkGA1UEBhMCQ04xDTALBgNVBAoMBHRlc3QxDTALBgNVBAMMBHRlc3Qw
HhcNMjYwODAxMDkxOTAwWhcNMzYwNzI5MDkxOTAwWjArMQswCQYDVQQGEwJDTjEN
MAsGA1UECgwEdGVzdDENMAsGA1UEAwwEdGVzdDCCASIwDQYJKoZIhvcNAQEBBQAD
ggEPADCCAQoCggEBAOKgVbpgGrd6Fj3VlblNuH5E1Se4Z+i76epuHQqvnf7cR/QQ
2hNs7VLbgGE2/lnroFaA2SqYa0F+bYrJkKv2jf66bcrwV/MQgM7JlxbGF1iLIhvX
HVOBSpNtu2pKeWLt3e+E11Iw8oo8A2pqUNh2U7vDugnsrrmGMR3KVtFZJErWiB18
7anwRlfCvRDOu3G0pgvp6vimPjiKPax2VJAp3C66wXR3+fSJPynUbBFNRM81Kj3u
XVWA7Loj8VkL0H/PEoCrv/v9eWBYppaRMmvT3ZAnE1dMcvzkiIrCBR3pOo/SIuYy
76oVcGkmvuc5Ia60quP29dJFy44dyD7Ew3p2Le8CAwEAAaNTMFEwHQYDVR0OBBYE
FCcpJ5TvknGPLSSulN436iJLfxaSMB8GA1UdIwQYMBaAFCcpJ5TvknGPLSSulN43
6iJLfxaSMA8GA1UdEwEB/wQFMAMBAf8wDQYJKoZIhvcNAQELBQADggEBADW/wu/P
4xgnguiHaZAmV2eN8qLb5GIJl1068o2J8rohkOVN++havkqYDpyWggL9GYsR8q+O
EJKWAZV4XhGxcIX5TxCrHhP2KiTCgh4gxr+VMkuGt6hzm5YMbzz7tjaVS+1/cnch
x7e4jShlWvygb3cwmQFaE70eaeMqyxubncZN5sR9pPDrk2ELEBILGUnhkX3V8BGM
LjTT8Yu9xiF0/z9cTVJ1A3W/AMB1m4wpG3K/vcR0gpTBHbIgyksSzxJHiqVBRGgM
+jfCyYuMoSCqj10i0jntpUOMkW+MSisLfAzRnShSjqEnIoJiFl1GzWze2v+KzzcY
Kj/YB4YRMULsAp8=
-----END CERTIFICATE-----";

/// 微信支付平台公钥（official-wechatpay-java `wechat_pay_public_key.pem`）。
const WECHAT_PAY_PUBLIC_KEY_PEM: &str = "-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA4qBVumAat3oWPdWVuU24
fkTVJ7hn6Lvp6m4dCq+d/txH9BDaE2ztUtuAYTb+WeugVoDZKphrQX5tismQq/aN
/rptyvBX8xCAzsmXFsYXWIsiG9cdU4FKk227akp5Yu3d74TXUjDyijwDampQ2HZT
u8O6CeyuuYYxHcpW0VkkStaIHXztqfBGV8K9EM67cbSmC+nq+KY+OIo9rHZUkCnc
LrrBdHf59Ik/KdRsEU1EzzUqPe5dVYDsuiPxWQvQf88SgKu/+/15YFimlpEya9Pd
kCcTV0xy/OSIisIFHek6j9Ii5jLvqhVwaSa+5zkhrrSq4/b10kXLjh3IPsTDenYt
7wIDAQAB
-----END PUBLIC KEY-----";

// ---- Golden 夹具：/v3/certificates 下载响应（密文由 Node.js crypto 独立生成）----

/// 有效平台证书的密文（official 文档示例 nonce `61a9c8685a6f` + aad
/// `certificate`；Node.js `crypto.createCipheriv('aes-256-gcm', ...)` 生成，
/// Base64(密文 || 16 字节 tag)）。
const VALID_PLATFORM_CERT_CIPHERTEXT_B64: &str = "5bXJ1U1piGm/ypGcB5doDh+9tgNwjCcZILywYlzKWzxtlwqenRWpTaGoJLbhy/zuapeBfeSXvAZQdv575Yp2H0P/L08aSKaxcLdtuk6KtVmDSEfAf9dBGqsn4gIcbiOOs7Ht8LJTMvJN0qMfYJVzwob4wxFITcs10R152wa3iHQt0EV8DywZBt1Dl9TSJ7zeQrrP4fgdiAbSRJIxMVON6Ey8yk5ovezQpyMydexr9URY8KbVeH9gmx6YP0Z50cHQAJ84UGDCUJQibZPzUNqO7Xvgm29p22Nnu4fkOjICcDuD8BWkWUJEtqFGlo9kGw5WGPd44gsgIl+a40j9/NGmfGFx/WfWCl6FVtSZCNdyvhS38GqPk1WhEcsXcoH2OvCb3NiI3iJJF3tHkzls1accnsqrKyBjf8mnxD+EoEJphEoUf08ED0ZEaQqUhm/sBCrvolJmGmEeRU8gUZH4+PaJSDcq7RX51BHRK9MLmu8S5A0krejqVZg5cVJsGf8o5zpHEPhK6zz2rJDX97GEBir2Eszc5fnodu0QyIi9Nbw62apcO34caiB0ShWEIC7TTN54YWcvU4wsAXjDefId0Zcz3qWzuI8FUUfKg/g9K5+gLRTLSOC+5+N82EgZ1l9B6Tok2zMZQymcgnfMn+pc1TsopabOziBiSMAEazVL3cZMu+pQ31WaD5RpYH5aCsvYhvJWbXWGXbLNBzIyCQ7wzBmd3dXEOxwYXAdMh8hnw2uGlPcIo7nyizxadM0aw2jmx3kuk11XttT3cZp9kvs+OoEdS4V7c3CLlM7RxQzSwSnkgqUSbRrJ7BhUYUiT4ftJwWUbk+iijvLQkDLhT4H/X8pjdDmYk3gFs06LUUsxYMX86AZSXzW+fiyMG5c/KgrNLT4rEsIDJUaaFU4sC50atHIAhKFxC5npZz1uSuprFn2fEEOouXW3JmiXIzNpPixP/gpL+j1FvEgIH5yuwh2XfaJtExghdcFYCuHFR+5PVT3wYPhK4uylsvXgOjgPINFp3Yfbb3qp5qYvwW3lVAF+t1pBgCbA8Fg/wDVVkKgyQS7YprYu4/cN+M08jC9UWxTytDL5NcJjtdjz1feTLijecxEZ+eXKFcZI+mUrnlaPIzOj+LhUDlLaAEUWBJ5dTqHqKzFHnXT0rLyaJatBHS3BPghFGHAgl4FtiMu/9aT8krd3rD92AQ1NlR0ECIKhqy2zhRLSZODminSK10cR+0z/eZ6JQ9PazGRFDku1jynyxlKutOv0O5suM9Cx+Q3FqaNZYL9jzyPwzOCCVRPJCL7iaZl8QfHVrEfy4Ov21NAvAj8hzSj9wMu7i96rNnJDRL/Gpe2O7ADOP8SKGQSz/Dkg2KIph+pwJPNDZOB/GYn9Li+Sk/KXnJ4A4pNxCM01l73p2Qi7MpfvIbwEorlk3zFLFP619RgqyYlfP6kwJQBEwzqbcc+KlKU2O04QlveQyzNefFrT6R8YikQJuDSAQa+GnViTzHD7PC2qnc68a4hWxpdC9zABGvZ7IzhWgoEsiK1zvNAUoeqm8QfClWuiM+R0NKzTCNCDRrqwJ2XRdIc0CXYreLPAS0GLlv1fiA==";

/// 已过期平台证书的密文（nonce `7b8c9d0e1f2a` + aad `certificate`；用于验证
/// `checkValidity()` 跳过失效证书的 Java 语义）。
const EXPIRED_PLATFORM_CERT_CIPHERTEXT_B64: &str = "dFwbV6G3+8ctLHSB8nlc4jzHBh3NWX+p5oGCX3qXnWuQUJGgmYzYp+JnCN+RHy9adDoBVWIxy07Mj9SglOAXpFvDZnO0qPDQRPoZL4Tw5DrXMEKJTWimT74VYNS7eiOD1ofSoQkgk2DTY1O4ECTtFr/bstLsDuD/Jc+PA1FlxgXE6rQjY3i/WlGSva7Qidw11JSF1tki2/MMJh03F+oYqpEjy2Empkunl4bTrtS8LC6PSKdmk2y3rBb1JizGB1wKjxraCbPlkqsfAcR4A1+yGsp+huRbHkytWg8MPD/S9TCLdk5h0nukKZ1KPotz9QHVTPCGeUTkLvheDOsbcwpL5bUTL8lh0Ua2B45ivB/iiVAFx6O2QIvnUmHWcATldHvYlsDgjt9GsScWJX9xCEgU9OO1nbh+gQGF+V1E6kZ6D4Orw3ryYT92tgbTqCIgmgKNywXdEuSTO2+SQ5DUTl8A1MWkrSw2erlmml5W23752ljYVnqLUgTi57Hl1FxmCZqzUHuqTjitDUFFKw3iiIDjXlTWputsILxFyX6fsK3paBpqhs0hw2FMk/Wj7HL3h80l2AsIVGDeU39ApsRWWKl1J4kbX868YSYc1F99p+IdklgcZjucYqkzWk+piqdKSRnWFKQhusGodb+dfQe+l5oczkxfxBgWrpoxJVv7Ymnt/NvB/6hRree1j+fJ9X4NDE6IAPzZX+QCBcUVcLlVoyNEDqTxt1glrzy5o4IPZPDJh5/IYf1itaJfVHuw3I/zmt6IFEZaqpQrbGMiF59Js4+ms2+4rua3ACS6bqmf5QIoEDH4+FnoiaLWbw+1+fKaINCYd2kmPHP4e77K0mX90MVAlEQe4BmPjQzVzerIxREZ7/qrP4/FkHyrMgvIJFUWFbRFuxAL+8wRaw+pJWT7F5ocBZ1CxQDZHeK4U9lhCd5VclGRLxQGxzplu8aVjaiU/FEPM9EY3lNzij5Uqr5r/DZk8Kg2nX0hxVW+XWmnemOMjS9bUtwk5+iTxsv6FH+tPl9c0/xUgJ7zh8fCXC0Q3412s3PFEL5IGWC6HCRFPi0DCCt6RtVW8HwhfrXYXuxAnpl1wXQSuR8q6UNUKwBmYr5FRAWVUM1HcDsRWu4bVXgD5paV0pw/Pyc6+7xocQkcfRZlpOe1uckerDckxkmjnZSU2/Xu+WOvMPkcmRVVhrQa7axXUz9aENdugtoz+16HjXel92QYiIL+Pz9xL01HUPuRKSpkAQJHBWUiCuREaBn1OIvoJTwDNKGYfv8waANKBh0zg0G4eX/ckWv9IsMrPuJicf2rY/lFAkyyu/JFzhCgmnmc4iqgfPVCwSMGPffovwUwMxZFSQgT/1WGxBtEwFNPagz3rSFU2CFwx+sXirHh6QQi8lKoIEHA2mFV8HwTpDUqb+wdOg4jxjLQFaI/F5M79q29VpdXS7tvlbWJyOEiGhcHHoZCujsiYB5FYmDxLs3PJV4JOfV2QsbZApXxaE7Wp/W5UthQkp7hRJcH8K19jG/Peph0aucMIampVN2YsZtT3kxHMr2XDv9wuaxv6dgqqgyKvYz1YQCzipVuAMlTouqOxYTspc2Bxs15cxhh00y5AlIVhPeQNYlf+dxlAfm0hB0LaEnHEzNWC1Uo88fPXJ091+TtkUw/MBMFqc3e19/h24J0zbEd/HNhvRPltGvSKZ4iUfUAg4n6UhPQ1/kZfWJ5vRRwhg406Vgpa9zdXRTeMNmPN87D66n4TS8G4LYbDS5Z/TqUCEEaPXiI97/4RaAWGuQ/HrufH4Bl";

/// 构建官方文档结构的下载响应 JSON（单条有效平台证书）。
fn certificates_response_body() -> String {
    json!({
        "data": [
            {
                "serial_no": WECHAT_PAY_CERT_SERIAL_NO,
                "effective_time": "2026-08-01T17:19:00+08:00",
                "expire_time": "2036-07-29T17:19:00+08:00",
                "encrypt_certificate": {
                    "algorithm": "AEAD_AES_256_GCM",
                    "nonce": "61a9c8685a6f",
                    "associated_data": "certificate",
                    "ciphertext": VALID_PLATFORM_CERT_CIPHERTEXT_B64
                }
            }
        ]
    })
    .to_string()
}

/// 双条目下载响应：第一条为有效平台证书，第二条为已过期证书（同序列号，
/// 用于验证 `checkValidity` 跳过语义）。
fn certificates_response_with_expired_body() -> String {
    json!({
        "data": [
            {
                "serial_no": WECHAT_PAY_CERT_SERIAL_NO,
                "effective_time": "2026-08-01T17:19:00+08:00",
                "expire_time": "2036-07-29T17:19:00+08:00",
                "encrypt_certificate": {
                    "algorithm": "AEAD_AES_256_GCM",
                    "nonce": "61a9c8685a6f",
                    "associated_data": "certificate",
                    "ciphertext": VALID_PLATFORM_CERT_CIPHERTEXT_B64
                }
            },
            {
                "serial_no": WECHAT_PAY_CERT_SERIAL_NO,
                "effective_time": "2022-05-10T02:04:30+08:00",
                "expire_time": "2025-05-09T02:04:30+08:00",
                "encrypt_certificate": {
                    "algorithm": "AEAD_AES_256_GCM",
                    "nonce": "7b8c9d0e1f2a",
                    "associated_data": "certificate",
                    "ciphertext": EXPIRED_PLATFORM_CERT_CIPHERTEXT_B64
                }
            }
        ]
    })
    .to_string()
}

// ---- Mock HTTP 服务器（对应 Java 测试的 mockServer / 门面 API 服务器）----

/// 捕获到的请求。
#[derive(Debug, Clone)]
struct MockRequest {
    method: String,
    path: String,
    headers: Vec<(String, String)>,
}

impl MockRequest {
    fn header(&self, name: &str) -> Option<&str> {
        self.headers
            .iter()
            .find(|(k, _)| k.eq_ignore_ascii_case(name))
            .map(|(_, v)| v.as_str())
    }
}

/// 启动单次响应的 mock HTTP 服务器。
///
/// `handler` 根据捕获到的请求返回 `(状态码, 响应体)`；请求（请求行/头）写入
/// `captured` 供断言。线程在测试进程退出时结束。
fn spawn_mock_server<F>(handler: F) -> (SocketAddr, Arc<Mutex<Vec<MockRequest>>>)
where
    F: Fn(&MockRequest) -> (u16, String) + Send + 'static,
{
    let listener = TcpListener::bind("127.0.0.1:0").expect("绑定 mock 端口");
    let addr = listener.local_addr().expect("获取 mock 地址");
    let captured = Arc::new(Mutex::new(Vec::new()));
    let captured_clone = Arc::clone(&captured);
    std::thread::spawn(move || {
        for stream in listener.incoming() {
            let Ok(mut stream) = stream else { continue };
            let mut buf = Vec::new();
            let mut tmp = [0u8; 8192];
            // 读至请求头结束（\r\n\r\n）；GET 无请求体
            loop {
                let n = stream.read(&mut tmp).unwrap_or(0);
                if n == 0 {
                    break;
                }
                buf.extend_from_slice(&tmp[..n]);
                if buf.windows(4).any(|w| w == b"\r\n\r\n") {
                    break;
                }
            }
            let text = String::from_utf8_lossy(&buf);
            let mut lines = text.lines();
            let request_line = lines.next().unwrap_or_default().to_string();
            let mut headers = Vec::new();
            for line in lines {
                if let Some((k, v)) = line.split_once(':') {
                    headers.push((k.trim().to_string(), v.trim().to_string()));
                }
            }
            let parts: Vec<&str> = request_line.split_whitespace().collect();
            let request = MockRequest {
                method: parts.first().unwrap_or(&"").to_string(),
                path: parts.get(1).unwrap_or(&"").to_string(),
                headers,
            };
            captured_clone
                .lock()
                .expect("captured 锁")
                .push(request.clone());

            let (status, body) = handler(&request);
            let reason = if status == 200 { "OK" } else { "ERROR" };
            let resp = format!(
                "HTTP/1.1 {status} {reason}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                body.len()
            );
            let _ = stream.write_all(resp.as_bytes());
            let _ = stream.shutdown(Shutdown::Both);
        }
    });
    (addr, captured)
}

/// 解析 v3 Authorization 头 token（`WECHATPAY2-SHA256-RSA2048
/// mchid="..",nonce_str="..",timestamp="..",serial_no="..",signature=".."`），
/// 返回 (mchid, nonce_str, timestamp, serial_no, signature)。
fn parse_auth_header(value: &str) -> Option<(String, String, String, String, String)> {
    let token = value.strip_prefix(&format!("{AUTHORIZATION_SCHEMA} "))?;
    let mut fields = std::collections::HashMap::new();
    for part in token.split(',') {
        let (k, v) = part.split_once('=')?;
        fields.insert(k.to_string(), v.trim_matches('"').to_string());
    }
    Some((
        fields.get("mchid")?.clone(),
        fields.get("nonce_str")?.clone(),
        fields.get("timestamp")?.clone(),
        fields.get("serial_no")?.clone(),
        fields.get("signature")?.clone(),
    ))
}

/// 构建带商户凭据的默认配置（apiHostUrl 指向 mock 服务器）。
fn test_config(host: &str) -> WxPayDefaultConfig {
    let mut config = WxPayDefaultConfig::new();
    config
        .set_mch_id(MCH_ID)
        .set_cert_serial_no(MERCHANT_CERT_SERIAL_NO)
        .set_private_key(MERCHANT_PRIVATE_KEY_PEM)
        .set_api_v3_key(API_V3_KEY)
        .set_api_host_url(host);
    config
}

// ---- 测试 ----

/// 官方下载响应 JSON golden 解密（对应 Java `deserializeToCerts`：
/// data[] → AES-256-GCM 解密 → PemUtils.loadCertificate → 序列号/公钥提取）。
#[test]
fn certificates_response_golden_decrypt() {
    let certs = deserialize_to_certs(API_V3_KEY, &certificates_response_body()).expect("解密失败");
    assert_eq!(certs.len(), 1);
    assert_eq!(certs[0].serial_no(), WECHAT_PAY_CERT_SERIAL_NO);
    // 解密出的公钥与平台公钥 PEM 一致
    let expected = load_public_key_from_pem(WECHAT_PAY_PUBLIC_KEY_PEM.as_bytes()).expect("公钥");
    assert_eq!(certs[0].public_key().expect("证书公钥"), expected);

    // Java `dataNode == null` → 空列表（不报错）
    let empty = deserialize_to_certs(API_V3_KEY, r#"{"foo":"bar"}"#).expect("解析");
    assert!(empty.is_empty());
}

/// 双条目响应：过期证书被 `checkValidity` 跳过（对应 Java catch
/// `CertificateExpiredException | CertificateNotYetValidException` 后
/// `continue`）。
#[test]
fn certificates_response_skips_expired_entry() {
    let certs = deserialize_to_certs(API_V3_KEY, &certificates_response_with_expired_body())
        .expect("解密失败");
    assert_eq!(certs.len(), 1);
    assert_eq!(certs[0].serial_no(), WECHAT_PAY_CERT_SERIAL_NO);
}

/// 解密/解析失败语义（对应 Java `GeneralSecurityException` 使整个更新失败）：
/// 篡改密文、非 32 字节密钥、缺字段。
#[test]
fn certificates_response_rejects_tampered_and_malformed() {
    // 篡改密文一个字节 → AES-GCM 认证失败
    let body = certificates_response_body().replace(
        VALID_PLATFORM_CERT_CIPHERTEXT_B64,
        &tamper_base64(VALID_PLATFORM_CERT_CIPHERTEXT_B64),
    );
    let err = deserialize_to_certs(API_V3_KEY, &body).expect_err("应解密失败");
    assert!(err.to_string().contains("解密失败"), "{err}");

    // 非 32 字节 apiV3Key（对应 Java AesUtils 构造 IllegalArgumentException）
    let err =
        deserialize_to_certs("short-key", &certificates_response_body()).expect_err("应拒绝密钥");
    assert!(err.to_string().contains("无效的ApiV3Key"), "{err}");

    // 缺 encrypt_certificate.nonce → 更新失败（对应 Java NPE 使更新失败的语义）
    let bad = json!({
        "data": [
            {
                "serial_no": WECHAT_PAY_CERT_SERIAL_NO,
                "encrypt_certificate": {
                    "algorithm": "AEAD_AES_256_GCM",
                    "associated_data": "certificate",
                    "ciphertext": VALID_PLATFORM_CERT_CIPHERTEXT_B64
                }
            }
        ]
    })
    .to_string();
    let err = deserialize_to_certs(API_V3_KEY, &bad).expect_err("应报错");
    assert!(
        err.to_string().contains("缺少 encrypt_certificate.nonce"),
        "{err}"
    );

    // 响应体不是 JSON
    let err = deserialize_to_certs(API_V3_KEY, "not-json").expect_err("应报错");
    assert!(err.to_string().contains("证书响应解析失败"), "{err}");
}

/// serial 路由命中/未命中（对应 Java `CertificatesVerifier.verify`：
/// `certificates.containsKey(val) && verify(...)`）。
#[test]
fn verify_routes_by_serial() {
    let platform_cert =
        load_certificate_from_pem(WECHAT_PAY_CERT_PEM.as_bytes()).expect("平台证书");
    let merchant_cert = load_certificate_from_pem(MERCHANT_CERT_PEM.as_bytes()).expect("商户证书");
    let verifier = WxPayCertificatesVerifier::with_certificates(vec![platform_cert, merchant_cert]);

    let message = "1700000000\nr0uYIzEaIUX9\n{\"id\":\"EV-1\"}\n";
    let platform_key =
        load_private_key_from_pem(WECHAT_PAY_PRIVATE_KEY_PEM.as_bytes()).expect("平台私钥");
    let signature = sign_sha256_rsa(&platform_key, message.as_bytes()).expect("签名");

    // 命中平台序列号 → 验签通过
    assert!(verifier.verify(WECHAT_PAY_CERT_SERIAL_NO, message.as_bytes(), &signature));
    // Java `new BigInteger(serialNumber, 16)` 大小写不敏感 → 小写序列号同样命中
    assert!(verifier.verify(
        &WECHAT_PAY_CERT_SERIAL_NO.to_ascii_lowercase(),
        message.as_bytes(),
        &signature
    ));
    // 序列号命中但证书不匹配（商户证书公钥验平台签名）→ false
    assert!(!verifier.verify(MERCHANT_CERT_SERIAL_NO, message.as_bytes(), &signature));
    // 未知序列号 → false（Java containsKey 短路语义，不抛异常）
    assert!(!verifier.verify("DEADBEEF", message.as_bytes(), &signature));
    // Base64 非法签名 → 归为验签失败返回 false
    assert!(!verifier.verify(
        WECHAT_PAY_CERT_SERIAL_NO,
        message.as_bytes(),
        "!!not-base64!!"
    ));
}

/// 篡改消息/签名拒绝（对应 Java `Signature.verify` 返回 false）。
#[test]
fn verify_rejects_tampered() {
    let platform_cert =
        load_certificate_from_pem(WECHAT_PAY_CERT_PEM.as_bytes()).expect("平台证书");
    let verifier = WxPayCertificatesVerifier::with_certificates(vec![platform_cert]);

    let message = "1700000000\nr0uYIzEaIUX9\n{\"id\":\"EV-1\"}\n";
    let platform_key =
        load_private_key_from_pem(WECHAT_PAY_PRIVATE_KEY_PEM.as_bytes()).expect("平台私钥");
    let signature = sign_sha256_rsa(&platform_key, message.as_bytes()).expect("签名");

    // 篡改消息
    let tampered_message = format!("{message}tampered");
    assert!(!verifier.verify(
        WECHAT_PAY_CERT_SERIAL_NO,
        tampered_message.as_bytes(),
        &signature
    ));
    // 篡改签名（翻转一个字节）
    let tampered_signature = tamper_base64(&signature);
    assert!(!verifier.verify(
        WECHAT_PAY_CERT_SERIAL_NO,
        message.as_bytes(),
        &tampered_signature
    ));
}

/// set_certificates 手动注入 + 查询（对应 Java `CertificatesVerifier` 构造/
/// `certificateMap` 整体替换语义）。
#[test]
fn set_certificates_manual_injection() {
    let platform_cert =
        load_certificate_from_pem(WECHAT_PAY_CERT_PEM.as_bytes()).expect("平台证书");
    let mut verifier = WxPayCertificatesVerifier::new();

    // 空存储
    assert!(verifier.is_empty());
    assert!(verifier.get_serial_numbers().is_empty());
    assert!(
        verifier
            .get_certificate(WECHAT_PAY_CERT_SERIAL_NO)
            .is_none()
    );
    assert!(!verifier.verify(WECHAT_PAY_CERT_SERIAL_NO, b"msg", "sig"));
    let err = verifier.get_valid_certificate().expect_err("应无有效证书");
    assert!(
        err.to_string().contains("没有有效的微信支付平台证书"),
        "{err}"
    );

    // 注入一张证书 → 覆盖式写入
    verifier.set_certificates(vec![platform_cert]);
    assert_eq!(verifier.len(), 1);
    assert_eq!(
        verifier.get_serial_numbers(),
        vec![WECHAT_PAY_CERT_SERIAL_NO.to_string()]
    );
    assert!(
        verifier
            .get_certificate(WECHAT_PAY_CERT_SERIAL_NO)
            .is_some()
    );
    assert_eq!(
        verifier
            .get_valid_certificate()
            .expect("有效证书")
            .serial_no(),
        WECHAT_PAY_CERT_SERIAL_NO
    );

    // 空列表整体替换 → 清空
    verifier.set_certificates(vec![]);
    assert!(verifier.is_empty());
}

/// 自动更新验签器的手动注入：注入视为一次有效更新（对应 Java 构造成功后
/// `instant = now`），`need_update` 为 false。
#[test]
fn auto_verifier_manual_injection_and_verify() {
    let platform_cert =
        load_certificate_from_pem(WECHAT_PAY_CERT_PEM.as_bytes()).expect("平台证书");
    let verifier = WxPayAutoUpdateCertificatesVerifier::new();

    // 无证书：verify 返回 false（对应 Java verifier == null 时 warn + false）
    assert!(!verifier.verify(WECHAT_PAY_CERT_SERIAL_NO, b"msg", "sig"));
    let err = verifier.get_valid_certificate().expect_err("应无证书");
    assert!(
        err.to_string()
            .contains("没有有效的证书可用，请检查配置或使用公钥模式"),
        "{err}"
    );
    assert!(verifier.need_update());

    verifier.set_certificates(vec![platform_cert]);
    assert_eq!(
        verifier.get_serial_numbers(),
        vec![WECHAT_PAY_CERT_SERIAL_NO.to_string()]
    );
    assert!(!verifier.need_update());

    let message = "1700000000\nr0uYIzEaIUX9\n{\"id\":\"EV-1\"}\n";
    let platform_key =
        load_private_key_from_pem(WECHAT_PAY_PRIVATE_KEY_PEM.as_bytes()).expect("平台私钥");
    let signature = sign_sha256_rsa(&platform_key, message.as_bytes()).expect("签名");
    assert!(verifier.verify(WECHAT_PAY_CERT_SERIAL_NO, message.as_bytes(), &signature));
}

/// auto_update 全流程（对应 Java `autoUpdateCert` + `checkAndAutoUpdateCert`）：
/// mock 服务器校验 v3 Authorization 头签名后返回官方结构下载 JSON；
/// 更新成功后按序列号路由验签通过，且间隔未到时不再刷新。
#[tokio::test]
async fn auto_update_full_flow_with_mock_server() {
    // mock 服务器：用商户证书公钥验证 Authorization 头签名（对应微信服务端
    // 校验商户请求签名的行为），验签失败返回 401
    let merchant_pubkey = load_certificate_from_pem(MERCHANT_CERT_PEM.as_bytes())
        .expect("商户证书")
        .public_key()
        .expect("商户公钥");
    let golden_body = certificates_response_body();
    let (addr, captured) = spawn_mock_server(move |req: &MockRequest| {
        let auth = req.header("Authorization").expect("缺少 Authorization 头");
        let (mchid, nonce, timestamp, serial_no, signature) =
            parse_auth_header(auth).expect("Authorization 头格式错误");
        assert_eq!(mchid, MCH_ID);
        assert_eq!(serial_no, MERCHANT_CERT_SERIAL_NO);
        let message = format!("GET\n/v3/certificates\n{timestamp}\n{nonce}\n\n");
        if verify_sha256_rsa(&merchant_pubkey, message.as_bytes(), &signature).unwrap_or(false) {
            (200, golden_body.clone())
        } else {
            (
                401,
                r#"{"code":"SIGN_ERROR","message":"验签失败"}"#.to_string(),
            )
        }
    });

    let host = format!("http://{addr}");
    let config = test_config(&host);
    let verifier = WxPayAutoUpdateCertificatesVerifier::new();
    let client = reqwest::Client::new();

    // 更新前无证书
    assert!(verifier.get_serial_numbers().is_empty());

    let result = verifier
        .auto_update_certificates(&config, &client)
        .await
        .expect("自动更新失败");
    assert!(result.refreshed);
    assert_eq!(result.updated_cert_count, 1);
    assert_eq!(
        result.serial_numbers,
        vec![WECHAT_PAY_CERT_SERIAL_NO.to_string()]
    );

    // 请求头断言：GET /v3/certificates + Accept + Wechatpay-Serial（严格模式）
    // 快照后立即释放锁，避免 MutexGuard 跨 await
    let requests = captured.lock().expect("captured 锁").clone();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, "GET");
    assert_eq!(requests[0].path, CERT_DOWNLOAD_PATH);
    assert_eq!(requests[0].header("Accept"), Some("application/json"));
    assert_eq!(
        requests[0].header("Wechatpay-Serial"),
        Some(MERCHANT_CERT_SERIAL_NO)
    );
    assert!(
        requests[0]
            .header("Authorization")
            .unwrap_or_default()
            .starts_with(&format!("{AUTHORIZATION_SCHEMA} "))
    );

    // 更新后的证书可对平台签名验签（对应 Java 下载后 verifier 生效）
    let message = "1700000000\nr0uYIzEaIUX9\n{\"id\":\"EV-2\"}\n";
    let platform_key =
        load_private_key_from_pem(WECHAT_PAY_PRIVATE_KEY_PEM.as_bytes()).expect("平台私钥");
    let signature = sign_sha256_rsa(&platform_key, message.as_bytes()).expect("签名");
    assert!(verifier.verify(WECHAT_PAY_CERT_SERIAL_NO, message.as_bytes(), &signature));

    // get_valid_certificate 返回有效证书（对应 Java getValidCertificate）
    assert_eq!(
        verifier
            .get_valid_certificate()
            .expect("有效证书")
            .serial_no(),
        WECHAT_PAY_CERT_SERIAL_NO
    );

    // 间隔未到：check_and_auto_update 跳过刷新（对应 Java `instant.plus(
    // minutesInterval) > now` 直接返回）
    let skip = verifier
        .check_and_auto_update(&config, &client)
        .await
        .expect("检查更新失败");
    assert!(!skip.refreshed);
    assert_eq!(
        skip.serial_numbers,
        vec![WECHAT_PAY_CERT_SERIAL_NO.to_string()]
    );
}

/// 配置 apiHostUrlPath（网关代理前缀）时：请求 URL 带前缀，但 Authorization
/// 签名串剥离开头路径前缀（对应 Java `WxPayCredentials.stripPathPrefix` +
/// VerifierBuilder 的 `signUriStripPrefix`）。
#[tokio::test]
async fn auto_update_strips_path_prefix_in_signature() {
    let merchant_pubkey = load_certificate_from_pem(MERCHANT_CERT_PEM.as_bytes())
        .expect("商户证书")
        .public_key()
        .expect("商户公钥");
    let golden_body = certificates_response_body();
    let (addr, captured) = spawn_mock_server(move |req: &MockRequest| {
        // 请求 URL 应带路径前缀
        assert_eq!(req.path, "/api-weixin/v3/certificates");
        let auth = req.header("Authorization").expect("缺少 Authorization 头");
        let (_, nonce, timestamp, _, signature) =
            parse_auth_header(auth).expect("Authorization 头格式错误");
        // 签名串应使用剥离前缀后的 `/v3/certificates`
        let message = format!("GET\n/v3/certificates\n{timestamp}\n{nonce}\n\n");
        if verify_sha256_rsa(&merchant_pubkey, message.as_bytes(), &signature).unwrap_or(false) {
            (200, golden_body.clone())
        } else {
            (401, r#"{"message":"验签失败"}"#.to_string())
        }
    });

    let mut config = test_config(&format!("http://{addr}"));
    config.set_api_host_url_path("/api-weixin");
    let verifier = WxPayAutoUpdateCertificatesVerifier::new();
    let client = reqwest::Client::new();

    let result = verifier
        .auto_update_certificates(&config, &client)
        .await
        .expect("自动更新失败");
    assert!(result.refreshed);
    assert_eq!(
        result.serial_numbers,
        vec![WECHAT_PAY_CERT_SERIAL_NO.to_string()]
    );
    assert_eq!(
        captured.lock().expect("captured 锁")[0].path,
        "/api-weixin/v3/certificates"
    );
}

/// 下载失败语义（对应 Java `autoUpdateCert`：非 200 抛
/// `WxRuntimeException(getErrorMsg(body))`、空列表抛 "Cert list is empty"；
/// 失败保留旧证书，仅告警不抛出——Rust 以 Result 表达）。
#[tokio::test]
async fn auto_update_failure_semantics() {
    let client = reqwest::Client::new();

    // 1. 非 200：错误消息取响应体 message 字段（对应 Java getErrorMsg）
    let (addr, _) = spawn_mock_server(|_| {
        (
            500,
            r#"{"code":"SYSTEM_ERROR","message":"系统繁忙"}"#.to_string(),
        )
    });
    let config = test_config(&format!("http://{addr}"));
    let verifier = WxPayAutoUpdateCertificatesVerifier::new();
    let err = verifier
        .auto_update_certificates(&config, &client)
        .await
        .expect_err("应下载失败");
    assert!(err.to_string().contains("系统繁忙"), "{err}");
    assert!(verifier.get_serial_numbers().is_empty());

    // 2. 非 200 且响应体无 message：缺省 "update cert failed"
    let (addr, _) = spawn_mock_server(|_| (503, "oops".to_string()));
    let config = test_config(&format!("http://{addr}"));
    let err = verifier
        .auto_update_certificates(&config, &client)
        .await
        .expect_err("应下载失败");
    assert!(err.to_string().contains("update cert failed"), "{err}");

    // 3. 200 但证书列表为空：对应 Java `throw new WxRuntimeException("Cert list is empty")`
    let (addr, _) = spawn_mock_server(|_| (200, r#"{"data":[]}"#.to_string()));
    let config = test_config(&format!("http://{addr}"));
    let err = verifier
        .auto_update_certificates(&config, &client)
        .await
        .expect_err("应报证书列表为空");
    assert!(err.to_string().contains("Cert list is empty"), "{err}");

    // 4. 200 但响应体非 JSON
    let (addr, _) = spawn_mock_server(|_| (200, "not-json".to_string()));
    let config = test_config(&format!("http://{addr}"));
    let err = verifier
        .auto_update_certificates(&config, &client)
        .await
        .expect_err("应解析失败");
    assert!(err.to_string().contains("证书响应解析失败"), "{err}");

    // 5. 配置缺私钥：对应 Java WxPayCredentials 构造校验
    let mut bad_config = WxPayDefaultConfig::new();
    bad_config
        .set_mch_id(MCH_ID)
        .set_cert_serial_no(MERCHANT_CERT_SERIAL_NO)
        .set_api_v3_key(API_V3_KEY);
    let err = verifier
        .auto_update_certificates(&bad_config, &client)
        .await
        .expect_err("应配置错误");
    assert!(err.to_string().contains("缺少商户 API 私钥"), "{err}");

    // 6. 更新失败不影响旧证书（Java：仅 warn 并继续使用旧证书验签）
    let platform_cert =
        load_certificate_from_pem(WECHAT_PAY_CERT_PEM.as_bytes()).expect("平台证书");
    verifier.set_certificates(vec![platform_cert]);
    let (addr, _) = spawn_mock_server(|_| (500, r#"{"message":"系统繁忙"}"#.to_string()));
    let config = test_config(&format!("http://{addr}"));
    let err = verifier
        .auto_update_certificates(&config, &client)
        .await
        .expect_err("应下载失败");
    assert!(err.to_string().contains("系统繁忙"), "{err}");
    let message = "1700000000\nr0uYIzEaIUX9\n{\"id\":\"EV-3\"}\n";
    let platform_key =
        load_private_key_from_pem(WECHAT_PAY_PRIVATE_KEY_PEM.as_bytes()).expect("平台私钥");
    let signature = sign_sha256_rsa(&platform_key, message.as_bytes()).expect("签名");
    assert!(verifier.verify(WECHAT_PAY_CERT_SERIAL_NO, message.as_bytes(), &signature));
}

/// 篡改 Base64 数据的一个字节（测试用）。
fn tamper_base64(input: &str) -> String {
    use base64::Engine;
    let mut bytes = base64::engine::general_purpose::STANDARD
        .decode(input)
        .expect("base64 解码");
    let len = bytes.len();
    bytes[len / 2] ^= 0x01;
    base64::engine::general_purpose::STANDARD.encode(bytes)
}
