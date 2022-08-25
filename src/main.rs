#![no_std]
#![no_main]

use core::panic::PanicInfo;

pub fn send(buf: impl AsRef<[u8]>) -> isize {
    let buf = buf.as_ref();
    let res = unsafe {
        core::mem::transmute::<usize, extern "C" fn(*const u8, usize) -> isize>(0x1234)(
            buf.as_ptr(),
            buf.len(),
        )
    };

    res
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> isize {
    send([0xaa, 0xbb, 0xcc])
}
