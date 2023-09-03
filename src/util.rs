use std::collections::HashSet;

pub fn has_duplicates<T: std::hash::Hash + std::cmp::Eq>(vec: &[T]) -> bool {
    let mut seen = HashSet::new();
    vec.iter().any(|value| !seen.insert(value))
}

#[cfg(test)]
pub mod tests {
    use super::has_duplicates;

    #[test]
    fn returns_true_when_duplicates_present() {
        let list = vec![
            "test", "string", "fuuuuuuu", "test", "uhhhhh", "wut", "sooper",
        ];

        assert!(has_duplicates(&list));
    }

    #[test]
    fn returns_false_when_duplicates_not_present() {
        let list = vec!["test", "t est", "t\\est", "tEst", "TEST", "", " "];

        assert!(!has_duplicates(&list));
    }
}
