#[cfg(not(feature = "enable-sdk"))]
use windows::core::{PCSTR, s};

#[cfg(not(feature = "enable-sdk"))]
pub(crate) struct DisableSdkConfiguration {
    pub(crate) sdk_dll: PCSTR,
    pub(crate) eula_accept: usize,
    pub(crate) sdk_go_away: usize,
}

pub(crate) struct InjectConfiguration {
    pub(crate) f_pak_file_check: usize,
    pub(crate) f_pak_file_check_preamble: u128,
    pub(crate) kuro_http_get: usize,
    #[cfg(not(feature = "enable-sdk"))]
    pub(crate) disable_sdk: DisableSdkConfiguration,
}

#[cfg(feature = "cn_beta_1_3_0")]
pub(crate) const CONFIG: InjectConfiguration = InjectConfiguration {
    f_pak_file_check: 0x3D2F460,
    f_pak_file_check_preamble: 0x943D80000000A8EC8148574157565340,
    kuro_http_get: 0xFC8CF0,
    #[cfg(not(feature = "enable-sdk"))]
    disable_sdk: DisableSdkConfiguration{
        sdk_dll: s!("KRSDKEx.dll"),
        eula_accept: 0x4A690,
        sdk_go_away: 0x8BB80,
    }
};

#[cfg(feature = "cn_live_1_3_0")]
pub(crate) const CONFIG: InjectConfiguration = InjectConfiguration {
    f_pak_file_check: 0x3D35DF0,
    f_pak_file_check_preamble: 0x943D80000000A8EC8148574157565340,
    kuro_http_get: 0xFC9900,
    #[cfg(not(feature = "enable-sdk"))]
    disable_sdk: DisableSdkConfiguration{
        sdk_dll: s!("KRSDKEx.dll"),
        eula_accept: 0x4A690,
        sdk_go_away: 0x8B9F0,
    }
};

#[cfg(feature = "os_live_1_3_0")]
pub(crate) const CONFIG: InjectConfiguration = InjectConfiguration {
    f_pak_file_check: 0x3CDC430,
    f_pak_file_check_preamble: 0x943D80000000A8EC8148574157565340,
    kuro_http_get: 0xFC6C20,
    #[cfg(not(feature = "enable-sdk"))]
    disable_sdk: DisableSdkConfiguration{
        sdk_dll: s!("KRSDK.dll"),
        eula_accept: 0x94710,
        sdk_go_away: 0x9FE10
    }
};