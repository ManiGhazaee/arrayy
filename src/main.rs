use arrayy::array;

fn main() {
    let a = array!(1, 2, 3; 10);
    let b = a.map(|i| if i % 2 == 0 { "even" } else { "odd" });
    println!("{:?}", a);
    println!("{:?}", b);
}
