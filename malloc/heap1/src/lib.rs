#![feature(intrinsics)]

#[macro_use]
extern crate lazy_static;

mod zero;

use crate::types::{Header, MBox};
use std::borrow::Borrow;

mod types;

pub fn boxy_malloc(size: usize, malloc_root: &MBox) -> MBox {
    let mut prev = unsafe {
        malloc_root.borrow()
    };
    if prev.is_null() {

    }

    let new_box = MBox::null();
    new_box
}

// pub fn malloc(size: usize) -> *mut u8 {
pub fn malloc(size: usize, malloc_root: &MBox) {
    boxy_malloc(size, malloc_root.clone()).data();
}

#[cfg(test)]
mod tests {
    use crate::malloc;
    use crate::types::{MBox, Header};
    use std::borrow::Borrow;

    #[test]
    fn call_malloc() {
        let mut malloc_root: MBox = MBox::new(0 as *mut Header);
        unsafe {
            malloc(8888, malloc_root.borrow());
        }
    }
}