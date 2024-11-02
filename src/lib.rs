use std::thread;
use std::time::Duration;

use ilhook::x64::Registers;
use interceptor_rs::Interceptor;
use windows::core::{PCSTR, PCWSTR};
use windows::Win32::Foundation::HINSTANCE;
use windows::Win32::System::Console;
use windows::Win32::System::LibraryLoader::GetModuleHandleA;
use windows::Win32::System::SystemServices::DLL_PROCESS_ATTACH;

use offsets::CONFIG;

mod offsets;
#[cfg(all(not(feature = "only-sig-bypass"), feature = "regular"))]
mod replacer;
#[cfg(all(not(feature = "only-sig-bypass"), feature = "regular"))]
mod extras;

fn thread_func() {
    unsafe { Console::AllocConsole() }.unwrap();
    println!("Wuthering Waves essential binary patcher");
    println!("Don't forget to visit https://discord.gg/reversedrooms");

    println!("Waiting for ACE init");
    let module = unsafe { GetModuleHandleA(PCSTR::null()) }.unwrap();
    let pak_file_offset = ((module.0 as usize) + CONFIG.f_pak_file_check) as *const u64;
    loop {
        if unsafe { std::ptr::read(pak_file_offset) } == CONFIG.f_pak_file_check_preamble {
            println!("ACE Initialization finished");
            break;
        }
        thread::sleep(Duration::from_millis(1))
    }

    let mut interceptor = Interceptor::new();
    interceptor
        .replace((module.0 as usize) + CONFIG.f_pak_file_check, fpakfile_check_replacement)
        .unwrap();

    #[cfg(all(not(feature = "only-sig-bypass"), feature = "regular"))]
    extras::configure_extras(&mut interceptor);

    println!("Successfully initialized!");

    thread::sleep(Duration::from_secs(u64::MAX));
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

#[no_mangle]
unsafe extern "system" fn DllMain(_: HINSTANCE, call_reason: u32, _: *mut ()) -> bool {
    if call_reason == DLL_PROCESS_ATTACH {
        thread::spawn(|| thread_func());
    }

    true
}