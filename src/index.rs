use super::row::RID;
use super::row::{FieldType, Row};
use std::collections::BTreeMap;

/// The Index structure
///
/// Use this to create an index on any column of a Row to achieve O(log n)
/// lookup for any key.
///
/// Index { index: {String("Foo"): [1, 2]} }
#[derive(Debug)]
pub struct Index {
    index: BTreeMap<FieldType, Vec<RID>>,
}

impl Index {
    pub fn new() -> Self {
        return Index {
            index: BTreeMap::new(),
        };
    }

    /// ```rust
    /// use kronicler::index::*;
    /// use kronicler::row::FieldType;
    /// use kronicler::row::Row;
    ///
    /// let mut name_bytes = [0u8; 64];
    /// let name_str = "Jake";
    /// let bytes = name_str.as_bytes();
    /// name_bytes[..bytes.len()].copy_from_slice(bytes);
    ///
    /// let mut index = Index::new();
    /// let row1 = Row::new(0, vec![FieldType::Name(name_bytes)]);
    /// let row2 = Row::new(1, vec![FieldType::Name(name_bytes)]);
    ///
    /// index.insert(row1, 0);
    /// index.insert(row2, 0);
    ///
    /// let results = index.get(
    ///     FieldType::Name(name_bytes),
    /// );
    ///
    /// assert_eq!(results.unwrap().len(), 2);
    /// ```
    pub fn insert(&mut self, row: Row, index_on_col: usize) {
        let key = row.fields[index_on_col].clone();
        let ids_node: Option<&mut Vec<usize>> = self.index.get_mut(&key);

        if let Some(ids_vec) = ids_node {
            ids_vec.push(row.id);
        } else {
            self.index.insert(key, vec![row.id]);
        }
    }

    // `get` now returns the RID instead of the Row
    // This separates the concerns better because an index
    // should not worry about how to read rows, even just by
    // implementing code from elsewhere.
    pub fn get(&self, key: FieldType) -> Option<Vec<RID>> {
        let ids_node = self.index.get(&key);

        if let Some(ids_vec) = ids_node {
            return Some(ids_vec.to_vec());
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_insert_test() {
        let mut rows = Vec::new();
        let mut index = Index::new();

        let mut name_bytes = [0u8; 64];
        let name_str = "Jake".as_bytes();
        name_bytes[..name_str.len()].copy_from_slice(name_str);

        let row_1 = Row::new(0, vec![FieldType::Name(name_bytes)]);
        rows.push(row_1.clone());

        index.insert(row_1, 0);

        let fetched_rows = index.get(FieldType::Name(name_bytes));

        assert_eq!(fetched_rows.unwrap()[0], 0);
    }

    #[test]
    fn duplicate_insert_test() {
        let mut index = Index::new();

        let mut name_bytes = [0u8; 64];
        let name_str = "Foo".as_bytes();
        name_bytes[..name_str.len()].copy_from_slice(name_str);

        let row_2 = Row::new(1, vec![FieldType::Name(name_bytes)]);
        let row_3 = Row::new(2, vec![FieldType::Name(name_bytes)]);
        index.insert(row_2, 0);
        index.insert(row_3, 0);

        let fetched_rows_opt_2 = index.get(FieldType::Name(name_bytes));
        let fetched_rows_2 = fetched_rows_opt_2.unwrap();

        println!("{:?}", index);

        assert_eq!(fetched_rows_2[0], 1);

        assert_eq!(fetched_rows_2[1], 2);
    }
}
