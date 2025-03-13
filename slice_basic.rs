// slice length not req at compile time
// string liter: str, hardcoded

fn main() {
    let arr: [i32; 3] = [1, 2, 3];
    let s1: &[i32] = &arr[0..3]; // syntax : passed by ref , the type of the elements which are
                                 // to be sliced
    let _s2: &str = "hello world!"; // stirng literal, always passed by ref

    let s: [char; 3] = ['a', 'b', 'c'];
    let slice = &s[..1];
    let size = std::mem::size_of_val(&slice);
    println!("size of s = {}", std::mem::size_of_val(&s));
    println!("size of slice = {}", size);
    println!("Success!");

    // Explanation:
    // A slice (&[T]) in Rust is a "fat pointer" comprised of:
    // 1. A pointer to the starting element of the slice (8 bytes on most 64-bit systems).
    // 2. The length of the slice (a usize, which is also 8 bytes on most 64-bit systems).
    // Therefore, the total size of the slice itself is 8 bytes (pointer) + 8 bytes (length) = 16 bytes.
    // This 16-byte size represents the slice's metadata, not the size of the underlying data it refers to.
    assert!(std::mem::size_of_val(&s1) == 16);
    println!("s1:  => {:?}", s1); // Not happening, can't print the whole string here.
                                  // let slice1 : &str = &s[x..y] // considers x to y-1 'bytes'
                                  // let slice2 : &[i32] = &arr[x..y] // considers x to y-1 'int's.
}
