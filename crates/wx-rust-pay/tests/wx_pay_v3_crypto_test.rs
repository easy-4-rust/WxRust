//! wx-rust-pay v3 证书/签名/AES-GCM 单元测试。
//!
//! Golden 来源标注：
//! - `official-wechatpay-java`：微信支付官方 SDK
//!   https://github.com/wechatpay-apiv3/wechatpay-java 测试资源与测试类
//!   （`core/src/test/resources/merchant_private_key.pem` 等密钥材料、
//!   `AeadAesCipherTest`、`WechatPay2CredentialTest`、`TestConfig`）；
//! - 测试证书：商户/平台证书由官方 SDK 私钥重新签发（有效期 2026-2036，
//!   序列号保持官方 `TestConfig` 值不变，避免官方原件 2025-05 过期导致
//!   checkValidity 语义与 golden 冲突）；官方过期原件保留用于
//!   "证书已过期" 行为测试；
//! - `openssl`：RSA-SHA256 签名 golden 由 `openssl dgst -sha256 -sign`
//!   独立生成并验签通过（PKCS#1 v1.5 确定性签名）；
//! - 商户 p12 夹具由 `openssl pkcs12 -export -legacy` 生成（对应微信
//!   apiclient_cert.p12 的 3DES/RC2 传统格式），密码为商户号。

use base64::Engine;
use serde::{Deserialize, Serialize};
use serde_json::json;

use wx_rust_pay::bean::notify::SignatureHeader;
use wx_rust_pay::util::crypto::wx_pay_cert_utils::{
    load_certificate_from_pem, load_private_key_and_cert_from_p12, load_private_key_from_pem,
    load_public_key_from_pem,
};
use wx_rust_pay::util::crypto::wx_pay_v3_crypto_utils::{
    AUTHORIZATION_SCHEMA, aes_gcm_decrypt, aes_gcm_encrypt, build_authorization_token,
    build_request_message, canonical_url_from_url, create_authorization_header, gen_gcm_nonce,
    rsa_oaep_decrypt, rsa_oaep_encrypt, sign_sha256_rsa, verify_response_signature,
    verify_sha256_rsa,
};
use wx_rust_pay::util::wx_pay_notify_utils::{
    NotifyV3Result, build_notify_sign_message, parse_notify_v3_result, parse_signature_header,
    verify_notify_signature,
};

// ---- 夹具：密钥材料来自官方 wechatpay-java SDK 测试资源（测试专用） ----

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

/// 商户 API 证书（由官方 SDK 商户私钥重新签发，有效期 2026-08 ~ 2036-07；
/// 序列号保持官方 `TestConfig.MERCHANT_CERTIFICATE_SERIAL_NUMBER`）。
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

/// 已过期商户 API 证书（official-wechatpay-java `merchant_certificate.pem`
/// 原件，有效期 2022-05-10 ~ 2025-05-09；用于验证"证书已过期"行为，
/// 对应 Java `PemUtils.loadCertificate` 的 `CertificateExpiredException`）。
const EXPIRED_MERCHANT_CERT_PEM: &str = "-----BEGIN CERTIFICATE-----
MIIDrTCCApWgAwIBAgIUXxxy4qiTG3Ki4Tr43ukkces5cRUwDQYJKoZIhvcNAQEL
BQAwZjELMAkGA1UEBhMCQ04xEjAQBgNVBAgMCUd1YW5nZG9uZzERMA8GA1UEBwwI
c2hlbnpoZW4xITAfBgNVBAoMGEludGVybmV0IFdpZGdpdHMgUHR5IEx0ZDENMAsG
A1UEAwwEdGVzdDAeFw0yMjA1MTAwMTU3MTBaFw0yNTA1MDkwMTU3MTBaMGYxCzAJ
BgNVBAYTAkNOMRIwEAYDVQQIDAlHdWFuZ2RvbmcxETAPBgNVBAcMCHNoZW56aGVu
MSEwHwYDVQQKDBhJbnRlcm5ldCBXaWRnaXRzIFB0eSBMdGQxDTALBgNVBAMMBHRl
c3QwggEiMA0GCSqGSIb3DQEBAQUAA4IBDwAwggEKAoIBAQCaZzehUwzcxdLgGn/U
NryHLdX0yBvCqD0p92/BdlCIBi0dmzZzrfc+FF0xK70AP9b2+Ry5q+zXUU+dPucJ
mgwABiZ+Lte+4euMxqPCEkdEu9lyiphalpoaOVPbmDNatzq9k5a/P454QRWMdkLU
JZCeoL9bF2Gn/2+wWEw3sL8zFFcOM8Jr1PdOLmAx+h7pf/87jcmXXCm+SZqw5MtI
LKQi9zHWujYdMA0IcYNeQaNl1h/NUnungdIHKaaU+17wCXqTcZsipAGoqfqrHx/s
r30ZszOdHYOvFNiB+rhEldBGSLWwSYE6LFkbP9GdQWKIQCip3E5dLj5ZFkDeOv4H
ekf9AgMBAAGjUzBRMB0GA1UdDgQWBBQQicr3Vq+2NG2ykH0nxQxsct1RtTAfBgNV
HSMEGDAWgBQQicr3Vq+2NG2ykH0nxQxsct1RtTAPBgNVHRMBAf8EBTADAQH/MA0G
CSqGSIb3DQEBCwUAA4IBAQCH//ZlSAbc6xRlwKwwfbHMsVpAfo/p+Y3397Gi4sR8
RbsW6a3ezXhbiSGiJxDnerx9Cmp4K6U+D3Fl0N9K6CDWfwpGNiZvjmCxtM/wb7dM
u+4U0MXJGBcaePj4mtV/+qPxSWDCQ+PR5OcYRQnac9LfrffTuejQx5aw9FcD7egI
SYfgkZ/wgHyQcZyhh8s6DuigpEaVGMLLoWDZGJ3D+rt4kXk9y5+03KBB2vobyl6P
+PDgaYrlNR/UQMnKMdkgWp7bebmrcXte32/6FAwK3SuHZOSkteDtRCYhYfG5SGSr
go1ujfIK1zIFyPkKKnsgTUPJXwdC7LI+ZqJuWTyVTNj6
-----END CERTIFICATE-----";

