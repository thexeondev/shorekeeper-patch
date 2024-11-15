#[cfg(feature = "cn_beta_1_3_0")]
pub(crate) const CONFIG: InjectConfiguration = InjectConfiguration {
    f_pak_file_check: 0x3D2F460,
    f_pak_file_check_preamble: 0x8148574157565340,
    #[cfg(all(not(feature = "only-sig-bypass"), feature = "regular"))]
    kuro_http_get: 0xFC8CF0,
    #[cfg(all(not(feature = "enable-sdk"), not(feature = "only-sig-bypass"), feature = "regular"))]
    disable_sdk: DisableSdkConfiguration{
        sdk_dll: s!("KRSDKEx.dll"),
        eula_accept: 0x4A690,
        sdk_go_away: 0x8BB80,
    }
};

#[cfg(feature = "cn_live_1_3_0")]
pub(crate) const CONFIG: InjectConfiguration = InjectConfiguration {
    f_pak_file_check: 0x3D35DF0,
    f_pak_file_check_preamble: 0x8148574157565340,
    #[cfg(all(not(feature = "only-sig-bypass"), feature = "regular"))]
    kuro_http_get: 0xFC9900,
    #[cfg(all(not(feature = "enable-sdk"), not(feature = "only-sig-bypass"), feature = "regular"))]
    disable_sdk: DisableSdkConfiguration{
        sdk_dll: s!("KRSDKEx.dll"),
        eula_accept: 0x4A690,
        sdk_go_away: 0x8B9F0,
    }
};

#[cfg(feature = "os_live_1_3_0")]
pub(crate) const CONFIG: InjectConfiguration = InjectConfiguration {
    f_pak_file_check: 0x3CDC430,
    f_pak_file_check_preamble: 0x8148574157565340,
    #[cfg(all(not(feature = "only-sig-bypass"), feature = "regular"))]
    kuro_http_get: 0xFC6C20,
    #[cfg(all(not(feature = "enable-sdk"), not(feature = "only-sig-bypass"), feature = "regular"))]
    disable_sdk: DisableSdkConfiguration{
        sdk_dll: s!("KRSDK.dll"),
        eula_accept: 0x95440,
        sdk_go_away: 0xA1280
    }
};