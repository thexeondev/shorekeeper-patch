#[cfg(all(not(feature = "enable-sdk"), not(feature = "only-sig-bypass"), feature = "regular"))]
use windows::core::{PCSTR, s};

#[cfg(all(not(feature = "enable-sdk"), not(feature = "only-sig-bypass"), feature = "regular"))]
pub(crate) struct DisableSdkConfiguration {
    pub(crate) sdk_dll: PCSTR,
    pub(crate) eula_accept: usize,
    pub(crate) sdk_go_away: usize,
}

pub(crate) struct InjectConfiguration {
    pub(crate) f_pak_file_check: usize,
    pub(crate) f_pak_file_check_preamble: u64,
    #[cfg(all(not(feature = "only-sig-bypass"), feature = "regular"))]
    pub(crate) kuro_http_get: usize,
    #[cfg(all(not(feature = "enable-sdk"), not(feature = "only-sig-bypass"), feature = "regular"))]
    pub(crate) disable_sdk: DisableSdkConfiguration,
}

#[cfg(feature = "cn_beta_1_4_0")]
pub(crate) const CONFIG: InjectConfiguration = InjectConfiguration {
    f_pak_file_check: 0x3E37D90,
    f_pak_file_check_preamble: 0x8148574157565340,
    #[cfg(all(not(feature = "only-sig-bypass"), feature = "regular"))]
    kuro_http_get: 0xFE9E00,
    #[cfg(all(not(feature = "enable-sdk"), not(feature = "only-sig-bypass"), feature = "regular"))]
    disable_sdk: DisableSdkConfiguration{
        sdk_dll: s!("KRSDKEx.dll"),
        eula_accept: 0x4A6D0,
        sdk_go_away: 0x8BB40,
    }
};

#[cfg(feature = "cn_live_1_4_0")]
pub(crate) const CONFIG: InjectConfiguration = InjectConfiguration {
    f_pak_file_check: 0x3E3FCB0,
    f_pak_file_check_preamble: 0x5741544156535540,
    #[cfg(all(not(feature = "only-sig-bypass"), feature = "regular"))]
    kuro_http_get: 0xFED3E0,
    #[cfg(all(not(feature = "enable-sdk"), not(feature = "only-sig-bypass"), feature = "regular"))]
    disable_sdk: DisableSdkConfiguration{
        sdk_dll: s!("KRSDKEx.dll"),
        eula_accept: 0x4A6D0,
        sdk_go_away: 0x8BB40,
    }
};

#[cfg(feature = "os_live_1_4_0")]
pub(crate) const CONFIG: InjectConfiguration = InjectConfiguration {
    f_pak_file_check: 0x3DE6650,
    f_pak_file_check_preamble: 0x5741544156535540,
    #[cfg(all(not(feature = "only-sig-bypass"), feature = "regular"))]
    kuro_http_get: 0xFEA7A0,
    #[cfg(all(not(feature = "enable-sdk"), not(feature = "only-sig-bypass"), feature = "regular"))]
    disable_sdk: DisableSdkConfiguration{
        sdk_dll: s!("KRSDK.dll"),
        eula_accept: 0x959D0,
        sdk_go_away: 0xA1810
    }
};