#![feature(intrinsics)]

extern "rust-intrinsic" {
    pub fn size_of<T>() -> usize;
}
