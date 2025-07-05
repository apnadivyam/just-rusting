use crate::my_utils::{array_factory, binary_search_array, input_num, merge_sort};

pub fn sorting_an_array() {
    println!("What size of array do you want?");
    let siz: usize = input_num() as usize;
    let mut v = array_factory(siz);
    println!("Here is our random array : {:?}", v);
    merge_sort(&mut v, 0, siz);
    println!("Here is our sorted array : {:?}", v);
    println!("Would you like to binary search anything? (0/1)");
    if input_num() == 1 {
        binary_search_array(&v);
    }
}
