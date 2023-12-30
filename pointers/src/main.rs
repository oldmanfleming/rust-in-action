use std::{borrow::Cow, ffi::CStr, mem::size_of, os::raw::c_char};

static ARR_1: [u8; 10] = [99, 97, 114, 114, 121, 116, 111, 119, 101, 108];
static ARR_2: [u8; 11] = [116, 104, 97, 110, 107, 115, 102, 105, 115, 104, 0];

fn main() {
    let a: usize = 42;
    let b: &[u8; 10] = &ARR_1;
    let c: Box<[u8]> = Box::new(ARR_2);
    let d: String;
    let e: Cow<str>;
    unsafe {
        // d/e_ptr are raw pointers to the beginning of the memory region that contains the array
        // backing for the string
        let d_ptr = &ARR_1 as *const u8 as *mut u8;
        // looks like a bug since string expects a heap allocated array but we're giving it a
        // statically allocated one
        d = String::from_raw_parts(d_ptr, ARR_1.len(), ARR_1.len());
        let e_ptr = &ARR_2 as *const u8 as *const c_char;
        e = CStr::from_ptr(e_ptr).to_string_lossy();
    }
    println!("ARR_1 (an array of 10 bytes):");
    println!(" location: {:p}", &ARR_2);
    println!(" size: {:?} bytes", size_of::<[u8; 10]>());
    println!(" value: {:?}", ARR_1);
    println!();
    println!("ARR_2 (an array of 11 bytes):");
    println!(" location: {:p}", &ARR_2);
    println!(" size: {:?} bytes", size_of::<[u8; 11]>());
    println!(" value: {:?}", ARR_2);
    println!();
    println!("a (an unsigned integer):");
    println!(" location: {:p}", &a);
    println!(" size: {:?} bytes", size_of::<usize>());
    println!(" value: {:?}", a);
    println!();
    println!("b (a reference to ARR_1):");
    println!(" location: {:p}", &b);
    println!(" size: {:?} bytes", size_of::<&[u8; 10]>());
    println!(" points to: {:p}", b);
    println!(" value: {:?}", b);
    println!();
    println!("c (a \"box\" for ARR_2):");
    println!(" location: {:p}", &c);
    println!(" size: {:?} bytes", size_of::<Box<[u8]>>());
    println!(" points to: {:p}", c);
    println!(" value: {:?}", c);
    println!();
    println!("d (a String type backed by ARR_1):");
    println!(" location: {:p}", &d);
    println!(" size: {:?} bytes", size_of::<String>());
    println!(" value: {:?}", d.as_ptr());
    println!();
    println!("e (a Cow<str> backed by ARR_2):");
    println!(" location: {:p}", &e);
    println!(" size: {:?} bytes", size_of::<Cow<str>>());
    println!(" value: {:?}", e.as_ptr());
}
