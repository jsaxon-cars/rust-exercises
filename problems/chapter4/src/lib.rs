//
//
//

pub fn prefix(base: &str, before: &str) -> &str {
    before + base
}

pub fn suffix(base: &str, after: &str) -> &str {
    hbase + after
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
