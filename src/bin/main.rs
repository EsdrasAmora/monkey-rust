fn main() {
    let v = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9", "10"];

    let mut iter = v.iter();

    // Consume the first 3 elements
    for _ in 0..3 {
        println!("original: {}", iter.next().unwrap());
    }

    // Create a new iterator that borrows from the original iterator
    let remaining_iter = iter.clone();

    for val in remaining_iter {
        println!("remaining: {}", val);
    }

    // Use the original iterator again
    for val in iter {
        println!("original: {}", val);
    }
}