/// 微信支付平台私钥（official-wechatpay-java `wechat_pay_private_key.pem`，
/// 测试中用于模拟平台侧对响应/通知签名）。
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

/// 微信支付平台证书（由官方 SDK 平台私钥重新签发，有效期 2026-08 ~ 2036-07；
/// 序列号保持官方 `TestConfig.WECHAT_PAY_CERTIFICATE_SERIAL_NUMBER`）。
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

/// 已过期平台证书（official-wechatpay-java `wechat_pay_certificate.pem`
/// 原件，有效期 2022-05-10 ~ 2025-05-09）。
const EXPIRED_WECHAT_PAY_CERT_PEM: &str = "-----BEGIN CERTIFICATE-----
MIIDozCCAougAwIBAgIURAAkBFxKQnWZ0Ju0494CefLoE/0wDQYJKoZIhvcNAQEL
BQAwYTELMAkGA1UEBhMCQ04xEjAQBgNVBAgMCUd1YW5nZG9uZzERMA8GA1UEBwwI
U2hlbnpoZW4xDTALBgNVBAoMBHRlc3QxDTALBgNVBAsMBHRlc3QxDTALBgNVBAMM
BHRlc3QwHhcNMjIwNTEwMDIwNDMwWhcNMjUwNTA5MDIwNDMwWjBhMQswCQYDVQQG
EwJDTjESMBAGA1UECAwJR3Vhbmdkb25nMREwDwYDVQQHDAhTaGVuemhlbjENMAsG
A1UECgwEdGVzdDENMAsGA1UECwwEdGVzdDENMAsGA1UEAwwEdGVzdDCCASIwDQYJ
KoZIhvcNAQEBBQADggEPADCCAQoCggEBAOKgVbpgGrd6Fj3VlblNuH5E1Se4Z+i7
6epuHQqvnf7cR/QQ2hNs7VLbgGE2/lnroFaA2SqYa0F+bYrJkKv2jf66bcrwV/MQ
gM7JlxbGF1iLIhvXHVOBSpNtu2pKeWLt3e+E11Iw8oo8A2pqUNh2U7vDugnsrrmG
MR3KVtFZJErWiB187anwRlfCvRDOu3G0pgvp6vimPjiKPax2VJAp3C66wXR3+fSJ
PynUbBFNRM81Kj3uXVWA7Loj8VkL0H/PEoCrv/v9eWBYppaRMmvT3ZAnE1dMcvzk
iIrCBR3pOo/SIuYy76oVcGkmvuc5Ia60quP29dJFy44dyD7Ew3p2Le8CAwEAAaNT
MFEwHQYDVR0OBBYEFCcpJ5TvknGPLSSulN436iJLfxaSMB8GA1UdIwQYMBaAFCcp
J5TvknGPLSSulN436iJLfxaSMA8GA1UdEwEB/wQFMAMBAf8wDQYJKoZIhvcNAQEL
BQADggEBAN14iJLYlcIGOFv+0ZXp8pyPvUUMw4j1Re4CBqugXdtLvMVvnca+l+SP
daGRbOi1my/TafSms9yyDMTLnp9wmjZAPWBAMleaAef8a1PRSVq43HaalEzMvrU5
FXGEKaz71/Xk4kWdVNAJNbR9iJp1RLD7Rjdg77BBZKsZzmJlR552UxpGIHdian9h
zHvoFK7lDjdIb3LZf9jaBJ+TjC5ZofKPWiwOmTRXPT9pLLPQ2g6EVxdHVaNOl56W
dmgagF+nsIjBuBm+bjGBUKRy7p1ES8QNdr+fzmbih+D8Ss5BYQ5Z/Beu9yElqC2o
53LlJ4Cc44G6XY/7SR8OJFhOqHmKMuE=
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

