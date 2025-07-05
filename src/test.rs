#[allow(warnings)]
pub fn ground() {
    let x = 5;
    println!(" {x}");
    let y = x + 1;
    println!(" {y}");
}

#[allow(warnings)]
fn print_slice(v: &mut [i32]) {
    println!("{:?}", v);
    println!("{}   {}   {}", v[0], v[1], v[2]);
    v[1] = 9;
    println!("{}   {}   {}", v[0], v[1], v[2]);
}

#[allow(warnings)]
pub fn test() {
    let mut v = vec![3, 5, 6, 4, 2, 4, 3];
    print_slice(&mut v[1..4]);
}
