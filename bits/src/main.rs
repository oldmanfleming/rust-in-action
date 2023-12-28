fn main() {
    let a: f32 = 42.42;

    let frankentype: u32 = unsafe { std::mem::transmute(a) };

    println!("{}", frankentype);
    println!("{:032b}", frankentype);

    let big_endian: [u8; 4] = [0xAA, 0xBB, 0xCC, 0xDD];
    let little_endian: [u8; 4] = [0xDD, 0xCC, 0xBB, 0xAA];

    let b: i32 = unsafe { std::mem::transmute(big_endian) };
    let c: i32 = unsafe { std::mem::transmute(little_endian) };

    println!("{}", b);
    println!("{}", c);
}