/// 商户 p12 证书夹具（`openssl pkcs12 -export -legacy` 由商户私钥+证书生成，
/// 传统 3DES/RC2 格式；密码为商户号 `1234567891`，对应 Java
/// `WxPayConfig.p12ToPem` 以 mchId 作 p12 密码）。
const MERCHANT_P12_BASE64: &str =
    "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
BMfvJQQQ8QQsT1mnFF3iTOfoHbyj2wICCAA=";

// ---- Golden 常量 ----

/// APIv3 密钥（official-wechatpay-java `TestConfig.API_V3_KEY`）。
const API_V3_KEY: &str = "a7cde1ZJB1kG2e7VfTs3jQzaWizur8Gb";

/// 商户号（official-wechatpay-java `TestConfig.MERCHANT_ID`）。
const MCH_ID: &str = "1234567891";

/// 商户证书序列号（official-wechatpay-java `TestConfig.MERCHANT_CERTIFICATE_SERIAL_NUMBER`）。
const MERCHANT_CERT_SERIAL_NO: &str = "5F1C72E2A8931B72A2E13AF8DEE92471EB397115";

/// 平台证书序列号（official-wechatpay-java `TestConfig.WECHAT_PAY_CERTIFICATE_SERIAL_NUMBER`）。
const WECHAT_PAY_CERT_SERIAL_NO: &str = "440024045C4A427599D09BB4E3DE0279F2E813FD";

/// AES-GCM golden（official-wechatpay-java `AeadAesCipherTest`）：
/// key=`API_V3_KEY`，aad=`associatedData`，nonce=`uluk4a9R25RW`，
/// plaintext=`message` → ciphertext(base64)=`ulwSiIajGClcvcOYvOQ7+l+0PAbzzwI=`。
/// （已用 Node.js `crypto` 独立复核通过。）
const GCM_NONCE: &str = "uluk4a9R25RW";
const GCM_AAD: &str = "associatedData";
const GCM_CIPHERTEXT_B64: &str = "ulwSiIajGClcvcOYvOQ7+l+0PAbzzwI=";

/// 请求签名串 golden（official-wechatpay-java `WechatPay2CredentialTest`）：
/// GET + 带 query 的 URL + timestamp=1652750623 + nonce=eBYqCOxa0QudMnPvVilkTzOg3gHh5Z3u + 空 body。
const REQ_METHOD: &str = "GET";
const REQ_URL: &str = "https://api.mch.weixin.qq.com/v3/pay/transactions/id/1217752501201407033233368018?mchid=1230000109";
const REQ_TIMESTAMP: i64 = 1652750623;
const REQ_NONCE: &str = "eBYqCOxa0QudMnPvVilkTzOg3gHh5Z3u";
const REQ_EXPECTED_MESSAGE: &str = "GET\n/v3/pay/transactions/id/1217752501201407033233368018?mchid=1230000109\n1652750623\neBYqCOxa0QudMnPvVilkTzOg3gHh5Z3u\n\n";

/// RSA-SHA256 签名 golden（openssl 独立生成）：
/// `openssl dgst -sha256 -sign merchant_private_key.pem` 对 `REQ_EXPECTED_MESSAGE`
/// 的签名（PKCS#1 v1.5 签名确定，可做精确相等断言）。
const REQ_MESSAGE_OPENSSL_SIGNATURE_B64: &str = "I3rBjiGKeDrfOqpT4aW2VvjhsA2GEpOyCQivyRwmSroe6EftplTwTBtIyg0QKmxX+5xWmgOkHEWjebST5Vt8NL9Dka8cNN3oZsRvG8hvBxuTLMrvigG7tOF2YNUW6AKzKlCCXMAo0MF6BE/d7jNKyM7nAK+ZaiI/22T4v2RmlQFFGo1op6Li+MBeLb7624rDqkw5VKY97jMavtEFgaVWy0DKTmiXinujsU9kpYVJq8noO0NiyhDWG5RFYQmGYRzd3wio9IWRDWYi+qNWuGhg1CoSOgo02UbmT9W8/RXXxP+w0c9RLLhL3aBPT5coCp6cEbyA+KukOGAhKi93HHIYAg==";

