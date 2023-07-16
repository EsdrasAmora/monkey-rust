use hashbrown::HashSet;
use std::hash::Hash;

struct StorageHolderIter<'a, T: ?Sized> {
    storage: &'a mut dyn Iterator<Item = &'a T>,
    seen: HashSet<&'a T>,
}

impl<'a, T: ?Sized + Hash + Eq> Iterator for StorageHolderIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(x) = self.storage.next() {
            if self.seen.contains(x) {
                continue;
            }
            self.seen.insert(x);
            return Some(x);
        }
        None
    }
}

fn main() {
    let foo: Vec<_> = vec!["a", "b", "a", "cc", "cc", "d"];

    let mut thing = StorageHolderIter {
        storage: &mut foo.into_iter(),
        seen: HashSet::new(),
    };

    while let Some(x) = thing.next() {
        println!("{}", x)
    }
}
