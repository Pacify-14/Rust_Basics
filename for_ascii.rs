fn main() {
    println!("Here are the numbers: ");
    for i in 1..7 {
        // 1..7 -> for '=' na ho to < 7 tak loop
        println!("{}", i);
    }
    for c in 'a'..='z' {
        // 'a'..='z' means <= 'z' tak loop
        println!("{}", c as u16);
    }
}