// ---- 测试 ----

/// AES-256-GCM 解密 golden（official-wechatpay-java `AeadAesCipherTest`）。
#[test]
fn aes_gcm_decrypt_official_sdk_vector() {
    let plaintext =
        aes_gcm_decrypt(API_V3_KEY, GCM_AAD, GCM_NONCE, GCM_CIPHERTEXT_B64).expect("解密失败");
    assert_eq!(plaintext, "message");
}

/// AES-256-GCM 加密 golden（official-wechatpay-java `AeadAesCipherTest`）。
#[test]
fn aes_gcm_encrypt_official_sdk_vector() {
    let ciphertext =
        aes_gcm_encrypt(API_V3_KEY, GCM_AAD, GCM_NONCE.as_bytes(), "message").expect("加密失败");
    assert_eq!(ciphertext, GCM_CIPHERTEXT_B64);
}

/// AES-GCM 认证失败：篡改密文/换 AAD 必须报错（对应 Java JCE
/// AEADBadTagException，AesUtils 解密失败）。
#[test]
fn aes_gcm_decrypt_tampered_rejected() {
    assert!(aes_gcm_decrypt(API_V3_KEY, "bad-aad", GCM_NONCE, GCM_CIPHERTEXT_B64).is_err());
    // 篡改密文一个字节
    let mut bytes = base64::engine::general_purpose::STANDARD
        .decode(GCM_CIPHERTEXT_B64)
        .expect("base64");
    bytes[0] ^= 0x01;
    let tampered = base64::engine::general_purpose::STANDARD.encode(bytes);
    assert!(aes_gcm_decrypt(API_V3_KEY, GCM_AAD, GCM_NONCE, &tampered).is_err());
    // apiV3Key 长度错误
    assert!(aes_gcm_decrypt("short-key", GCM_AAD, GCM_NONCE, GCM_CIPHERTEXT_B64).is_err());
}

/// 请求签名串构造 golden（official-wechatpay-java `WechatPay2CredentialTest`）：
/// `METHOD\ncanonicalUrl\nTIMESTAMP\nNONCE\nBODY\n`，canonicalUrl 含 query。
#[test]
fn build_request_message_official_sdk_vector() {
    let canonical_url = canonical_url_from_url(REQ_URL).expect("URL 解析失败");
    assert_eq!(
        canonical_url,
        "/v3/pay/transactions/id/1217752501201407033233368018?mchid=1230000109"
    );
    let message = build_request_message(REQ_METHOD, &canonical_url, REQ_TIMESTAMP, REQ_NONCE, "");
    assert_eq!(message, REQ_EXPECTED_MESSAGE);
}

/// Authorization 头格式（对应 Java `WxPayCredentials.getToken` 与
/// `getSchema`：`WECHATPAY2-SHA256-RSA2048 mchid="..",nonce_str="..",...`）。
#[test]
fn authorization_token_and_header_format() {
    let token = build_authorization_token(
        MCH_ID,
        REQ_NONCE,
        REQ_TIMESTAMP,
        MERCHANT_CERT_SERIAL_NO,
        "test-signature",
    );
    assert_eq!(
        token,
        "mchid=\"1234567891\",nonce_str=\"eBYqCOxa0QudMnPvVilkTzOg3gHh5Z3u\",timestamp=\"1652750623\",serial_no=\"5F1C72E2A8931B72A2E13AF8DEE92471EB397115\",signature=\"test-signature\""
    );

    // 一键构造完整头：签名由商户私钥生成，头前导为 schema
    let key = load_private_key_from_pem(MERCHANT_PRIVATE_KEY_PEM.as_bytes()).expect("私钥");
    let header = create_authorization_header(
        MCH_ID,
        MERCHANT_CERT_SERIAL_NO,
        &key,
        REQ_METHOD,
        "/v3/pay/transactions/id/1217752501201407033233368018?mchid=1230000109",
        REQ_TIMESTAMP,
        REQ_NONCE,
        "",
    )
    .expect("构造失败");
    assert!(header.starts_with(&format!("{AUTHORIZATION_SCHEMA} ")));
    // 签名与 openssl golden 一致（PKCS#1 v1.5 确定性签名）
    let expected_sig = &format!(
        "mchid=\"1234567891\",nonce_str=\"eBYqCOxa0QudMnPvVilkTzOg3gHh5Z3u\",timestamp=\"1652750623\",serial_no=\"5F1C72E2A8931B72A2E13AF8DEE92471EB397115\",signature=\"{REQ_MESSAGE_OPENSSL_SIGNATURE_B64}\""
    );
    assert!(header.ends_with(expected_sig));
}

