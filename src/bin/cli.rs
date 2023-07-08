fn main() {
    let myvec = vec![1, 2];
    match myvec.as_slice() {
        [first] => {
            println!("{}", first)
        }
        _ => println!("hello"),
    };
}

trait MyTrait {
    fn my_method(&self);
}
