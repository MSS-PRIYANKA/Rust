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
