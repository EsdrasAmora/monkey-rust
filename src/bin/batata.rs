struct SkipEveryOther<I> {
    iter: I,
    skip: bool,
}

impl<I> SkipEveryOther<I>
where
    I: Iterator,
{
    fn new(iter: I) -> Self {
        SkipEveryOther { iter, skip: true }
    }
}

impl<I> Iterator for SkipEveryOther<I>
where
    I: Iterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.iter.next();
        self.skip = !self.skip;
        if self.skip {
            self.next()
        } else {
            item
        }
    }
}

trait SkipEveryOtherExt: Iterator {
    fn skip_every_other(self) -> SkipEveryOther<Self>
    where
        Self: Sized,
    {
        SkipEveryOther::new(self)
    }
}

impl<T: ?Sized + Iterator> SkipEveryOtherExt for T {}

fn main() {
    let v = vec![1, 2, 3, 4, 5, 6];
    for i in v.iter().skip_every_other() {
        println!("{}", i);
    }
}
