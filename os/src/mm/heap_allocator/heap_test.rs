use core::{alloc::Layout, mem::size_of};

use crate::{
    info,
    mm::heap_allocator::{buddy_allocator::Heap, instrusive_linked_list::InLinkedList},
};

#[allow(unused)]
pub fn heap_test() {
    info!("heap test start!");
    use alloc::boxed::Box;
    use alloc::vec::Vec;
    extern "C" {
        fn sbss();
        fn ebss();
    }
    let bss_range = sbss as usize..ebss as usize;
    let a = Box::new(5);
    assert_eq!(*a, 5);
    assert!(bss_range.contains(&(a.as_ref() as *const _ as usize)));
    drop(a);
    let mut v: Vec<usize> = Vec::new();
    for i in 0..500 {
        v.push(i);
    }
    for i in 0..500 {
        assert_eq!(v[i], i);
    }
    assert!(bss_range.contains(&(v.as_ptr() as usize)));
    drop(v);
    test_linked_list();
    test_empty_heap();
    test_heap_add();
    test_heap_oom();
    test_heap_alloc_and_free();
    info!("heap test passed!");
}

pub fn test_linked_list() {
    let mut value1: usize = 0;
    let mut value2: usize = 0;
    let mut value3: usize = 0;
    let mut list = InLinkedList::new();

    list.push(&mut value1 as *mut usize);
    list.push(&mut value2 as *mut usize);
    list.push(&mut value3 as *mut usize);

    // Test links
    assert_eq!(value3, &value2 as *const usize as usize);
    assert_eq!(value2, &value1 as *const usize as usize);
    assert_eq!(value1, 0);

    // Test iter
    let mut iter = list.iter();
    assert_eq!(iter.next(), Some(&mut value3 as *mut usize));
    assert_eq!(iter.next(), Some(&mut value2 as *mut usize));
    assert_eq!(iter.next(), Some(&mut value1 as *mut usize));
    assert_eq!(iter.next(), None);

    // debug!("here");
    // Test iter_mut
    let mut iter_mut = list.iter_mut();
    // debug!("here");
    assert_eq!(iter_mut.next().unwrap().remove(), &mut value3 as *mut usize);

    // Test pop
    assert_eq!(list.pop(), Some(&mut value2 as *mut usize));
    assert_eq!(list.pop(), Some(&mut value1 as *mut usize));
    assert_eq!(list.pop(), None);
}

pub fn test_empty_heap() {
    let mut heap = Heap::new();
    assert!(heap.alloc(Layout::from_size_align(1, 1).unwrap()).is_err());
}

pub fn test_heap_add() {
    let mut heap = Heap::new();
    assert!(heap.alloc(Layout::from_size_align(1, 1).unwrap()).is_err());

    let space: [usize; 100] = [0; 100];
    unsafe {
        heap.add_to_heap(space.as_ptr() as usize, space.as_ptr().add(100) as usize);
    }
    let addr = heap.alloc(Layout::from_size_align(1, 1).unwrap());
    assert!(addr.is_ok());
}

pub fn test_heap_oom() {
    let mut heap = Heap::new();
    let space: [usize; 100] = [0; 100];
    unsafe {
        heap.add_to_heap(space.as_ptr() as usize, space.as_ptr().add(100) as usize);
    }

    assert!(heap
        .alloc(Layout::from_size_align(100 * size_of::<usize>(), 1).unwrap())
        .is_err());
    assert!(heap.alloc(Layout::from_size_align(1, 1).unwrap()).is_ok());
}

pub fn test_heap_alloc_and_free() {
    let mut heap = Heap::new();
    assert!(heap.alloc(Layout::from_size_align(1, 1).unwrap()).is_err());

    let space: [usize; 100] = [0; 100];
    unsafe {
        heap.add_to_heap(space.as_ptr() as usize, space.as_ptr().add(100) as usize);
    }
    for _ in 0..100 {
        let addr = heap.alloc(Layout::from_size_align(1, 1).unwrap()).unwrap();
        heap.dealloc(addr, Layout::from_size_align(1, 1).unwrap());
    }
}
