#[test]
#[cfg(not(windows))]
fn version() {
    use libftd2xx_ffi::FT_GetLibraryVersion;

    let mut dw_library_ver = 0;

    let ft_status = unsafe { FT_GetLibraryVersion(&mut dw_library_ver) };
    assert_eq!(ft_status, 0);

    cfg_if::cfg_if! {
        if #[cfg(target_os = "macos")] {
            assert_eq!(dw_library_ver, 0x01_04_24);
        } else {
            // version "1.4.22" is represented as 0x010422
            assert_eq!(dw_library_ver, 0x01_04_22);
        }
    }
}
