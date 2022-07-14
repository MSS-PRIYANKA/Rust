fn main() {
    let names = [String::from("Sunfei"), "Sunface".to_string()];
    let name0 = names.get(0).unwrap();
    let _name1 = &names[1];
    println!("Success!");
}
