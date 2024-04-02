mod row;

#[cfg(test)]

mod tests {
    use crate::row::Row;

    #[test]
    fn insert_row() {
        let mut row_test = Row::from("this is test");

        let len = row_test.len();
        row_test.insert(3, 'w');
        // panic!("{}", row_test.string);
        assert_eq!(len + 1, row_test.len());
        // panic!(len == row_test.len())
    }

    #[test]
    fn delete_row() {
        let mut row_test = Row::from("this is test");
        let mut row_test1 = Row::from("this is test");
        row_test.delete(3, 'B');
        row_test1.delete(3, 'D');
        // panic!("")
        // panic!("{}",row_test.string);
        panic!("{}", row_test1.string);
    }
}
