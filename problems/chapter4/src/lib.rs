///
///
///

pub fn my_prefix(base: String, before: String) -> String {
    format!("{}{}", before, base)
}

pub fn my_suffix(base: String, after: String) -> String {
    format!("{}{}", base, after)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prefix() {
        assert_eq!("thisguy", my_prefix("guy".to_string(), "this".to_string()));
    }
}
