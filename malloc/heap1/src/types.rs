use crate::zero::size_of;

pub struct Header {
    prev: MBox,
    next: MBox,
    size: usize,
    free: bool,
}

impl Header {
    pub fn default() -> Header {
        Header {
            prev: MBox::null(),
            next: MBox::null(),
            size: 0,
            free: true,
        }
    }
}

#[inline(always)]
pub fn header_size() -> usize {
    // function rather than macro because a macro complains about
    // unnecessary `unsafe` if used inside an `unsafe` block
    unsafe { size_of::<Header>() }
}

// (header, data...)
pub struct MBox(*mut Header);

// [header](data...)
pub struct Data(*mut u8);

impl MBox {
    pub fn new(header: *mut Header) -> MBox {
        MBox(header)
    }
    #[inline(always)]
    pub fn null() -> MBox { MBox::from_uint(0) }
    #[inline(always)]
    pub fn from_ptr(ptr: *mut u8) -> MBox {
        MBox::new(ptr as *mut Header)
    }
    #[inline(always)]
    pub fn from_uint(u: usize) -> MBox { MBox(u as *mut Header) }
    #[inline(always)]
    pub fn is_null(&self) -> bool {
        **self as usize == 0
    }
    #[inline]
    pub fn data(&self) -> Data {
        Data::from_uint((**self as usize) + header_size())
    }
}

impl Data {
    #[inline(always)]
    pub fn from_uint(u: usize) -> Data {
        Data(u as *mut u8)
    }
    pub fn mbox(&self) -> MBox {
        MBox::from_uint((*(*self) as usize) - header_size())
    }
}