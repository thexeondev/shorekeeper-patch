GOTO:MAIN

:cargoReleaseBuild
    SETLOCAL ENABLEDELAYEDEXPANSION
        cargo clean
        cargo build --release --no-default-features -F %~1
        set features=%~1:,=-%
        COPY target\release\wicked_waifus_win.dll build\wicked-waifus-win-%~1.dll
        cargo clean
    ENDLOCAL
EXIT /B 0

:MAIN
if exist "build" rd /q /s "build"
mkdir build
cargo clean

: Build for cn_beta_1_4_0
call:cargoReleaseBuild cn_beta_1_4_0
: Build for cn_beta_1_3_0
call:cargoReleaseBuild cn_beta_1_3_0
: Build for cn_live_1_3_0
call:cargoReleaseBuild cn_live_1_3_0
: Build for os_live_1_3_0
call:cargoReleaseBuild os_live_1_3_0

tar -acvf wicked-waifus-win-patch-win64.zip -C build .