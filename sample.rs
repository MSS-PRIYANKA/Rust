fn main() {
/*
    let x = true;
    let res = if x{ "correct"} else {"404"};
    println!("res {}", res);
    */
    //loop
    //let mut counter = 10;
    /*let res = loop {
        counter += 1;
        println!("Never_ending");
        if counter == 10{
            break counter * 2;
            }
    };*/
    //while loop
    /*while counter != 0{
        println!("something");
        counter -= 1;
    };*/
    //println!("Never_ending { }", counter);
    // for in loop
    //let arr = [1, 23, 4, 56];
    for element in (1..=6).rev(){
        println!("element { }", element);
    };
}
