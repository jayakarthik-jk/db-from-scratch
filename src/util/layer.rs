pub(crate) trait Layer<T, E>: Iterator<Item = Result<T, E>> {
    fn rewind(&mut self, item: T);
}

pub(crate) struct BufferedLayer<I, T, E>
where
    I: Iterator<Item = Result<T, E>>,
{
    buffer: Vec<T>,
    iter: I,
}

impl<I, T, E> BufferedLayer<I, T, E>
where
    I: Iterator<Item = Result<T, E>>,
{
    pub(crate) fn new(iter: I) -> Self {
        Self {
            buffer: Vec::new(),
            iter,
        }
    }
}

impl<I, T, E> Iterator for BufferedLayer<I, T, E>
where
    I: Iterator<Item = Result<T, E>>,
{
    type Item = Result<T, E>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.buffer.pop() {
            Some(Ok(item))
        } else {
            self.iter.next()
        }
    }
}

impl<I, T, E> Layer<T, E> for BufferedLayer<I, T, E>
where
    I: Iterator<Item = Result<T, E>>,
{
    fn rewind(&mut self, item: T) {
        self.buffer.push(item);
    }
}
