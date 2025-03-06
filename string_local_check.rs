fn main() {
    let s = String::from("hello");
    {
        let s1 = s.clone();
        // let s1 = s; // causing error, Ig what happens is s1 ka value is dropped , s1 takes it,
        // and then outside the scope, s1 is also dropped. Hence there is compilation error as we
        // println s1.
        println!("{}", s1);
    }
    println!("{} ", s);
}
