use crate::my_utils::{array_factory, input_num};

pub fn sorting_an_array() {
    println!("What size of array do you want?");
    let siz = input_num();
    let v = array_factory(siz);
    println!("Here is our random array : {:?}", v);
}
