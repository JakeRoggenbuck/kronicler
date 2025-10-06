use super::page::{Page, PageID};
use super::row::FieldType;
use log::{info, warn};
use std::sync::{Arc, RwLock};

// I had planned to test many different hashmap implementations
type BHashMap<K, V> = std::collections::HashMap<K, V>;

// Idea for ID values:
// When you insert into a page, if's full, you then allocate a new page
// Each time you insert, this record gets a Record ID, and the value corresponds to when it was
// inserted. This tells us the location in memory. Using this, we can tell both which page it
// should be in, and where in the page it should be located
//
// If I want ID 0, I go to the 0th page, and the 0th value
// If I want ID 100, I go to the 0th page, and the 100th value
// If I want ID 513, I go to the 1st page, and the 0th value
//
// This way, when we check for a value, we might see that that page is not actually loaded into
// memory, this will cause us to find the file and then load it into the bufferpool.
//
// The page is `index // 512` and the value index is `index % 512`

pub struct Bufferpool {
    // Right now, there is no removal strategy
    pages_collections: Vec<BHashMap<PageID, Arc<RwLock<Page>>>>,
    page_index: PageID,
    page_limit: usize,
    page_hit_count: usize,
    page_miss_count: usize,
}

impl Bufferpool {
    pub fn new(column_count: usize) -> Self {
        let mut page_maps = vec![];
        for _ in 0..column_count {
            page_maps.push(BHashMap::new())
        }

        Bufferpool {
            pages_collections: page_maps,
            page_index: 0,
            page_limit: 0,
            page_hit_count: 0,
            page_miss_count: 0,
        }
    }

    // pub fn create_column(&mut self, column_index: usize) {
    //     // TODO: Create an internal column
    //     // self.columns[0].pages ...
    //     todo!()
    // }

    pub fn set_page_limit(&mut self, limit: usize) {
        self.page_limit = limit;
    }

    pub fn create_page(&mut self, column_index: usize, field_type: FieldType) -> Arc<RwLock<Page>> {
        let p = Page::new(self.page_index, column_index, field_type.get_size());
        let page = Arc::new(RwLock::new(p));

        self.pages_collections[column_index].insert(self.page_index, page.clone());
        self.page_index += 1;
        return page.clone();
    }

    pub fn size(&self) -> usize {
        self.page_index
    }

    pub fn empty(&self) -> bool {
        self.size() == 0
    }

    pub fn full(&self) -> bool {
        self.page_index >= self.page_limit
    }

    pub fn fetch(
        &mut self,
        index: usize,
        column_index: usize,
        field_type_size: usize,
    ) -> Option<FieldType> {
        let pid: usize = (index * field_type_size) / 512;
        let index_in_page = (index * field_type_size) % 512;

        if self.page_hit_count as f64 / ((self.page_hit_count + self.page_miss_count) as f64) < 0.40
        {
            warn!("Page hit rate is below 40%");
        }

        info!("Fetching index {} from page {}", index_in_page, pid);

        if self.pages_collections[column_index].contains_key(&pid) {
            let page = self.pages_collections[column_index].get(&pid);
            self.page_hit_count += 1;

            if let Some(p) = page {
                info!("Fetching value {} in page {}", index_in_page, pid);

                let b = p.read().unwrap();
                return b.get_value(index_in_page);
            }
        } else {
            // The page was not loading in yet, so open the page
            let mut page = Page::new(pid, column_index, field_type_size);
            page.open();
            self.page_miss_count += 1;

            // Then load the page into the pages_collections
            self.pages_collections[column_index].insert(pid, Arc::new(RwLock::new(page)));

            // Excellent use of recursion. Call this function again now that the page is loaded
            return self.fetch(index, column_index, field_type_size);
        }

        None
    }

