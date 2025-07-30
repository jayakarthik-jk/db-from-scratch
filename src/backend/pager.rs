use std::{
    fs::File,
    io::{self, Read, Seek, Write},
    path::Path,
};

use crate::backend::page::PAGE_SIZE;

use super::{
    page::Page,
    row::{ROWS_PER_PAGE, ROW_SIZE},
};

pub(crate) const TOTAL_PAGE: usize = 100;

#[derive(Debug)]
pub(crate) struct Pager {
    file: File,
    file_length: usize,
    pages: [Option<Page>; TOTAL_PAGE],
}

impl Pager {
    pub(crate) fn new<P>(path: P) -> Result<Self, io::Error>
    where
        P: AsRef<Path>,
    {
        let file = File::options()
            .read(true)
            .write(true)
            .create(true)
            .open(path)?;

        let meta = file.metadata()?;
        let file_length = meta.len() as usize;
        println!("file length: {file_length}");
        let pages = [const { None }; TOTAL_PAGE];
        Ok(Self {
            file,
            file_length,
            pages,
        })
    }

    pub(crate) fn len(&self) -> usize {
        self.file_length
    }

    pub(crate) fn get(&mut self, i: usize) -> Option<&mut Page> {
        if i >= TOTAL_PAGE {
            return None;
        }
        if self.pages[i].is_none() {
            let seeked = self
                .file
                .seek(io::SeekFrom::Start((i * PAGE_SIZE) as u64))
                .expect("Unable to seek into the file");

            println!("Seek: {seeked}");

            let mut page_data = [0u8; PAGE_SIZE];
            let read = self
                .file
                .read(&mut page_data)
                .expect("Unable to read from database");

            println!("Read: {read}");

            self.pages[i] = Some(Page::new(page_data));
        }

        self.pages[i].as_mut()
    }

    pub(crate) fn flush(&mut self, rows: usize) {
        let full_pages = rows / ROWS_PER_PAGE;

        for (i, page) in self.pages.iter().enumerate() {
            let Some(page) = page else {
                continue;
            };

            let rows = if i < full_pages {
                ROW_SIZE
            } else {
                rows % ROWS_PER_PAGE
            };
            // flush page
            self.file
                .seek(io::SeekFrom::Start(i as u64))
                .expect("Unable to seek into database");

            self.file
                .write(&page.data[..(rows * ROW_SIZE)])
                .expect("Unable to write into database");
        }
    }
}
