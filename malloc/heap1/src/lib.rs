mod zero;

use crate::types::{Header, MBox};
use std::borrow::Borrow;

mod types;// pub for diagnostics
pub static mut malloc_root: MBox = MBox::new(0 as *mut Header);

pub fn boxy_malloc(size: usize) -> MBox {
    let mut prev = unsafe {
        malloc_root.borrow()
    };
    if prev.is_null() {

    }

    let new_box = MBox::null();
    new_box
}

pub fn malloc(size: usize) -> *mut u8 {
    *boxy_malloc(size).data()
}

#[cfg(test)]
mod tests {
    use crate::malloc;

    #[test]
    fn call_malloc() {
        malloc(8888);
    }
}