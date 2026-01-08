use core::{ops::Deref, panic::PanicInfo};

unsafe extern "C" {
    pub safe fn halt() -> !;
    safe fn disable_int() -> bool;
    safe fn restore_int(interrupt: bool);
}

pub struct Crit<T> {
    value: T,
}

impl<'a, T> Crit<T> {
    pub const fn new(value: T) -> Self {
        Crit { value }
    }
    
    pub fn enter(&'a self) -> CritGuard<'a, T> {
        CritGuard::new(&self.value)
    }

    pub fn with(&'a self, func: impl FnOnce(&T) -> ()) {
        let guard = CritGuard::new(&self.value);
        func(&*guard)
    }
}

pub struct CritGuard<'a, T> {
    value: &'a T,
    interrupt: bool,
}

unsafe impl<T> Sync for Crit<T> {}

impl<'a, T> CritGuard<'a, T> {
    pub fn new(value: &'a T) -> Self {
        CritGuard {
            value: value,
            interrupt: disable_int(),
        }
    }
}

impl<T> Deref for CritGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.value
    }
}

impl<T> Drop for CritGuard<'_, T> {
    fn drop(&mut self) {
        restore_int(self.interrupt);
    }
}

#[unsafe(no_mangle)]
extern "cdm-isr" fn on_exception() {
    halt();
}

#[panic_handler]
unsafe fn on_panic(_info: &PanicInfo) -> ! {
    halt();
}
