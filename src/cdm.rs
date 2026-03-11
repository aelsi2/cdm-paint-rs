use core::arch::cdm as arch;
use core::arch::global_asm;
use core::panic::PanicInfo;
use critical_section::RawRestoreState;
use embedded_alloc::LlffHeap as Heap;

global_asm!(include_str!("./cdm.asm"));
global_asm!(include_str!("./ivt.asm"));

#[global_allocator]
static HEAP: Heap = Heap::empty();

pub unsafe fn initialize() {
    embedded_alloc::init!(HEAP, 2048);
}

struct CDMCriticalSection;
critical_section::set_impl!(CDMCriticalSection);

unsafe impl critical_section::Impl for CDMCriticalSection {
    unsafe fn acquire() -> RawRestoreState {
        unsafe {
            let ps = arch::ldps();
            arch::di();
            ps
        }
    }

    unsafe fn release(token: RawRestoreState) {
        unsafe { arch::stps(token) }
    }
}

#[unsafe(no_mangle)]
extern "cdm-isr" fn on_exception() {
    unsafe { arch::halt() }
}

#[panic_handler]
fn on_panic(_info: &PanicInfo) -> ! {
    unsafe { arch::halt() }
}
