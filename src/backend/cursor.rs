use super::{row::{Row, ROWS_PER_PAGE, ROW_SIZE}, table::Table};

pub(crate) struct Cursor<'table> {
    table: &'table mut Table,
    current: usize,
}

impl<'table> Cursor<'table> {
    pub(crate) fn from_start(table: &'table mut Table) -> Self {
        Self {
            table,
            current: 0
        }
    }

    pub(crate) fn from_end(table: &'table mut Table) -> Self {
        Self {
            current: table.row_count,
            table,
        }
    }

    pub(crate) fn end_of_table(&self) -> bool {
        self.current >= self.table.row_count
    }
}

impl<'table> Iterator for Cursor<'table> {
    type Item = Row;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.table.row_count {
            return None;
        }
        let page_idx = self.current / ROWS_PER_PAGE;
        let page = self.table.pager.get(page_idx)?;
        // Remaining rows in current page
        let row_offset = self.current % ROWS_PER_PAGE;
        // How much bytes we what to skip
        let byte_offset = row_offset * ROW_SIZE;
        let row: &[u8; ROW_SIZE] = &page.data[byte_offset..byte_offset + ROW_SIZE]
            .try_into()
            .expect("Unable to get row from page");

        self.current += 1;

        return Some(Row::deserialize(row));
    }
}
