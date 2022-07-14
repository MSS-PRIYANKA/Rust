fn main() {
    let raw_str = "Escapes don't work here: \x3F \u{211D}";
    assert_eq!(raw_str, "Escapes don't work here: ? ℝ");

    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    let  delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", delimiter);

    let long_delimiter = "Hello, \"##\"";
    assert_eq!(long_delimiter, "Hello, \"##\"");

    println!("Success!");
}
