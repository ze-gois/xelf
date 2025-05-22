use core::panic::PanicInfo;

use xelf::{exit, info};

#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    let mut count = 5;
    loop {
        count -= 1;
        info!(".");
        if count == 0 {
            info!("\n");
            exit();
        }
    }
}
