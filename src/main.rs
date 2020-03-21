fn main() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    let r3 = &mut num as *mut i32;

    println!("{}", num);
    unsafe {
        println!("{}", *r1);
        println!("{}", *r2);
        *r3 = 10;
        println!("{}", *r1);
        println!("{}", *r2);
    }
    println!("{}", num);

    let address = 0x012345usize;
    let pointed = address as *mut i32;

    unsafe {
        *pointed = 666;
        // println!("{}", *pointed)
    }
}
