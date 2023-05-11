use core::panic::PanicInfo;

use crate::api::exit;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        error!(
            "Panic occurred at {}:{} {}",
            location.file(),
            location.line(),
            info.message().unwrap()
        );
    } else {
        error!("Panicked: {}", info.message().unwrap());
    }
    // when user program panic, just hang up...
    exit(-1);
    loop {}
}