/// RSA-SHA256 签名 golden（openssl 独立生成，PKCS#1 v1.5 确定性签名做精确相等断言）。
#[test]
fn rsa_sign_matches_openssl_golden() {
    let key = load_private_key_from_pem(MERCHANT_PRIVATE_KEY_PEM.as_bytes()).expect("私钥");
    let signature = sign_sha256_rsa(&key, REQ_EXPECTED_MESSAGE.as_bytes()).expect("签名失败");
    assert_eq!(signature, REQ_MESSAGE_OPENSSL_SIGNATURE_B64);

    // 验签通过
    let cert = load_certificate_from_pem(MERCHANT_CERT_PEM.as_bytes()).expect("证书");
    let public_key = cert.public_key().expect("公钥");
    assert!(
        verify_sha256_rsa(&public_key, REQ_EXPECTED_MESSAGE.as_bytes(), &signature).expect("验签")
    );
}

/// RSA 验签拒绝篡改消息/错误签名。
#[test]
fn rsa_verify_rejects_tampered() {
    let key = load_private_key_from_pem(MERCHANT_PRIVATE_KEY_PEM.as_bytes()).expect("私钥");
    let cert = load_certificate_from_pem(MERCHANT_CERT_PEM.as_bytes()).expect("证书");
    let public_key = cert.public_key().expect("公钥");
    let signature = sign_sha256_rsa(&key, REQ_EXPECTED_MESSAGE.as_bytes()).expect("签名");

    let tampered_message = format!("{REQ_EXPECTED_MESSAGE}tampered");
    assert!(
        !verify_sha256_rsa(&public_key, tampered_message.as_bytes(), &signature).expect("验签")
    );
    assert!(
        !verify_sha256_rsa(&public_key, REQ_EXPECTED_MESSAGE.as_bytes(), "AAAA").expect("验签")
    );
}

/// RUSTSEC-2023-0071（rsa 0.9.x Marvin）缓解证据测试。
///
/// 缓解策略（与 `deny.toml` 例外注释、`docs/known-issues.md` 一致）：
/// - WxRust 私钥解密路径统一走 **RSA-OAEP**（OAEP 内建完整性校验，对
///   时序侧信道攻击的暴露面远小于 PKCS#1 v1.5 无填充校验的裸 RSA 解密）；
/// - 全库不存在对任意密文执行 PKCS#1 v1.5 解密的使用点（v1.5 仅用于
///   确定性 RSA-SHA256 签名，不受 Marvin 影响）；
/// - 彻底修复等待 rsa 0.10 stable（Phase D 跟踪项）。
///
/// 本测试证明 OAEP 加解密闭环可用且具备随机化与完整性：
/// 1. 同一明文两次加密密文不同（随机填充）；
/// 2. 解密恢复原明文；
/// 3. 篡改密文必须解密失败（OAEP 完整性校验）。
#[test]
fn rsa_oaep_roundtrip_marvin_mitigation_evidence() {
    let key = load_private_key_from_pem(MERCHANT_PRIVATE_KEY_PEM.as_bytes()).expect("私钥");
    let public_key = key.to_public_key();
    let msg = "WxRust-OAEP-marvin-mitigation-evidence";

    let cipher_b64 = rsa_oaep_encrypt(&public_key, msg).expect("OAEP 加密");
    let cipher_b64_2 = rsa_oaep_encrypt(&public_key, msg).expect("OAEP 加密（重复）");
    assert_ne!(cipher_b64, cipher_b64_2, "OAEP 应具备随机填充");

    let plain = rsa_oaep_decrypt(&key, &cipher_b64).expect("OAEP 解密");
    assert_eq!(plain, msg, "解密应恢复原明文");

    let mut tampered =
        base64::engine::general_purpose::STANDARD.decode(&cipher_b64).expect("base64 解码");
    if let Some(b) = tampered.last_mut() {
        *b ^= 0x01;
    }
    let tampered_b64 = base64::engine::general_purpose::STANDARD.encode(&tampered);
    assert!(
        rsa_oaep_decrypt(&key, &tampered_b64).is_err(),
        "篡改密文必须解密失败（OAEP 完整性校验）"
    );
}

