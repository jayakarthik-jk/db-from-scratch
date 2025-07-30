use std::{io, path::Path};

use super::{
    pager::Pager,
    row::{Row, ROWS_PER_PAGE, ROW_SIZE},
};

pub(crate) struct Table {
    pub(crate) row_count: usize,
    pub(crate) pager: Pager,
}

pub(crate) enum InsertError {
    MaxRowReached,
}

impl Drop for Table {
    fn drop(&mut self) {
        self.pager.flush(self.row_count);
    }
}

impl Table {
    pub(crate) fn new<P>(path: P) -> Result<Self, io::Error>
    where
        P: AsRef<Path>,
    {
        let pager = Pager::new(path)?;
        let row_count = pager.len() / ROW_SIZE;
        Ok(Self { pager, row_count })
    }

    pub(crate) fn insert(&mut self, row: Row) -> Result<(), InsertError> {
        // index of the page we what to insert
        let page_idx = self.row_count / ROWS_PER_PAGE;

        let Some(page) = self.pager.get(page_idx) else {
            return Err(InsertError::MaxRowReached);
        };
        // Remaining rows in current page
        let row_offset = self.row_count % ROWS_PER_PAGE;
        // How much bytes we what to skip
        let byte_offset = row_offset * ROW_SIZE;
        let row = row.serialize();
        page.data[byte_offset..byte_offset + ROW_SIZE].copy_from_slice(&row);
        self.row_count += 1;

        Ok(())
    }
}
