fn main() {
    let x: i32 = 10;
    {
    let y: i32 = 5;
        println!("The value of y is {}",y);
    }
    println!("The value of x is {}", x); 
}
