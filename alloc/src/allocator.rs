// Copyright (c) 2022 The MobileCoin Foundation

use core::alloc::{GlobalAlloc, Layout};
use core::ffi::c_void;
use core::ptr::null_mut;
use mc_sgx_tservice_sys::{sgx_aligned_free, sgx_aligned_malloc};

struct Allocator;

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let align = layout.align();
        let memory = sgx_aligned_malloc(size, align, null_mut(), 0);
        memory as *mut u8
    }
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        sgx_aligned_free(ptr as *mut c_void);
    }
}
