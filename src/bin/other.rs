use std::iter::Peekable;

struct PeekableWrapper<I: Iterator> {
    inner: Peekable<I>,
}

impl<I: Iterator> PeekableWrapper<I> {
    fn new(iter: I) -> Self {
        PeekableWrapper {
            inner: iter.peekable(),
        }
    }

    fn peek_nth(&mut self, n: usize) -> Option<&I::Item>
    where
        I: Iterator,
    {
        let iter = self.inner.by_ref();
        for _ in 0..n {
            iter.peek()?;
            iter.next();
        }
        iter.peek()
    }
}

// Extend Peekable trait with new wrapper
trait PeekableExt: Iterator + Sized {
    fn peek_wrapper(self) -> PeekableWrapper<Self> {
        PeekableWrapper::new(self)
    }
}

impl<T: Iterator> PeekableExt for T {}

fn main() {
    let v = vec![1, 2, 3, 4, 5, 6];
    let mut peekable_wrapper = v.iter().peek_wrapper();

    // Peek at the third item
    if let Some(item) = peekable_wrapper.peek_nth(2) {
        println!("Third item: {}", item);
    } else {
        println!("Iterator is too short!");
    }

    for i in peekable_wrapper.inner {
        println!("{}", i);
    }
}