/// 证书序列号提取 golden（official-wechatpay-java `TestConfig` 两个序列号）。
#[test]
fn cert_serial_number_official_sdk_certs() {
    let merchant_cert = load_certificate_from_pem(MERCHANT_CERT_PEM.as_bytes()).expect("商户证书");
    assert_eq!(merchant_cert.serial_no(), MERCHANT_CERT_SERIAL_NO);

    let wechat_cert = load_certificate_from_pem(WECHAT_PAY_CERT_PEM.as_bytes()).expect("平台证书");
    assert_eq!(wechat_cert.serial_no(), WECHAT_PAY_CERT_SERIAL_NO);
}

/// 已过期证书必须被拒绝（对应 Java `PemUtils.loadCertificate` 的
/// `checkValidity()` → `CertificateExpiredException("证书已过期")`）。
#[test]
fn expired_cert_rejected_mirrors_java() {
    let err = load_certificate_from_pem(EXPIRED_MERCHANT_CERT_PEM.as_bytes())
        .expect_err("应拒绝过期证书");
    assert!(err.to_string().contains("证书已过期"));

    let err = load_certificate_from_pem(EXPIRED_WECHAT_PAY_CERT_PEM.as_bytes())
        .expect_err("应拒绝过期证书");
    assert!(err.to_string().contains("证书已过期"));
}

/// PEM 私钥/证书加载 + 证书公钥验签 openssl golden。
#[test]
fn load_private_key_and_cert_from_pem() {
    let key = load_private_key_from_pem(MERCHANT_PRIVATE_KEY_PEM.as_bytes()).expect("私钥");
    let cert = load_certificate_from_pem(MERCHANT_CERT_PEM.as_bytes()).expect("证书");

    // 证书公钥与私钥匹配：私钥签名与 openssl golden 一致，且能被证书公钥验签
    let sig = sign_sha256_rsa(&key, REQ_EXPECTED_MESSAGE.as_bytes()).expect("签名");
    assert_eq!(sig, REQ_MESSAGE_OPENSSL_SIGNATURE_B64);
    let public_key = cert.public_key().expect("公钥");
    assert!(verify_sha256_rsa(&public_key, REQ_EXPECTED_MESSAGE.as_bytes(), &sig).expect("验签"));

    // 平台公钥 PEM（微信公钥模式，对应 PemUtils.loadPublicKey）
    let pub_key = load_public_key_from_pem(WECHAT_PAY_PUBLIC_KEY_PEM.as_bytes()).expect("公钥");
    let wechat_cert = load_certificate_from_pem(WECHAT_PAY_CERT_PEM.as_bytes()).expect("平台证书");
    assert_eq!(pub_key, wechat_cert.public_key().expect("公钥"));
}

/// p12 夹具的 base64（源码中按 76 字符换行，解码前需去掉换行）。
fn p12_der() -> Vec<u8> {
    base64::engine::general_purpose::STANDARD
        .decode(MERCHANT_P12_BASE64.replace('\n', ""))
        .expect("p12 base64")
}

/// p12 容器加载（对应 Java `WxPayConfig.p12ToPem`：密码=商户号，取第一个
/// 私钥与证书，序列号与 PEM 通道一致）。
#[test]
fn load_p12_matches_pem_channel() {
    let data = load_private_key_and_cert_from_p12(&p12_der(), MCH_ID).expect("p12 解析失败");
    assert_eq!(data.certificate.serial_no(), MERCHANT_CERT_SERIAL_NO);

    // p12 私钥与 PEM 私钥等价：同一消息签名一致（确定性 PKCS#1 v1.5）
    let pem_key = load_private_key_from_pem(MERCHANT_PRIVATE_KEY_PEM.as_bytes()).expect("PEM 私钥");
    let sig_p12 =
        sign_sha256_rsa(&data.private_key, REQ_EXPECTED_MESSAGE.as_bytes()).expect("签名");
    let sig_pem = sign_sha256_rsa(&pem_key, REQ_EXPECTED_MESSAGE.as_bytes()).expect("签名");
    assert_eq!(sig_p12, sig_pem);

    // p12 证书公钥也能验签
    let public_key = data.certificate.public_key().expect("公钥");
    assert!(
        verify_sha256_rsa(&public_key, REQ_EXPECTED_MESSAGE.as_bytes(), &sig_p12).expect("验签")
    );
}

/// p12 密码错误必须报错（对应 Java KeyStore.load 密码校验失败）。
#[test]
fn load_p12_wrong_password_rejected() {
    assert!(load_private_key_and_cert_from_p12(&p12_der(), "wrong-password").is_err());
}

