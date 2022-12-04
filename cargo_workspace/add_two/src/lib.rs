pub fn add_two(x: i32) -> i32 {
    x + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_two() {
        let number = add_two(1);
        assert_eq!(number, 3);
    }
}
