use crate::io::uart;
use cdm::execution::halt;
use cdm_rt::exception;
use core::panic::PanicInfo;
use embedded_io::Write;

#[exception(UnalignedSP)]
fn unaligned_sp() -> ! {
    let _ = write!(uart(), "HARDWARE EXCEPTION: unaligned stack pointer\n");
    cdm::execution::halt();
}

#[exception(UnalignedPC)]
fn unaligned_pc() -> ! {
    let _ = write!(uart(), "HARDWARE EXCEPTION: unaligned program counter\n");
    halt();
}

#[exception(DoubleFault)]
fn double_fault() -> ! {
    let _ = write!(uart(), "HARDWARE EXCEPTION: double fault\n");
    halt();
}

#[exception(InvalidInst)]
fn invalid_inst() -> ! {
    let _ = write!(uart(), "HARDWARE EXCEPTION: invalid instruction\n");
    halt();
}

#[inline(never)]
#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    let mut uart = uart();
    let _ = write!(uart, "PANIC: {}", info.message());
    if let Some(location) = info.location() {
        let _ = write!(uart, " at {}:{}", location.file(), location.line());
    }
    let _ = write!(uart, "\n");
    halt();
}
