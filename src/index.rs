use super::row::RID;
use super::row::{FieldType, Row};
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct IndexValue {
    ids: Vec<RID>,

    // We can actually add a value to the average without storing the total value
    // new_avg = ((old_avg + (current_index + 1)) + new_value) / (current_index + 2)
    pub average: Option<f64>,
}

/// The Index structure
///
/// Use this to create an index on any column of a Row to achieve O(log n)
/// lookup for any key.
///
/// Index { index: {String("Foo"): [1, 2]} }
#[derive(Debug)]
pub struct Index {
    pub index: BTreeMap<FieldType, IndexValue>,
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
    /// use kronicler::row::create_function_name;
    ///
    /// let name_bytes = create_function_name("Jake");
    ///
    /// let mut index = Index::new();
    /// let row1 = Row::new(0, vec![
    ///     FieldType::Name(name_bytes),
    ///     FieldType::Epoch(10),
    ///     FieldType::Epoch(20),
    ///     FieldType::Epoch(10),
    /// ]);
    /// let row2 = Row::new(1, vec![
    ///     FieldType::Name(name_bytes),
    ///     FieldType::Epoch(10),
    ///     FieldType::Epoch(20),
    ///     FieldType::Epoch(10),
    /// ]);
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
        let index_value = self.index.get_mut(&key);

