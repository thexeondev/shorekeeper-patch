use std::thread;
use std::time::Duration;

use ilhook::x64::Registers;
use interceptor::Interceptor;
use windows::core::{s, w, PCSTR, PCWSTR};
use windows::Win32::System::Console;
use windows::Win32::System::SystemServices::DLL_PROCESS_ATTACH;
use windows::Win32::{Foundation::HINSTANCE, System::LibraryLoader::GetModuleHandleA};

mod interceptor;

const FPAKFILE_CHECK: usize = 0x3D2F460;
const KUROHTTP_GET: usize = 0xFC8CF0;

unsafe fn thread_func() {
    Console::AllocConsole().unwrap();
    println!("Wuthering Waves signature check bypass");
    println!("Don't forget to visit https://discord.gg/reversedrooms");

    let module = GetModuleHandleA(PCSTR::null()).unwrap();
    println!("Base: {:X}", module.0 as usize);

    let mut interceptor = Interceptor::new();
    interceptor
        .replace(
            (module.0 as usize) + FPAKFILE_CHECK,
            fpakfile_check_replacement,
        )
        .unwrap();

    interceptor
        .attach((module.0 as usize) + KUROHTTP_GET, on_kurohttp_get)
        .unwrap();

    let krsdk_ex = loop {
        match GetModuleHandleA(s!("KRSDKEx.dll")) {
            Ok(handle) => break handle,
            Err(_) => thread::sleep(Duration::from_millis(1)),
        }
    };

    interceptor
        .replace((krsdk_ex.0 as usize) + 0x4A690, dummy)
        .unwrap();

    interceptor
        .replace((krsdk_ex.0 as usize) + 0x8BB80, dummy)
        .unwrap();

    println!("Successfully initialized!");
    thread::sleep(Duration::from_secs(u64::MAX));
}

unsafe extern "win64" fn on_kurohttp_get(reg: *mut Registers, _: usize) {
    let wstr = *((*reg).rcx as *const usize) as *mut u16;
    let url = PCWSTR::from_raw(wstr).to_string().unwrap();

    println!("HTTP GET: {url}");
    if url.ends_with("/index.json") {
        println!("index.json requested, redirecting");
        let new_wstr = w!("http://127.0.0.1:10001/index.json");
        std::ptr::copy_nonoverlapping(new_wstr.as_ptr(), wstr, new_wstr.as_wide().len() + 2);
    }
}

unsafe extern "win64" fn fpakfile_check_replacement(
    reg: *mut Registers,
    _: usize,
    _: usize,
) -> usize {
    let wstr = *(((*reg).rcx + 8) as *const usize) as *const u16;
    let pak_name = PCWSTR::from_raw(wstr).to_string().unwrap();
    println!("Trying to verify pak: {pak_name}, returning true");

    1
}

unsafe extern "win64" fn dummy(_: *mut Registers, _: usize, _: usize) -> usize {
    1
}

#[no_mangle]
unsafe extern "system" fn DllMain(_: HINSTANCE, call_reason: u32, _: *mut ()) -> bool {
    if call_reason == DLL_PROCESS_ATTACH {
        thread::spawn(|| thread_func());
    }

    true
}
