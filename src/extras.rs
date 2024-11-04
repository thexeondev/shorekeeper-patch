#![cfg(all(not(feature = "only-sig-bypass"), feature = "regular"))]
use std::sync::OnceLock;
use std::thread;
use std::time::Duration;

use ilhook::x64::Registers;
use windows::core::{PCSTR, PCWSTR};
use windows::Win32::System::LibraryLoader::GetModuleHandleA;

use crate::offsets::CONFIG;
use crate::replacer::{GenericReplacer, Replacer};

static CFG_SERVER_REPLACER: OnceLock<GenericReplacer> = OnceLock::new();

pub(crate) fn configure_extras(interceptor: &mut interceptor_rs::Interceptor) {
    let module = unsafe { GetModuleHandleA(PCSTR::null()) }.unwrap();
    println!("Game base: {:X}", module.0 as usize);

    interceptor
        .attach((module.0 as usize) + CONFIG.kuro_http_get, on_kurohttp_get)
        .unwrap();

    let krsdk_ex = loop {
        match unsafe { GetModuleHandleA(CONFIG.disable_sdk.sdk_dll) } {
            Ok(handle) => break handle,
            Err(_) => thread::sleep(Duration::from_millis(1)),
        }
    };

    interceptor
        .replace((krsdk_ex.0 as usize) + CONFIG.disable_sdk.eula_accept, dummy)
        .unwrap();

    interceptor
        .replace((krsdk_ex.0 as usize) + CONFIG.disable_sdk.sdk_go_away, dummy)
        .unwrap();
}

unsafe extern "win64" fn on_kurohttp_get(reg: *mut Registers, _: usize) {
    let wstr = *((*reg).rcx as *const usize) as *mut u16;
    let url = PCWSTR::from_raw(wstr).to_string().unwrap();
    println!("HTTP GET: {url}");

    let replacer = CFG_SERVER_REPLACER.get_or_init(|| {
        GenericReplacer {
            regex: regex::Regex::new(r#"^(?:https|http)://.*/([a-zA-Z0-9]{32}/index\.json)$"#).unwrap(),
            replacement: std::env::var("CFG_SERVER_URL").unwrap_or("127.0.0.1:10001".to_string()),
            scheme: std::env::var("CFG_SERVER_SCHEME").unwrap_or("http".to_string()),
        }
    });

    if let Some(result) = replacer.replace(url.as_str()) {
        println!("Redirecting to: {result}");
        // TODO: Track https://doc.rust-lang.org/nightly/unstable-book/library-features/str-from-utf16-endian.html to replace widestring when stabilized
        let new_url = widestring::U16CString::from_str(result.as_str()).unwrap();
        let new_wstr = PCWSTR::from_raw(new_url.as_ptr());
        std::ptr::copy_nonoverlapping(new_wstr.as_ptr(), wstr, new_wstr.as_wide().len() + 2);
    };
}

unsafe extern "win64" fn dummy(_: *mut Registers, _: usize, _: usize) -> usize {
    1
}