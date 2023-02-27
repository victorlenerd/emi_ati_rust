fn add_with_lifetimes<'a, 'b>(i: &'a i32, j: &'b i32) -> i32 {
    *i + *j
}

fn main() {
    let x = 12;
    let y = 32;
    let result = add_with_lifetimes(&x, &y);
    print!("sum {}", result)
}
