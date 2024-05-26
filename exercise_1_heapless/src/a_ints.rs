use heapless::Vec;

/// Create a list of three items
///
/// Return a list with values 1, 2 and 3.
///
/// - [ ] Create slice of 1, 2 and 3
/// - [ ] Convert slice to Vec
pub fn get_vec_of_3_items() -> Vec<i8, 3> {
    // Make this a heapless list (Vec) of three items (1, 2 and 3).)
    todo!("Create vector of 3 items containing 1 2 and 3")
}

/// Converts given number from integer size 8 to integer size 16
///
/// - [x] Convert number to i16
pub fn convert_to_u16(number: i8) -> i16 {
    number as i16
}

/// Calculates sum of slice of numbers
///
/// Returns the sum value of the given slice of numbers.
///
/// - [ ] Iterate over all numbers
/// - [ ] Convert number to i16
/// - [ ] Calculate sum of slice
pub fn get_sum_of_items(items: &[i8]) -> i16 {
    // Compute the sum of the given slice
    // As heapless implements std traits, you can use use iter() to get a sum
    // map() is required to convert the input from integer size 8 to integer size 16. Rust does not allow implicit conversion.
    todo!("Calculate the sum of items, use map to convert types")
}

#[cfg(test)]
mod tests {
    use crate::a_ints::{get_sum_of_items, get_vec_of_3_items};

    #[test]
    fn test_list_has_items() {
        let result = get_vec_of_3_items();
        assert_eq!(3, result.len());
        assert_eq!(1, result[0]);
        assert_eq!(2, result[1]);
        assert_eq!(3, result[2]);
    }

    #[test]
    fn test_sum_of_items() {
        let items = get_vec_of_3_items();
        let result = get_sum_of_items(&items);
        assert_eq!(6, result);
    }
}
