fn main() {
    let arr0 = [1, 2, 3];
    let arr: [char; 3] = ['a', 'b', 'c'];
    
    assert!(std::mem::size_of_val(&arr) == 12);

    println!("Success!");
}
