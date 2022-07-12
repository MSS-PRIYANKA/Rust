1

fn main() {
    let x: i32 = 5;
    let mut y = 5;

    y = x;
    
    let z: i32 = 10; // Type of z ? 

    println!("Success!");
}

2
fn main() {
    let v: u16 = 38_u8 as u16;

    println!("Success!");
}

3
fn main() {
    let x = 5;
    assert_eq!("i32".to_string(), type_of(&x));

    println!("Success!");
}
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

4
fn main() {
    assert_eq!(i8::MAX, 127); 
    assert_eq!(u8::MAX, 255); 

    println!("Success!");
}

5
fn main() {
   let v1 = 251_u16 + 8;
   let v2 = u16::checked_add(251, 8).unwrap();
   println!("{},{}",v1,v2);
}

6
fn main() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert_eq!(v ,1597);

    println!("Success!");
}

7
fn main() {
    let x = 1_000.000_1; // f64
    let y: f32 = 0.12; // f32
    let z = 0.01_f64; // f64

    println!("Success!");
}

8
fn main() {
    assert!(0.1_f32+0.2_f32==0.3_f32);
    assert_eq!(0.1_f32+0.2_f32, 0.3_f32);
    println!("Success!");
}

9
fn main() {
    let mut sum = 0;
    for i in -3..2 {
        sum += i
    }

    assert!(sum == -5);

    for c in 'a'..='z' {
        println!("{}",c as u8);
    }
}

10
use std::ops::{Range, RangeInclusive};
fn main() {
    assert_eq!((1..5), Range{ start: 1, end: 5 });
    assert_eq!((1..=5), RangeInclusive::new(1, 5));

    println!("Success!");
}

11
fn main() {
    assert!(1u32 + 2 == 3);
    assert!(1i32 - 2 == -1);
    assert!(1i8 - 2 == -1); 
    assert!(3 * 50 == 150);
    assert!(9 / 3 == 3); 
    assert!(24 % 5 == 4);
    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}