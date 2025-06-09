mod error;
use std::fmt::Display;

use bincode::{config, error::EncodeError, Decode, Encode};

use crate::table::error::TableError;

#[derive(Encode, Decode)]
pub struct Row {
    pub id: u32,
    pub username: String,
    pub email: String,
}

impl Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.id, self.username, self.email)
    }
}

#[derive(Encode, Decode)]
struct Page {
    raw: Box<Vec<u8>>,
    index: usize,
}

const PAGE_SIZE: usize = 4096;
impl Page {
    fn new() -> Self {
        Self {
            raw: Box::new(vec![0_u8; PAGE_SIZE]),
            index: 0,
        }
    }

    fn insert_row(&mut self, row: &Row) -> Result<(), TableError> {
        let config = config::standard();
        match bincode::encode_into_slice(&row, &mut (self.raw.as_mut_slice()[self.index..]), config)
        {
            Ok(size) => {
                self.index += size;
                Ok(())
            }
            Err(EncodeError::UnexpectedEnd) => Err(TableError::PageIsFull),
            Err(e) => Err(e.into()),
        }
    }
}

struct PageIterator<'a> {
    page: &'a Page,
    iterator_index: usize,
}

impl<'a> PageIterator<'a> {
    fn new(page: &'a Page) -> Self {
        Self {
            page,
            iterator_index: 0,
        }
    }
}

impl<'a> Iterator for PageIterator<'a> {
    type Item = Row;

    fn next(&mut self) -> Option<Self::Item> {
        if self.iterator_index >= self.page.index {
            return None;
        }
        let config = config::standard();
        bincode::decode_from_slice(&self.page.raw[self.iterator_index..], config)
            .map(|(item, size)| {
                self.iterator_index += size;
                item
            })
            .ok()
    }
}

impl<'a> IntoIterator for &'a Page {
    type Item = Row;

    type IntoIter = PageIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        PageIterator::new(self)
    }
}

struct Pager {
    pages: Vec<Page>,
}

impl Pager {
    fn new(db_filename: &str) -> Result<Self, TableError> {
        let buffer = std::fs::read(db_filename)?;
        if buffer.len() == 0 {
            Ok(Self { pages: vec![] })
        } else {
            let config = config::standard();
            let (pages, _): (Vec<Page>, usize) = bincode::decode_from_slice(&buffer, config)?;
            Ok(Self { pages })
        }
    }

    fn persist(&self, db_filename: &str) -> Result<(), TableError> {
        let config = config::standard();
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(db_filename)?;
        bincode::encode_into_std_write(&self.pages, &mut file, config)?;
        Ok(())
    }

    fn add_page(&mut self) -> Result<&mut Page, TableError> {
        if self.pages.len() >= TABLE_MAX_PAGES {
            return Err(TableError::TableIsFull);
        }
        self.pages.push(Page::new());
        Ok(self.pages.last_mut().unwrap())
    }

    fn last_mut(&mut self) -> Option<&mut Page> {
        self.pages.last_mut()
    }

    fn page_count(&self) -> usize {
        self.pages.len()
    }

    fn get(&self, index: usize) -> Option<&Page> {
        self.pages.get(index)
    }
}

pub struct Table {
    num_rows: u32,
    pager: Pager,
    db_filename: String,
}

pub struct TableIterator<'a> {
    page_index: usize,
    table: &'a Table,
    page_iterator: Option<PageIterator<'a>>,
}

impl<'a> TableIterator<'a> {
    fn new(table: &'a Table) -> Self {
        TableIterator {
            page_index: 0,
            table,
            page_iterator: None,
        }
    }
}

impl<'a> IntoIterator for &'a Table {
    type Item = Row;

    type IntoIter = TableIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        TableIterator::new(&self)
    }
}

impl<'a> Iterator for TableIterator<'a> {
    type Item = Row;

    fn next(&mut self) -> Option<Self::Item> {
        let iter = match self.page_iterator.as_mut() {
            Some(iter) => iter,
            None => {
                let page = self.table.pager.get(self.page_index)?;
                self.page_iterator = Some(page.into_iter());
                self.page_iterator.as_mut().unwrap()
            }
        };
        match iter.next() {
            Some(row) => Some(row),
            None => {
                self.page_index += 1;
                if self.page_index >= self.table.pager.page_count() {
                    return None;
                }
                let page = self.table.pager.get(self.page_index)?;
                let mut iter = page.into_iter();
                iter.next()
            }
        }
    }
}

const TABLE_MAX_PAGES: usize = 100;
impl Table {
    pub fn open(filename: &str) -> Result<Table, TableError> {
        Ok(Self {
            num_rows: 0,
            pager: Pager::new(filename)?,
            db_filename: filename.to_string(),
        })
    }

    pub fn persist(&self) -> Result<(), TableError> {
        self.pager.persist(&self.db_filename)
    }

    pub fn insert_row(&mut self, row: &Row) -> Result<(), TableError> {
        let page = match self.pager.last_mut() {
            Some(page) => page,
            None => self.pager.add_page()?,
        };

        match page.insert_row(row) {
            Ok(_) => {
                self.num_rows += 1;
                Ok(())
            }
            Err(TableError::PageIsFull) => {
                let page = self.pager.add_page()?;
                page.insert_row(row).and_then(|_| {
                    self.num_rows += 1;
                    Ok(())
                })
            }
            Err(e) => Err(e),
        }
    }
}
