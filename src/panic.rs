use core::panic::PanicInfo;

use crate::{info, syscall};

#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    let mut count = 5;
    loop {
        count -= 1;
        info!(".");
        if count == 0 {
            info!("\n");
            syscall::exit(23);
        }
    }
}
