fn main() {
    let x: Box<i32> = Box::new(5); //unline usual i32 which is stored in stack, Bow i32 stored in heaps
    let mut y: Box<i32> = Box::new(1);
    *y = 4;
    assert_eq!(*x, 5);
}
