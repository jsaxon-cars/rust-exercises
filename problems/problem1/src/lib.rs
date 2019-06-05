pub fn find_largest_element(elems: &Vec<i64>) -> Option<i64> {
    println!("Vector: {:?}", elems);

    let mut largest: Option<i64> = None;

    if elems.len() > 0 {
        for val in elems.iter() {
            if largest == None || *val > largest.unwrap() {
                largest = Some(*val);
            }
        }
    }
    largest
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_no_elements() {
        let empty_vec = Vec::new();

        let result = find_largest_element(&empty_vec);

        assert_eq!(result, None);
    }

    #[test]
    fn test_some_elements() {
        let myvec = vec![1, 5, 3, 8, 0];

        let result = find_largest_element(&myvec);

        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_negative_elements() {
        let myvec = vec![-1, -5, -3, -8];

        let result = find_largest_element(&myvec);

        assert_eq!(result, Some(-1));
    }
}
