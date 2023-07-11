fn main() {
    // let mut hashtable: HashMap<i32, i32> = HashMap::new();
    // hashtable.insert(1, 10);
    // hashtable.insert(2, 20);

    // let raw_pointer = hashtable.as_ptr();
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