        if let Some(found_index) = index_value {
            // Update average
            let n = found_index.ids.len();
            if let Some(avg) = found_index.average {
                let new_avg = ((avg * n as f64) + row.get_delta() as f64) / (n as f64 + 1.0);
                found_index.average = Some(new_avg);
            }

            // Push new value
            found_index.ids.push(row.id);
        } else {
            self.index.insert(
                key,
                IndexValue {
                    ids: vec![row.id],
                    average: Some(row.get_delta() as f64),
                },
            );
        }
    }

    // `get` now returns the RID instead of the Row
    // This separates the concerns better because an index
    // should not worry about how to read rows, even just by
    // implementing code from elsewhere.
    pub fn get(&self, key: FieldType) -> Option<Vec<RID>> {
        let ids_node = self.index.get(&key);

        if let Some(ids_vec) = ids_node {
            return Some(ids_vec.ids.to_vec());
        }

        None
    }

    pub fn get_average(&self, key: FieldType) -> Option<f64> {
        let ids_node = self.index.get(&key);

        if let Some(ids_vec) = ids_node {
            return ids_vec.average;
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::row::create_function_name;

    #[test]
    fn basic_insert_test() {
        let mut rows = Vec::new();
        let mut index = Index::new();

        let name_bytes = create_function_name("Jake");

        let row_1 = Row::new(
            0,
            vec![
                FieldType::Name(name_bytes),
                FieldType::Epoch(10),
                FieldType::Epoch(20),
                FieldType::Epoch(10),
            ],
        );
        rows.push(row_1.clone());

        index.insert(row_1, 0);

        let fetched_rows = index.get(FieldType::Name(name_bytes));

        assert_eq!(fetched_rows.unwrap()[0], 0);
    }

    #[test]
    fn duplicate_insert_test() {
        let mut index = Index::new();

        let name_bytes = create_function_name("Foo");

        let row_2 = Row::new(
            1,
            vec![
                FieldType::Name(name_bytes),
                FieldType::Epoch(10),
                FieldType::Epoch(20),
                FieldType::Epoch(10),
            ],
        );
        let row_3 = Row::new(
            2,
            vec![
                FieldType::Name(name_bytes),
                FieldType::Epoch(11),
                FieldType::Epoch(21),
                FieldType::Epoch(10),
            ],
        );
        index.insert(row_2, 0);
        index.insert(row_3, 0);

        let fetched_rows_opt_2 = index.get(FieldType::Name(name_bytes));
        let fetched_rows_2 = fetched_rows_opt_2.unwrap();

        println!("{:?}", index);

        assert_eq!(fetched_rows_2[0], 1);
        assert_eq!(fetched_rows_2[1], 2);
    }

    #[test]
    fn get_nonexistent_key_test() {
        let index = Index::new();

        let name_bytes = create_function_name("NonExistent");

        let result = index.get(FieldType::Name(name_bytes));
        assert!(result.is_none());
    }

    #[test]
    fn index_on_epoch_column_test() {
        let mut index = Index::new();

        let name_bytes = create_function_name("Test");

        let row_1 = Row::new(
            0,
            vec![
                FieldType::Name(name_bytes),
                FieldType::Epoch(100),
                FieldType::Epoch(200),
                FieldType::Epoch(100),
            ],
        );
        let row_2 = Row::new(
            1,
            vec![
                FieldType::Name(name_bytes),
                FieldType::Epoch(100),
                FieldType::Epoch(300),
                FieldType::Epoch(200),
            ],
        );

        // Index on column 1 (first Epoch field)
        index.insert(row_1, 1);
        index.insert(row_2, 1);

        let fetched_rows = index.get(FieldType::Epoch(100));
        assert_eq!(fetched_rows.unwrap().len(), 2);
    }

    #[test]
    fn average_calculation_single_row_test() {
        let mut index = Index::new();

        let name_bytes = create_function_name("Jake");

        let row = Row::new(
            0,
            vec![
                FieldType::Name(name_bytes),
                FieldType::Epoch(10),
                FieldType::Epoch(20),
                FieldType::Epoch(10),
            ],
        );

        index.insert(row, 0);

        let avg = index.get_average(FieldType::Name(name_bytes));
        assert!(avg.is_some());
        assert_eq!(avg.unwrap(), 10.0);
    }

    #[test]
    fn average_calculation_multiple_rows_test() {
        let mut index = Index::new();

        let name_bytes = create_function_name("Test");

        let row_1 = Row::new(
            0,
            vec![
                FieldType::Name(name_bytes),
                FieldType::Epoch(10),
                FieldType::Epoch(20),
                FieldType::Epoch(10),
            ],
        );
        let row_2 = Row::new(
            1,
            vec![
                FieldType::Name(name_bytes),
                FieldType::Epoch(10),
                FieldType::Epoch(30),
                FieldType::Epoch(20),
            ],
        );
        let row_3 = Row::new(
            2,
            vec![
                FieldType::Name(name_bytes),
                FieldType::Epoch(10),
                FieldType::Epoch(40),
                FieldType::Epoch(30),
            ],
        );

        index.insert(row_1, 0);
        index.insert(row_2, 0);
        index.insert(row_3, 0);

        let avg = index.get_average(FieldType::Name(name_bytes));
        assert!(avg.is_some());
        // Deltas are: 10, 20, 30. Average = 20.0
        assert_eq!(avg.unwrap(), 20.0);
    }

    #[test]
    fn average_nonexistent_key_test() {
        let index = Index::new();

        let name_bytes = create_function_name("Missing");

        let avg = index.get_average(FieldType::Name(name_bytes));
        assert!(avg.is_none());
    }

    #[test]
    fn multiple_keys_test() {
        let mut index = Index::new();

        let name_bytes_1 = create_function_name("Alice");
        let name_bytes_2 = create_function_name("Bob");

        let row_1 = Row::new(
            0,
            vec![
                FieldType::Name(name_bytes_1),
                FieldType::Epoch(10),
                FieldType::Epoch(20),
                FieldType::Epoch(10),
            ],
        );
        let row_2 = Row::new(
            1,
            vec![
                FieldType::Name(name_bytes_2),
                FieldType::Epoch(15),
                FieldType::Epoch(30),
                FieldType::Epoch(15),
            ],
        );
        let row_3 = Row::new(
            2,
            vec![
                FieldType::Name(name_bytes_1),
                FieldType::Epoch(20),
                FieldType::Epoch(40),
                FieldType::Epoch(20),
            ],
        );

        index.insert(row_1, 0);
        index.insert(row_2, 0);
        index.insert(row_3, 0);

        let alice_rows = index.get(FieldType::Name(name_bytes_1));
        let bob_rows = index.get(FieldType::Name(name_bytes_2));

        assert_eq!(alice_rows.unwrap().len(), 2);
        assert_eq!(bob_rows.unwrap().len(), 1);
    }

    #[test]
    fn empty_index_test() {
        let index = Index::new();
        assert_eq!(index.index.len(), 0);
    }

    #[test]
    fn large_batch_insert_test() {
        let mut index = Index::new();

        let name_bytes = create_function_name("Batch");

        // Insert 100 rows with the same key
        for i in 0..100 {
            let row = Row::new(
                i,
                vec![
                    FieldType::Name(name_bytes),
                    FieldType::Epoch(i as u128 * 10),
                    FieldType::Epoch(i as u128 * 20),
                    FieldType::Epoch(i as u128 * 10),
                ],
            );
            index.insert(row, 0);
        }

        let fetched_rows = index.get(FieldType::Name(name_bytes));
        assert_eq!(fetched_rows.unwrap().len(), 100);
    }

    #[test]
    fn index_preserves_insertion_order_test() {
        let mut index = Index::new();

        let name_bytes = create_function_name("Order");

        for i in 0..5 {
            let row = Row::new(
                i,
                vec![
                    FieldType::Name(name_bytes),
                    FieldType::Epoch(100),
                    FieldType::Epoch(200),
                    FieldType::Epoch(100),
                ],
            );
            index.insert(row, 0);
        }

        let fetched_rows = index.get(FieldType::Name(name_bytes)).unwrap();
        for i in 0..5 {
            assert_eq!(fetched_rows[i], i as RID);
        }
    }
}
