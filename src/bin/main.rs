use std::fmt::Display;

fn main() {
    let v = vec![
        DummyStruct { name: "1" },
        DummyStruct { name: "2" },
        DummyStruct { name: "3" },
        DummyStruct { name: "4" },
        DummyStruct { name: "5" },
    ];

    let mut iter = v.into_iter();

    for _ in 0..3 {
        println!("original: {}", iter.next().unwrap());
    }

    let remaining_iter = iter.as_slice();

    for val in remaining_iter {
        println!("remaining: {}", val);
    }

    for val in iter {
        println!("original: {}", val);
    }
}

#[derive(Debug)]
struct DummyStruct<'a> {
    name: &'a str,
}

// impl Clone for DummyStruct<'_> {
//     fn clone(&self) -> Self {
//         // panic!();
//         println!("cloning: {}", self.name);
//         DummyStruct { name: self.name }
//     }
// }

impl Display for DummyStruct<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name)
    }
}
