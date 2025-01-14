
pub(crate) const PAGE_SIZE: usize = 4096;

#[derive(Debug)]
pub(crate) struct Page {
    pub(crate) data: [u8; PAGE_SIZE],
}

impl Page {
    pub(crate) fn new(data: [u8; PAGE_SIZE]) -> Self {
        Self { data }
    }
    pub(crate) fn get_content(&self) -> &[u8; PAGE_SIZE] {
        &self.data
    }
}
