use libftd2xx_ffi::{FT_ListDevices, DWORD, FT_LIST_NUMBER_ONLY, FT_STATUS, PVOID};

fn main() {
    let mut num_devs: DWORD = 5;
    let num_devs_ptr: *mut DWORD = &mut num_devs;
    let dummy: PVOID = std::ptr::null_mut();
    let status: FT_STATUS = unsafe {
        FT_ListDevices(
            num_devs_ptr as *mut std::ffi::c_void,
            dummy,
            FT_LIST_NUMBER_ONLY,
        )
    };
    println!("Status: {}", status);
    println!("Number of devices: {}", num_devs);
}
