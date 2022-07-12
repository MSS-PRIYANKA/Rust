1
fn main() {
    let x: i32 = 5; 
    //let y: i32; 

    assert_eq!(x, 5);
    println!("Success!");
}

2
fn main() {
    let mut __ =  1;
    __ += 2; 
    
    assert_eq!(__, 3);
    println!("Success!");
}

3
fn main() {
    let x: i32 = 10;
    {
    let y: i32 = 5;
        println!("The value of y is {}",y);
    }
    println!("The value of x is {}", x); 
}

4
fn define_x() -> String{
    let x = String::from("hello");
    x
}

5
fn main() {
    
    println!("{}, world", define_x()); 
}

6
fn main() {
    let x: i32 = 12;
    {
        let x = 5;
        assert_eq!(x, 5);
    }

    assert_eq!(x, 12);

    let x =  42;
    println!("{}", x); // Prints "42".
}

7
// Remove a line in the code to make it compile
fn main() {
    let mut x: i32 = 1;
    x = 7;
    // Shadowing and re-binding
    let x = x; 
    //x += 3;


    let y = 4;
    // Shadowing
    let y = "I can also be bound to text!"; 

    println!("Success!");
}

8
fn main() {
    let x = 1; 
    assert_eq!(x, 1);
    //println!("{}", x);
}

9
fn main() {
    let (mut x, y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success!");
}

fn main() {
    let (x, y);
    (x,..) = (3, 4);
    [.., y] = [1, 2];
    // Fill the blank to make the code work
    assert_eq!([x,y], [3,2]);

    println!("Success!");
} 