/// 响应验签（对应 Java `WxPayValidator`：`timestamp\nnonce\nbody\n` +
/// SHA256withRSA）：平台私钥模拟签名，平台证书公钥验签。
#[test]
fn verify_response_signature_flow() {
    let body = r#"{"code":"SUCCESS","message":"成功"}"#;
    let timestamp = "1700000000";
    let nonce = "r0uYIzEaIUX9";

    let platform_key =
        load_private_key_from_pem(WECHAT_PAY_PRIVATE_KEY_PEM.as_bytes()).expect("平台私钥");
    let platform_cert =
        load_certificate_from_pem(WECHAT_PAY_CERT_PEM.as_bytes()).expect("平台证书");

    // 平台侧签名（对应微信服务器行为）
    let message = format!("{timestamp}\n{nonce}\n{body}\n");
    let signature = sign_sha256_rsa(&platform_key, message.as_bytes()).expect("平台签名");

    // 商户侧验签
    let public_key = platform_cert.public_key().expect("公钥");
    assert!(
        verify_response_signature(&public_key, timestamp, nonce, body, &signature).expect("验签")
    );
    assert!(
        !verify_response_signature(&public_key, timestamp, nonce, "tampered", &signature)
            .expect("验签")
    );
}

/// RSA-OAEP 敏感信息加解密往返（对应 Java `RsaCryptoUtil.encryptOAEP`/
/// `decryptOAEP`：RSA/ECB/OAEPWithSHA-1AndMGF1Padding）。
#[test]
fn rsa_oaep_roundtrip() {
    let private_key = load_private_key_from_pem(MERCHANT_PRIVATE_KEY_PEM.as_bytes()).expect("私钥");
    let cert = load_certificate_from_pem(MERCHANT_CERT_PEM.as_bytes()).expect("证书");
    let public_key = cert.public_key().expect("公钥");

    let ciphertext = rsa_oaep_encrypt(&public_key, "张三").expect("OAEP 加密");
    let plaintext = rsa_oaep_decrypt(&private_key, &ciphertext).expect("OAEP 解密");
    assert_eq!(plaintext, "张三");
}

