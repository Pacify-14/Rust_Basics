fn main() {
    let mut s = String::from("Hello!");
    /* let r1 = &mut s;
    let r2 = &mut s; // Problem: can't have 2 mutable reference to the same thing
    println!("r1: {}, r2: {}", r1, r2); // THROWS ERROR  */
    let _r1 = &s;
    let _r2 = &s; //NO problem, as we can have as many 'immutable' references to the same thing as
                  //posslbe
    println!("_r1: {}, _r2: {}", _r1, _r2);
    let r3 = &mut s;
    // println!("_r1: {}, _r2: {}, r3: {}", _r1, _r2, r3); // problem */
    println!("r3: {}", r3); // no problem, as simultaneously not accessed by mutable and immutable
                            // strings
}
