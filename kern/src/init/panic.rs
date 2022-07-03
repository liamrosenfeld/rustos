use core::panic::PanicInfo;

use crate::console::kprintln;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kprintln!("");
    kprintln!("---------- PANIC ----------");

    if let Some(location) = info.location() {
        kprintln!("File: {}", location.file());
        kprintln!("Line: {}", location.line());
        kprintln!("Col: {}", location.column());
    } else {
        kprintln!("Location not availiable")
    }

    kprintln!("");
    kprintln!(
        "{}",
        info.message()
            .unwrap_or(&format_args!("no message included"))
    );
    kprintln!("---------------------------");

    loop {}
}
