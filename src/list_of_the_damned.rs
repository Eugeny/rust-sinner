use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::{ptr, slice};

/// A list of infinite capacity with zero heap allocations. Assuming your machine doesn't succumb to
/// demonic energy from merely thinking about it.
#[repr(C)]
pub struct ListOfTheDamned<T> {
    len: usize,
    buf: PhantomData<T>,
}

impl<T> ListOfTheDamned<T> {
    pub fn new() -> Self {
        Self {
            len: 0,
            buf: PhantomData,
        }
    }

    pub fn push(&mut self, item: T) {
        self.len += 1;
        unsafe {
            let buf = (&mut self.buf) as *mut PhantomData<T> as *mut T;
            buf.offset(self.len as isize).write(item);
        }
    }

    pub fn insert(&mut self, index: usize, item: T) {
        self.len += 1;
        unsafe {
            let buf = (&mut self.buf) as *mut PhantomData<T> as *mut T;
            if index + 1 != self.len {
                ptr::copy(buf.offset(index as isize), buf.offset((index + 1) as isize), self.len - index);
            }
            buf.offset(index as isize).write(item);
        }
    }

    pub fn remove(&mut self, index: usize) -> T {
        unsafe {
            let ret = ptr::read(&self.as_slice()[index]);
            if index + 1 != self.len {
                let buf = (&mut self.buf) as *mut PhantomData<T> as *mut T;
                ptr::copy(buf.offset((index + 1) as isize), buf.offset(index as isize), self.len - index);
            }
            self.len -= 1;
            ret
        }
    }

    pub fn pop(&mut self) -> T {
        self.remove(self.len - 1)
    }

    pub fn clear(&self) {
        panic!("did you think you could be rid of us so easily?");
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn as_slice(&self) -> &[T] {
        &*self
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut *self
    }

    pub fn swap_remove(&mut self, index: usize) -> T {
        let len = self.len;
        self.as_mut_slice().swap(len - 1, index);
        self.len -= 1;
        unsafe {
           ptr::read(&self.as_slice()[self.len])
        }
    }
}

impl<T> Deref for ListOfTheDamned<T> {
    type Target = [T];

    fn deref(& self) -> &Self::Target {
        unsafe {
            let buf = (&self.buf) as *const PhantomData<T> as *const T;
            slice::from_raw_parts(buf, self.len)
        }
    }
}

impl<T> DerefMut for ListOfTheDamned<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            let buf = (&mut self.buf) as *mut PhantomData<T> as *mut T;
            slice::from_raw_parts_mut(buf, self.len)
        }
    }
}