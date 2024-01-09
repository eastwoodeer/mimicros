use crate::println;

use allocator;

use core::alloc::Layout;

extern "C" {
    fn exception_vector_base();
}

pub extern "C" fn rust_start_main(cpuid: usize) {
    println!("cpuid: {}", cpuid);

    crate::mem::clear_bss();
    crate::arch::aarch64::exception::exception_init(exception_vector_base as usize);

    let mem: [usize; 4096] = [0; 4096];
    let mut heap = allocator::Heap::<64>::new();
    unsafe {
        heap.add_to_heap(mem.as_ptr() as usize, mem.as_ptr().add(1024) as usize);
    }

    println!("heap: {:?}, {:?}", heap.free_list, heap);

    let mut ptr_array: [usize; 3] = [0; 3];

    for i in 1..3 {
        if let Ok(ret) = heap.alloc(Layout::from_size_align(100, 256).expect("OK")) {
            ptr_array[i] = ret.as_ptr() as usize;
            println!("result: {:?}", ret);
        } else {
            println!("failed to alloc");
        }

        println!("heap: {:?}, {:?}", heap.free_list, heap);
    }

    for i in 1..3 {
        heap.dealloc(core::ptr::NonNull::new(ptr_array[i] as *mut u8).unwrap(), Layout::from_size_align(100, 256).expect("ALIGN"));
    }

    println!("heap: {:?}, {:?}", heap.free_list, heap);

    panic!("ends here");
}
