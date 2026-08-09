//! 视频号小店 枚举（对应 Java `SendTime`）。

/// SendTime（对应 Java `me.chanjar.weixin.channel.enums.SendTime`）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SendTime {
    /// 4小时内发货 / // FOUR_HOUR("SendTime_FOUR_HOUR", "4小时内发货"), // / 8小时内发货 / // EIGHT_HOUR("SendTime_EIGHT_HOUR", "8小时内发货"), // / 12小时内发货 / // TWELVE_HOUR("SendTime_TWELVE_HOUR", "12小时内发货"), // / 16小时内发货 / // SIXTEEN_HOUR("SendTime_SIXTEEN_HOUR", "16小时内发货"), // / 20小时内发货 / // TWENTY_HOUR("SendTime_TWENTY_HOUR", "20小时内发货"), / 24小时内发货
    TwentyfourHour,
    /// 48小时内发货
    FoutyeightHour,
    ThreeDay,
    FiveDay,
    SevenDay,
    TenDay,
    TwelveDay,
    FouteenDay,
    SixteenDay,
    TwentyDay,
    TwentyfiveDay,
    ThiryDay,
    ThiryfiveDay,
    FourtyfiveDay,
}

impl SendTime {
    /// 枚举 key（对应 Java `getKey()`）。
    pub fn key(&self) -> &'static str {
        match self {
            SendTime::TwentyfourHour => "SendTime_TWENTYFOUR_HOUR",
            SendTime::FoutyeightHour => "SendTime_FOUTYEIGHT_HOUR",
            SendTime::ThreeDay => "SendTime_THREE_DAY",
            SendTime::FiveDay => "SendTime_FIVE_DAY",
            SendTime::SevenDay => "SendTime_SEVEN_DAY",
            SendTime::TenDay => "SendTime_TEN_DAY",
            SendTime::TwelveDay => "SendTime_TWELVE_DAY",
            SendTime::FouteenDay => "SendTime_FOUTEEN_DAY",
            SendTime::SixteenDay => "SendTime_SIXTEEN_DAY",
            SendTime::TwentyDay => "SendTime_TWENTY_DAY",
            SendTime::TwentyfiveDay => "SendTime_TWENTYFIVE_DAY",
            SendTime::ThiryDay => "SendTime_THIRY_DAY",
            SendTime::ThiryfiveDay => "SendTime_THIRYFIVE_DAY",
            SendTime::FourtyfiveDay => "SendTime_FOURTYFIVE_DAY",
        }
    }

    /// 枚举中文说明（对应 Java `getValue()`）。
    pub fn value(&self) -> &'static str {
        match self {
            SendTime::TwentyfourHour => "24小时内发货",
            SendTime::FoutyeightHour => "48小时内发货",
            SendTime::ThreeDay => "3天内发货",
            SendTime::FiveDay => "5天内发货",
            SendTime::SevenDay => "7天内发货",
            SendTime::TenDay => "10天内发货",
            SendTime::TwelveDay => "12天内发货",
            SendTime::FouteenDay => "14天内发货",
            SendTime::SixteenDay => "16天内发货",
            SendTime::TwentyDay => "20天内发货",
            SendTime::TwentyfiveDay => "25天内发货",
            SendTime::ThiryDay => "30天内发货",
            SendTime::ThiryfiveDay => "35天内发货",
            SendTime::FourtyfiveDay => "45天内发货",
        }
    }
}
