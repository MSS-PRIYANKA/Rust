1
fn main() {
    // Don't modify the following two lines!
    let (x, y) = (1, 2);
    let s = sum(x, y);

    assert_eq!(s, 3);

    println!("Success!");
}

fn sum(x:i32, y: i32)->i32 {
    x + y
}

2
fn main() {
   print();
}

// Replace i32 with another type
fn print() -> (){
   println!("Success!");
}

3
fn main() {
    never_return();

    //println!("Failed!");
}

fn never_return() -> ! {
    panic!();
}

fn main() {
    never_return();

    println!("Failed!");
}

fn never_return() -> ! {
    unimplemented!();
}

4
fn main() {
    println!("Success!");
}

fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => {
            // TODO
        }
        _ => {
            // TODO
        }
    };
    
    // Rather than returning a None, we use a diverging function instead
    never_return_fn()
}

// IMPLEMENT this function in THREE ways
fn never_return_fn() -> ! {
    unimplemented!();
}

  fn main() {
    println!("Success!");
}

fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => {
            // TODO
        }
        _ => {
            // TODO
        }
    };
    
    // Rather than returning a None, we use a diverging function instead
    never_return_fn()
}

// IMPLEMENT this function in THREE ways
fn never_return_fn() -> ! {
    panic!();
}

5

fn main() {
    // FILL in the blank
    let b = false;

    let v = match b {
        true => 1,
        
        false => {
            println!("Success!");
            panic!("we have no value for `false`, but we can panic");
        }
    };

    println!("Exercise Failed if printing out this line!");
}

