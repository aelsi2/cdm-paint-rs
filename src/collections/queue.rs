use core::mem::MaybeUninit;
use crate::collections::ErrNoSpace;

pub struct Queue<T, const S: usize> {
    array: [MaybeUninit<T>; S],
    start_index: usize,
    end_index: usize,
}

unsafe impl<T, const S: usize> Sync for Queue<T, S> {}

impl<T, const S: usize> Drop for Queue<T, S> {
    fn drop(&mut self) {
        let range = if self.start_index <= self.end_index {
            (self.start_index..self.end_index).chain(0..0)
        } else {
            (self.start_index..self.array.len()).chain(0..self.end_index)
        };
        for index in range {
            unsafe {
                self.array[index].assume_init_drop();
            }
        }
    }
}

impl<T, const S: usize> Queue<T, S> {
    pub const fn new() -> Self {
        unsafe {
            Queue {
                array: MaybeUninit::uninit().assume_init(),
                start_index: 0,
                end_index: 0,
            }
        }
    }

    pub fn enqueue(&mut self, value: T) -> Result<(), ErrNoSpace> {
        let start_index = self.start_index;
        let end_index = self.end_index;
        let next_index = (end_index + 1) % S;

        if next_index == start_index {
            Err(ErrNoSpace)
        } else {
            self.array[end_index] = MaybeUninit::new(value);
            self.end_index = next_index;
            Ok(())
        }
    }
    
    pub fn dequeue(&mut self) -> Option<T> {
        let start_index = self.start_index;
        let end_index = self.end_index;
        let next_index = (start_index + 1) % S;

        if start_index == end_index {
            None
        } else {
            let value = unsafe {
                self.array[start_index].assume_init_read()
            };
            self.start_index = next_index;
            Some(value)
        }
    }
}
