use crate::io::uart;
use cdm::execution::halt;
use cdm_rt::exception;
use core::panic::PanicInfo;
use embedded_io::Write;

#[exception(Default)]
fn on_exception() -> ! {
    let _ = write!(uart(), "\nEXCEPTION: other\n");
    cdm::execution::halt();
}

#[exception(UnalignedSP)]
fn unaligned_sp() -> ! {
    let _ = write!(uart(), "\nEXCEPTION: unaligned stack pointer\n");
    cdm::execution::halt();
}

#[exception(UnalignedPC)]
fn unaligned_pc() -> ! {
    let _ = write!(uart(), "\nEXCEPTION: unaligned program counter\n");
    halt();
}

#[exception(InvalidInst)]
fn invalid_inst() -> ! {
    let _ = write!(uart(), "\nEXCEPTION: invalid instruction\n");
    halt();
}

#[exception(DoubleFault)]
fn double_fault() -> ! {
    let _ = write!(uart(), "\nEXCEPTION: double fault\n");
    halt();
}

#[inline(never)]
#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    let mut uart = uart();
    let _ = write!(uart, "\nPANIC: {}", info.message());
    if let Some(location) = info.location() {
        let _ = write!(uart, " at {}:{}", location.file(), location.line());
    }
    let _ = write!(uart, "\n");
    halt();
}