    pub fn insert(&mut self, index: usize, column_index: usize, value: &FieldType) {
        let field_type_size = value.get_size();

        let pid: usize = (index * field_type_size) / 512;
        let index_in_page = (index * field_type_size) % 512;

        info!("Getting collection {}", column_index);
        let collection = &self.pages_collections[column_index];

        // Check if page is opened in bufferpool
        if collection.contains_key(&pid) {
            // Get the page because it was opened
            let poption = collection.get(&pid);
            self.page_hit_count += 1;

            let mut b = poption.unwrap().write().unwrap();
            // TODO: Remove clone if possible
            b.set_value(index_in_page, value.clone());

            // TODO: Should this always write?
            // If so, it should do so async
            b.write_page();
        } else {
            // Open the page cause it was not opened
            let mut new_page = Page::new(pid, column_index, value.get_size());
            new_page.open();
            self.page_miss_count += 1;

            // TODO: Remove clone if possible
            new_page.set_value(index_in_page, value.clone());

            // TODO: Should this always write?
            // If so, it should do so async
            new_page.write_page();

            // Make an Arc
            let page = Some(Arc::new(RwLock::new(new_page)));
            self.pages_collections[column_index].insert(pid, page.clone().unwrap());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::PAGE_SIZE;
    use crate::database::Database;

    #[test]
    fn a_setup_test() {
        let mut db = Database::new(true);
        db.capture("a".to_string(), vec![], 1, 2);
    }

    #[test]
    fn basic_integration_test() {
        let mut bpool = Bufferpool::new(3);

        // Each page is 4KB on an x86-64 machine
        // Adding 4 pages, is 16KB or 16384 bytes
        bpool.set_page_limit(4);
        assert_eq!(bpool.page_limit, 4);

        // Create 4KB of data
        let four_k_of_data: [u8; PAGE_SIZE] = [0; PAGE_SIZE];
        assert_eq!(std::mem::size_of_val(&four_k_of_data), 512 * 8);

        assert_eq!(bpool.size(), 0);
        assert!(bpool.empty());

        // Create a page of data
        let page_1_arc = bpool.create_page(0, FieldType::Epoch(0));
        {
            let mut page_1 = page_1_arc.write().unwrap();
            page_1.set_all_values(four_k_of_data);
            page_1.write_page();

            assert_eq!(page_1.size(), 0);
            assert_eq!(page_1.capacity(), PAGE_SIZE);
        }

        assert_eq!(bpool.size(), 1);
        assert!(!bpool.empty());
        assert!(!bpool.full());

        // Insert 3 more pages to fill the bufferpool
        let page_2_arc = bpool.create_page(0, FieldType::Epoch(0));
        {
            let mut page_2 = page_2_arc.write().unwrap();
            page_2.set_all_values(four_k_of_data);
            page_2.write_page();
        }

        let page_3_arc = bpool.create_page(0, FieldType::Epoch(0));
        {
            let mut page_3 = page_3_arc.write().unwrap();
            page_3.set_all_values(four_k_of_data);
            page_3.write_page();
        }

        let page_4_arc = bpool.create_page(0, FieldType::Epoch(0));
        {
            let mut page_4 = page_4_arc.write().unwrap();
            page_4.set_all_values(four_k_of_data);
            page_4.write_page();
        }

        assert_eq!(bpool.size(), 4);
        assert!(bpool.full());

        // Add another page after it's full
        let page_5_arc = bpool.create_page(0, FieldType::Epoch(0));
        {
            let mut page_5 = page_5_arc.write().unwrap();
            page_5.set_all_values(four_k_of_data);
            page_5.write_page();
        }

        // Since the limit is 4, it should have removed one page to allow space for this new one
        // TODO: Make a removal strategy and this will be true
        //assert_eq!(bpool.size(), 4);
        assert!(bpool.full());

        bpool.insert(0, 0, &FieldType::Epoch(100));

        let field_type_size = 16;

        // Read the 0th value
        let val = bpool.fetch(0, 0, field_type_size);

        // Read the first value
        assert_eq!(val.unwrap(), FieldType::Epoch(100));

        for x in 0..600 {
            bpool.insert(x + 1, 0, &FieldType::Epoch(2 * (x + 1) as u128));
        }

        assert_eq!(
            bpool.fetch(1, 0, field_type_size),
            Some(FieldType::Epoch(2))
        );
        assert_eq!(
            bpool.fetch(2, 0, field_type_size),
            Some(FieldType::Epoch(4))
        );
        assert_eq!(
            bpool.fetch(100, 0, field_type_size),
            Some(FieldType::Epoch(200))
        );

        assert_eq!(
            bpool.fetch(500, 0, field_type_size),
            Some(FieldType::Epoch(1000))
        );

        // Read after first page!
        assert_eq!(
            bpool.fetch(550, 0, field_type_size),
            Some(FieldType::Epoch(1100))
        );
    }
}
