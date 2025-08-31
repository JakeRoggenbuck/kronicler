use once_cell::sync::Lazy;
use std::collections::BTreeMap;
use std::sync::Mutex;

pub type RID = usize;

#[derive(Debug, Eq, Clone, PartialEq, Ord, PartialOrd)]
pub enum FieldType {
    Int(i64),
    String(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Row {
    id: RID,
    fields: Vec<FieldType>,
}

// Pretend like this is a bufferpool that can "store" all of the rows.
// I put store in quotes, because it will keep some rows in memory, and
// others will be put on disk
// TODO: Use real bufferpool
// https://github.com/JakeRoggenbuck/logfrog/issues/19
static BUFFERPOOL_MOCK: Lazy<Mutex<Vec<Row>>> = Lazy::new(|| Mutex::new(Vec::new()));

pub fn get_row_from_bufferpool(id: usize) -> Row {
    let bp = BUFFERPOOL_MOCK.lock().unwrap();
    bp[id].clone()
}

// TODO: Move this out of index and use real bufferpool
// https://github.com/JakeRoggenbuck/logfrog/issues/19
impl Row {
    pub fn new(id: RID, fields: Vec<FieldType>) -> Self {
        let row = Row { id, fields };
        let mut b = BUFFERPOOL_MOCK.lock().unwrap();
        b.push(row.clone());
        row
    }
}

/// The Index structure
///
/// Use this to create an index on any column of a Row to achieve O(log n)
/// loopup for any key.
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
    /// use indexrs::*;
    ///
    /// let mut index = Index::new();
    /// let row1 = Row::new(0, vec![FieldType::String("Jake".to_string())]);
    /// let row2 = Row::new(1, vec![FieldType::String("Jake".to_string())]);
    ///
    /// index.insert(row1, 0);
    /// index.insert(row2, 0);
    ///
    /// let results = index.get(
    ///     FieldType::String("Jake".to_string()),
    ///     &get_row_from_bufferpool
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

    pub fn get(&self, key: FieldType, get_row: &dyn Fn(usize) -> Row) -> Option<Vec<Row>> {
        let ids_node = self.index.get(&key);

        let mut rows = Vec::<Row>::new();

        if let Some(ids_vec) = ids_node {
            // Collect all of the rows with that key
            ids_vec.iter().for_each(|id| rows.push(get_row(*id)));
            return Some(rows);
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_insert_test() {
        let mut index = Index::new();

        let row_1 = Row::new(0, vec![FieldType::String(String::from("Jake"))]);
        index.insert(row_1, 0);

        let fetched_rows = index.get(
            FieldType::String(String::from("Jake")),
            &get_row_from_bufferpool,
        );

        assert_eq!(
            fetched_rows.unwrap()[0].fields[0],
            FieldType::String(String::from("Jake"))
        );
    }

    #[test]
    fn duplicate_insert_test() {
        let mut index = Index::new();

        let row_2 = Row::new(1, vec![FieldType::String(String::from("Foo"))]);
        let row_3 = Row::new(2, vec![FieldType::String(String::from("Foo"))]);
        index.insert(row_2, 0);
        index.insert(row_3, 0);

        let fetched_rows_opt_2 = index.get(
            FieldType::String(String::from("Foo")),
            &get_row_from_bufferpool,
        );
        let fetched_rows_2 = fetched_rows_opt_2.unwrap();

        println!("{:?}", index);

        assert_eq!(
            fetched_rows_2[0].fields[0],
            FieldType::String(String::from("Foo"))
        );

        assert_eq!(
            fetched_rows_2[1].fields[0],
            FieldType::String(String::from("Foo"))
        );
    }
}
