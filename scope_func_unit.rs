fn main() {
    let s = sum(1, 2);
    println!("value of sum is : {}", s);
    let y = {
        let mut x = 5;
        x += 5; // only not putting semicolumn here is not going to assign y value, as this is an
                // assigning statement.

        //  x;
        let y = 10; // y is a local variable so i don't think it would cause any problem
        x + y // no semicolumn to return value ;
    };
    println!("value of y using scope is : {}", y);
}

fn sum(x: i32, y: i32) -> i32 {
    x + y // no semicolumn should be here to get the return statement
}
