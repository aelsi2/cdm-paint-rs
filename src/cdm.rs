use core::panic::PanicInfo;
use critical_section::RawRestoreState;
use embedded_alloc::LlffHeap as Heap;

#[global_allocator]
static HEAP: Heap = Heap::empty();

pub unsafe fn initialize() {
    embedded_alloc::init!(HEAP, 2048);
}

struct CDMCriticalSection;
critical_section::set_impl!(CDMCriticalSection);

unsafe impl critical_section::Impl for CDMCriticalSection {
    unsafe fn acquire() -> RawRestoreState {
        disable_int()
    }

    unsafe fn release(token: RawRestoreState) {
        restore_int(token)
    }
}

unsafe extern "C" {
    pub safe fn halt() -> !;
    safe fn disable_int() -> bool;
    safe fn restore_int(interrupt: bool);
}

#[unsafe(no_mangle)]
extern "cdm-isr" fn on_exception() {
    halt();
}

#[panic_handler]
unsafe fn on_panic(_info: &PanicInfo) -> ! {
    halt();
}
