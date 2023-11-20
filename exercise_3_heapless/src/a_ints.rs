use heapless::Vec;

pub fn get_vec_of_3_items() -> Vec<u8, 3> {
    // Make this a heapless list (Vec) of three items (1, 2 and 3).
    todo!()
}

pub fn convert_to_u16(number: u8) -> u16 {
    number as u16
}

pub fn get_sum_of_items(items: &[u8]) -> u16 {
    // Compute the sum of the given slice
    todo!()
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
