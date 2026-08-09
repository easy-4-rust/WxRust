//! 数据处理工具类。
//!
//! 对应 Java `me.chanjar.weixin.common.util.DataUtils`。

/// 数据处理工具。
pub struct DataUtils;

impl DataUtils {
    /// 将数据中包含的 secret 字符使用星号替换，防止日志打印时被输出。
    ///
    /// 对应 Java `handleDataWithSecret`：对字符串中 `&secret=xxxx&` 的
    /// 敏感参数做脱敏（替换为 `&secret=******&`）。
    ///
    /// # 参数
    /// - `data`：需要脱敏的数据（字符串）
    ///
    /// # 返回
    /// 脱敏后的字符串。
    pub fn handle_data_with_secret(data: &str) -> String {
        // 使用正则替换 &secret=word& 为 &secret=******&
        regex_lite(data)
    }
}

/// 轻量正则替换（避免引入 regex 依赖；语义与 Java `&secret=\\w+&` 一致）。
fn regex_lite(data: &str) -> String {
    // 逐段查找 "&secret=" 与下一个 "&" 之间的内容并替换
    let marker = "&secret=";
    let mut result = String::with_capacity(data.len());
    let mut rest = data;
    loop {
        match rest.find(marker) {
            Some(pos) => {
                result.push_str(&rest[..pos + marker.len()]);
                let after = &rest[pos + marker.len()..];
                match after.find('&') {
                    Some(end) => {
                        result.push_str("******");
                        result.push_str(&after[end..]);
                        rest = &after[end..];
                    }
                    None => {
                        // 无结尾 &：替换到末尾
                        result.push_str("******");
                        return result;
                    }
                }
            }
            None => {
                result.push_str(rest);
                return result;
            }
        }
    }
}
