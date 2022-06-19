#![no_std]
#![no_main]

use bit_iter;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn start() -> ! {
    let _x = bit_iter::BitIter::from(0);
    loop {}
}
