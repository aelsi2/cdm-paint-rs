use core::arch::cdm as arch;
use core::arch::global_asm;
use core::panic::PanicInfo;
use critical_section::RawRestoreState;
use embedded_alloc::LlffHeap as Heap;

#[repr(C)]
struct IvtEntry(extern "cdm-isr" fn() -> (), u16);

#[used]
#[unsafe(link_section = ".ivt")]
static IVT: [IvtEntry; 7] = [
    IvtEntry(crate::main, 0x8000),
    IvtEntry(on_exception, 0),
    IvtEntry(on_exception, 0),
    IvtEntry(on_exception, 0),
    IvtEntry(on_exception, 0),
    IvtEntry(crate::io::on_input, 0),
    IvtEntry(crate::io::on_timer, 0),
];

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

global_asm!(include_str!("./cdm.asm"));

extern "cdm-isr" fn on_exception() {
    unsafe { arch::halt() }
}

#[panic_handler]
fn on_panic(_info: &PanicInfo) -> ! {
    unsafe { arch::halt() }
}
