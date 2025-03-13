/*Very imp: the size of the array should be known at the compile time, not the run time only. Hence
 * the function below would cause error.*/
/*
fn init_arr(n: i32){
let arr = [1;n]; //
}
*/

fn main() {
    let _arr: [i32; 5] = [1, 2, 3, 4, 5]; // arr:[T,size_of_array] , size is immutable in arr, T -> type of
                                          // elements in array
                                          //ignore array type or both the array type and the size too. (compiler infers it  on own)
    let _arr0 = [1, 2, 3];
    // all elements same value init
    let list: [i32; 100] = [1; 100];
    assert!(list[0] == 1);
    assert!(list.len() == 100);
    // safe option : get()
    let names: [String; 2] = [String::from("hello"), String::from("world!")];
    let _n0 = names.get(0).unwrap(); // option<T> return, safer than direct access.
    let _n1 = &names[1];
}
