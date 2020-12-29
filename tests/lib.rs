use libftd2xx_ffi::FT_GetLibraryVersion;

#[test]
#[cfg(not(windows))]
fn version() {
    let mut dw_library_ver = 0;

    let ft_status = unsafe { FT_GetLibraryVersion(&mut dw_library_ver) };
    assert_eq!(ft_status, 0);

    // version "1.4.8" is represented as 0x010408
    assert_eq!(dw_library_ver, 0x01_04_08);
}
