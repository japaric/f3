use cortex_m::interrupt::Mutex;
use linked_list_allocator::Heap;

// NOTE You need to initialize the Heap before you use it!
pub static HEAP: Mutex<Heap> = Mutex::new(Heap::empty());

#[no_mangle]
/// Rust allocation function (c.f. malloc)
pub extern "C" fn __rust_allocate(size: usize, align: usize) -> *mut u8 {
    HEAP.lock(|heap| {
        heap.allocate_first_fit(size, align).expect("out of memory")
    })
}

#[no_mangle]
/// Rust de-allocation function (c.f. free)
pub extern "C" fn __rust_deallocate(ptr: *mut u8, size: usize, align: usize) {
    HEAP.lock(|heap| unsafe { heap.deallocate(ptr, size, align) });
}

/// Rust re-allocation function (c.f. realloc)
#[no_mangle]
pub extern "C" fn __rust_reallocate(ptr: *mut u8,
                                    size: usize,
                                    new_size: usize,
                                    align: usize)
                                    -> *mut u8 {
    use core::{ptr, cmp};

    // from: https://github.com/rust-lang/rust/blob/
    //     c66d2380a810c9a2b3dbb4f93a830b101ee49cc2/
    //     src/liballoc_system/lib.rs#L98-L101

    let new_ptr = __rust_allocate(new_size, align);
    unsafe { ptr::copy(ptr, new_ptr, cmp::min(size, new_size)) };
    __rust_deallocate(ptr, size, align);
    new_ptr
}

/// Rust re-allocation function which guarantees not to move the data
/// somewhere else.
#[no_mangle]
pub extern "C" fn __rust_reallocate_inplace(_ptr: *mut u8,
                                            size: usize,
                                            _new_size: usize,
                                            _align: usize)
                                            -> usize {
    size
}

/// Some allocators (pool allocators generally) over-allocate. This checks how
/// much space there is at a location. Our allocator doesn't over allocate so
/// this just returns `size`
#[no_mangle]
pub extern "C" fn __rust_usable_size(size: usize, _align: usize) -> usize {
    size
}