/// 通知验签与完整解析流程（对应 Java `verifyNotifySign` +
/// `baseParseOrderNotifyV3Result`）。
#[test]
fn notify_verify_and_parse_flow() {
    // 1. 构造解密后明文（对应微信回调加密前的原始 JSON）
    let decrypted_payload = json!({
        "appid": "wxd678efh567hg6787",
        "mchid": "1230000109",
        "out_trade_no": "1217752501201407033233368018",
        "transaction_id": "1217752501201407033233368018",
        "trade_type": "JSAPI",
        "trade_state": "SUCCESS",
        "trade_state_desc": "支付成功",
        "bank_type": "CMC",
        "attach": "自定义数据",
        "success_time": "2018-06-08T10:34:56+08:00",
        "payer": { "openid": "oUpF8uMuAJO_M2pxb1Q9zNjWeS6o" },
        "amount": {
            "total": 100,
            "payer_total": 100,
            "currency": "CNY",
            "payer_currency": "CNY"
        }
    });

    // 2. 平台侧用 APIv3 密钥 AES-GCM 加密 resource（对应微信服务器行为；
    //    AEAD_AES_256_GCM nonce 固定 12 字节）
    let nonce = "fdasflkja4qw";
    let ciphertext = aes_gcm_encrypt(
        API_V3_KEY,
        "transaction",
        nonce.as_bytes(),
        &decrypted_payload.to_string(),
    )
    .expect("AES 加密");

    let notify_json = json!({
        "id": "EV-2018022511223320873",
        "create_time": "2015-05-20T13:29:35+08:00",
        "event_type": "TRANSACTION.SUCCESS",
        "resource_type": "encrypt-resource",
        "summary": "支付成功",
        "resource": {
            "original_type": "transaction",
            "algorithm": "AEAD_AES_256_GCM",
            "ciphertext": ciphertext,
            "associated_data": "transaction",
            "nonce": nonce
        }
    });
    let notify_data = notify_json.to_string();

    // 3. 平台侧对通知体签名（对应微信服务器的 Wechatpay-Signature 头）
    let timestamp = "1700000000";
    let header_nonce = "r0uYIzEaIUX9";
    let platform_key =
        load_private_key_from_pem(WECHAT_PAY_PRIVATE_KEY_PEM.as_bytes()).expect("平台私钥");
    let sign_message = format!("{timestamp}\n{header_nonce}\n{notify_data}\n");
    let signature = sign_sha256_rsa(&platform_key, sign_message.as_bytes()).expect("平台签名");
    let header = SignatureHeader::new(
        Some(timestamp.to_string()),
        Some(header_nonce.to_string()),
        Some(signature),
        Some(WECHAT_PAY_CERT_SERIAL_NO.to_string()),
    );

    // 4. 商户侧：验签（verifyNotifySign 语义）
    let platform_cert =
        load_certificate_from_pem(WECHAT_PAY_CERT_PEM.as_bytes()).expect("平台证书");
    let public_key = platform_cert.public_key().expect("公钥");
    assert!(verify_notify_signature(&public_key, &header, &notify_data).expect("验签"));

    // 5. 完整解析入口（baseParseOrderNotifyV3Result 语义：验签→解析→解密→反序列化）
    let verifier = |_serial: &str, message: &[u8], sig: &str| {
        verify_sha256_rsa(&public_key, message, sig).unwrap_or(false)
    };
    let result: NotifyV3Result<DecryptedNotifyPayload> =
        parse_notify_v3_result(&notify_data, Some(&header), API_V3_KEY, verifier)
            .expect("解析失败");

    assert_eq!(
        result.raw_data.id.as_deref(),
        Some("EV-2018022511223320873")
    );
    assert_eq!(
        result.raw_data.event_type.as_deref(),
        Some("TRANSACTION.SUCCESS")
    );
    assert_eq!(result.result.mchid, "1230000109");
    assert_eq!(result.result.out_trade_no, "1217752501201407033233368018");
    assert_eq!(result.result.trade_state, "SUCCESS");
    assert_eq!(result.result.amount.total, 100);
    assert_eq!(result.result.payer.openid, "oUpF8uMuAJO_M2pxb1Q9zNjWeS6o");

    // 6. 篡改 body 后验签失败 → 解析报错（对应 "非法请求，头部信息验证失败"）
    let tampered_data = notify_data.replace("TRANSACTION.SUCCESS", "TRANSACTION.FAIL");
    let err = parse_notify_v3_result::<DecryptedNotifyPayload>(
        &tampered_data,
        Some(&header),
        API_V3_KEY,
        verifier,
    )
    .expect_err("应验签失败");
    assert!(err.to_string().contains("非法请求"));

    // 7. header 为 None 时跳过验签（对应 Java Objects.nonNull(header) 语义）
    let no_header: NotifyV3Result<DecryptedNotifyPayload> =
        parse_notify_v3_result(&notify_data, None, API_V3_KEY, verifier).expect("解析失败");
    assert_eq!(no_header.result.mchid, "1230000109");
}

/// 通知头解析与验签串构造（对应 Java `SignatureHeader` 构造与
/// `verifyNotifySign` 的 beforeSign）。
#[test]
fn notify_header_parse_and_sign_message() {
    let headers = [
        ("Wechatpay-Timestamp", "1700000000"),
        ("Wechatpay-Nonce", "r0uYIzEaIUX9"),
        ("Wechatpay-Signature", "abc123"),
        (
            "Wechatpay-Serial",
            "440024045C4A427599D09BB4E3DE0279F2E813FD",
        ),
        ("Content-Type", "application/json"),
    ];
    let header = parse_signature_header(&headers);
    assert_eq!(header.time_stamp.as_deref(), Some("1700000000"));
    assert_eq!(header.nonce.as_deref(), Some("r0uYIzEaIUX9"));
    assert_eq!(header.signature.as_deref(), Some("abc123"));
    assert_eq!(
        header.serial.as_deref(),
        Some("440024045C4A427599D09BB4E3DE0279F2E813FD")
    );

    let data = r#"{"id":"EV-1"}"#;
    assert_eq!(
        build_notify_sign_message(&header, data),
        "1700000000\nr0uYIzEaIUX9\n{\"id\":\"EV-1\"}\n"
    );
}

/// gen_gcm_nonce 长度与随机性。
#[test]
fn gen_gcm_nonce_shape() {
    let a = gen_gcm_nonce();
    let b = gen_gcm_nonce();
    assert_eq!(a.len(), 12);
    assert_ne!(a, b);
}

/// 通知解析结果反序列化目标类型（对应 Java `WxPayNotifyV3Result.DecryptNotifyResult`
/// 的子集）。
#[derive(Debug, Clone, Deserialize, Serialize)]
struct DecryptedNotifyPayload {
    appid: String,
    mchid: String,
    out_trade_no: String,
    transaction_id: String,
    trade_state: String,
    payer: Payer,
    amount: Amount,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Payer {
    openid: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Amount {
    total: u32,
}
