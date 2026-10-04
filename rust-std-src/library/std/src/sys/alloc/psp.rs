use super::MIN_ALIGN;
use crate::alloc::Layout;
use crate::ptr;

unsafe extern "C" {
    fn __psp_alloc(size: u32, align: u32) -> *mut u8;
    fn __psp_dealloc(ptr: *mut u8);
    fn __psp_realloc(ptr: *mut u8, old_size: u32, new_size: u32, align: u32) -> *mut u8;
}

#[inline]
pub unsafe fn alloc(layout: Layout) -> *mut u8 {
    unsafe { __psp_alloc(layout.size() as u32, layout.align() as u32) }
}

#[inline]
pub unsafe fn alloc_zeroed(layout: Layout) -> *mut u8 {
    let p = unsafe { alloc(layout) };
    if !p.is_null() {
        unsafe { ptr::write_bytes(p, 0, layout.size()) };
    }
    p
}

#[inline]
pub unsafe fn dealloc(ptr: *mut u8, _layout: Layout) {
    unsafe { __psp_dealloc(ptr) }
}

#[inline]
pub unsafe fn realloc(ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
    unsafe {
        __psp_realloc(ptr, layout.size() as u32, new_size as u32, layout.align() as u32)
    }
}
