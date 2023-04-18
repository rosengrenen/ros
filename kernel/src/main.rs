#![no_std]
#![no_main]

use core::panic::PanicInfo;

use uefi::{string::RawString16, SystemTable};

#[no_mangle]
pub extern "sysv64" fn _start(st: &'static SystemTable) -> usize {
    let text: [u16; 22] = [
        0x0068, 0x0065, 0x006c, 0x006c, 0x006f, 0x0020, 0x0066, 0x0072, 0x006f, 0x006d, 0x0020,
        0x0074, 0x0068, 0x0065, 0x0020, 0x006b, 0x0065, 0x0072, 0x006e, 0x0065, 0x006c, 0x0000,
    ];
    let string = RawString16(text.as_ptr() as _);
    st.inner.con_out.reset(false);
    st.inner.con_out.output_string(&string);
    // loop {}
    return 42;
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // println!("{}", info);
    loop {}
}
