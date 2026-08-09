//! 隐私 key 枚举。
//!
//! 对应 Java `me.chanjar.weixin.open.bean.ma.privacy.PrivacyKeyEnum`
//! （`@Getter @AllArgsConstructor` 枚举：`key` 为隐私接口 key（如
//! `UserInfo`），`desc` 为中文描述）。
//!
//! serde 对照：Java Gson 序列化枚举默认输出常量名（`USER_INFO` 等，
//! 无 `@SerializedName`），Rust 以 `#[serde(rename = "USER_INFO")]`
//! 镜像同一线格式；`key()`/`desc()` 为固有方法（Java `@Getter`）。
//! 注意：已生成的隐私 bean（`SetPrivacySetting.Setting` 等）的
//! `privacy_key` 字段为 `String`（Java 侧同为 String 字段），本枚举
//! 供调用方取值填充。

/// 隐私 key 枚举（对应 Java `PrivacyKeyEnum`）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum PrivacyKeyEnum {
    /// 用户信息（微信昵称、头像）
    #[serde(rename = "USER_INFO")]
    UserInfo,
    /// 位置信息
    #[serde(rename = "LOCATION")]
    Location,
    /// 地址
    #[serde(rename = "ADDRESS")]
    Address,
    /// 发票信息
    #[serde(rename = "INVOICE")]
    Invoice,
    /// 微信运动数据
    #[serde(rename = "RUN_DATA")]
    RunData,
    /// 麦克风
    #[serde(rename = "RECORD")]
    Record,
    /// 选中的照片或视频信息
    #[serde(rename = "ALBUM")]
    Album,
    /// 摄像头
    #[serde(rename = "CAMERA")]
    Camera,
    /// 手机号码
    #[serde(rename = "PHONE_NUMBER")]
    PhoneNumber,
    /// 通讯录（仅写入）权限
    #[serde(rename = "CONTACT")]
    Contact,
    /// 设备信息
    #[serde(rename = "DEVICE_INFO")]
    DeviceInfo,
    /// 身份证号码
    #[serde(rename = "EXID_NUMBER")]
    ExidNumber,
    /// 订单信息
    #[serde(rename = "EX_ORDER_INFO")]
    ExOrderInfo,
    /// 发布内容
    #[serde(rename = "EX_USER_PUBLISH_CONTENT")]
    ExUserPublishContent,
    /// 所关注账号
    #[serde(rename = "EX_USER_FOLLOW_ACCT")]
    ExUserFollowAcct,
    /// 操作日志
    #[serde(rename = "EX_USER_OP_LOG")]
    ExUserOpLog,
    /// 相册（仅写入）权限
    #[serde(rename = "ALBUM_WRITE_ONLY")]
    AlbumWriteOnly,
    /// 车牌号
    #[serde(rename = "LICENSE_PLATE")]
    LicensePlate,
    /// 蓝牙
    #[serde(rename = "BLUE_TOOTH")]
    BlueTooth,
    /// 日历（仅写入）权限
    #[serde(rename = "CALENDAR_WRITE_ONLY")]
    CalendarWriteOnly,
    /// 邮箱
    #[serde(rename = "EMAIL")]
    Email,
    /// 选中的文件
    #[serde(rename = "MESSAGE_FILE")]
    MessageFile,
}

impl PrivacyKeyEnum {
    /// 隐私接口 key（对应 Java `getKey()`，如 `UserInfo`）。
    pub fn key(self) -> &'static str {
        match self {
            Self::UserInfo => "UserInfo",
            Self::Location => "Location",
            Self::Address => "Address",
            Self::Invoice => "Invoice",
            Self::RunData => "RunData",
            Self::Record => "Record",
            Self::Album => "Album",
            Self::Camera => "Camera",
            Self::PhoneNumber => "PhoneNumber",
            Self::Contact => "Contact",
            Self::DeviceInfo => "DeviceInfo",
            Self::ExidNumber => "EXIDNumber",
            Self::ExOrderInfo => "EXOrderInfo",
            Self::ExUserPublishContent => "EXUserPublishContent",
            Self::ExUserFollowAcct => "EXUserFollowAcct",
            Self::ExUserOpLog => "EXUserOpLog",
            Self::AlbumWriteOnly => "AlbumWriteOnly",
            Self::LicensePlate => "LicensePlate",
            Self::BlueTooth => "BlueTooth",
            Self::CalendarWriteOnly => "CalendarWriteOnly",
            Self::Email => "Email",
            Self::MessageFile => "MessageFile",
        }
    }

    /// 中文描述（对应 Java `getDesc()`）。
    pub fn desc(self) -> &'static str {
        match self {
            Self::UserInfo => "用户信息（微信昵称、头像）",
            Self::Location => "位置信息",
            Self::Address => "地址",
            Self::Invoice => "发票信息",
            Self::RunData => "微信运动数据",
            Self::Record => "麦克风",
            Self::Album => "选中的照片或视频信息",
            Self::Camera => "摄像头",
            Self::PhoneNumber => "手机号码",
            Self::Contact => "通讯录（仅写入）权限",
            Self::DeviceInfo => "设备信息",
            Self::ExidNumber => "身份证号码",
            Self::ExOrderInfo => "订单信息",
            Self::ExUserPublishContent => "发布内容",
            Self::ExUserFollowAcct => "所关注账号",
            Self::ExUserOpLog => "操作日志",
            Self::AlbumWriteOnly => "相册（仅写入）权限",
            Self::LicensePlate => "车牌号",
            Self::BlueTooth => "蓝牙",
            Self::CalendarWriteOnly => "日历（仅写入）权限",
            Self::Email => "邮箱",
            Self::MessageFile => "选中的文件",
        }
    }
}
