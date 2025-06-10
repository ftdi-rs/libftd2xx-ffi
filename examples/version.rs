use libftd2xx_ffi::{DWORD, FT_GetLibraryVersion, FT_STATUS};

fn main() {
    let mut version: DWORD = 5;
    let status: FT_STATUS = unsafe { FT_GetLibraryVersion(&mut version) };
    println!("Status: {status}");
    println!("Version: {version}");
}
