use std::iter::Peekable;

pub(crate) trait ConsumeIf<T, I>
where
    T: Iterator<Item = I>,
{
    /// Consumes the next element if it matches the predicate.
    ///
    /// Returns `true` if an element was consumed, `false` otherwise.
    fn consume_if<F>(&mut self, predicate: F) -> Option<I>
    where
        F: FnMut(&I) -> bool;

    /// Checks if the next element matches the predicate and consumes it if it does.
    ///
    /// Returns `true` if an element was consumed, `false` otherwise.
    fn if_consume<F>(&mut self, predicate: F) -> bool
    where
        F: FnMut(&I) -> bool,
    {
        self.consume_if(predicate).is_some()
    }

    fn consume_while<F>(&mut self, mut predicate: F) -> Option<I>
    where
        F: FnMut(&I) -> bool,
    {
        let mut last = None;
        while let Some(item) = self.consume_if(&mut predicate) {
            last = Some(item);
        }
        last
    }
}

impl<T, I> ConsumeIf<T, I> for Peekable<T>
where
    T: Iterator<Item = I>,
{
    fn consume_if<F>(&mut self, mut predicate: F) -> Option<I>
    where
        F: FnMut(&I) -> bool,
    {
        if let Some(item) = self.peek() {
            if predicate(item) {
                return self.next();
            }
        }
        None
    }
}
