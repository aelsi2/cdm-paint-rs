use core::mem::MaybeUninit;
use crate::collections::ErrNoSpace;

pub struct Stack<T, const S: usize> {
    array: [MaybeUninit<T>; S],
    top: usize,
}

impl<T, const S: usize> Drop for Stack<T, S> {
    fn drop(&mut self) {
        for index in 0..self.top {
            unsafe {
                self.array[index].assume_init_drop();
            }
        }
    }
}

impl<T, const S: usize> Stack<T, S> {
    pub const fn new() -> Self {
        Stack {
            array: unsafe { MaybeUninit::uninit().assume_init() },
            top: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.top == 0
    }

    pub fn push(&mut self, value: T) -> Result<(), ErrNoSpace> {
        if self.top == S - 1 {
            return Err(ErrNoSpace);
        }
        self.array[self.top] = MaybeUninit::new(value);
        self.top += 1;
        Ok(())
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.top == 0 {
            return None;
        }
        let value = unsafe { self.array[self.top - 1].assume_init_read() };
        self.top -= 1;
        Some(value)
    }
}
