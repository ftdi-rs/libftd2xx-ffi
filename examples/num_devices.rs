use libftd2xx_ffi::{DWORD, FT_LIST_NUMBER_ONLY, FT_ListDevices, FT_STATUS, PVOID};

fn main() {
    let mut num_devs: DWORD = 5;
    let dummy: PVOID = std::ptr::null_mut();
    let status: FT_STATUS = unsafe {
        FT_ListDevices(
            (&raw mut num_devs).cast::<std::ffi::c_void>(),
            dummy,
            FT_LIST_NUMBER_ONLY,
        )
    };
    println!("Status: {status}");
    println!("Number of devices: {num_devs}");
}
