fn main() {
    let x: (i32, i32, (), &str/*string literal ,instead of 'String' */) /*way of initializing a tuple */ = (1, 2, (), "helloworld!");
    let y: (i32, i32, (), &str) = x; //earlier it was (i32, i32, (), String), changed to &str, as
                                     //we were mentioned in question to use copy, instead of clone,
                                     //else y = x.clone() would have been used.
    println!("{:?}, {:?}", x, y);
}
