fn main() {
    let s = String::from("hello!");
    let mut s1 = s; // ownership transferred
    s1.push_str("world!");
    println!("{}", s1);
}
