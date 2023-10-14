use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

pub fn has_duplicates<T: std::hash::Hash + std::cmp::Eq>(vec: &[T]) -> bool {
    let mut seen = HashSet::new();
    vec.iter().any(|value| !seen.insert(value))
}

pub fn expand_tilde_if_extant<P: AsRef<Path>>(path: P) -> Option<PathBuf> {
    let p = path.as_ref();
    if !p.starts_with("~") {
        return Some(p.to_path_buf());
    }
    if p == Path::new("~") {
        return dirs::home_dir();
    }

    dirs::home_dir().map(|mut h| {
        if h == Path::new("/") {
            // Edge case: home dir is root directory;
            // don't prepend extra `/`, just drop the tilde.
            p.strip_prefix("~").unwrap().to_path_buf()
        } else {
            h.push(p.strip_prefix("~/").unwrap());
            h
        }
    })
}

#[cfg(test)]
pub mod tests {
    use std::path::PathBuf;

    use crate::util::expand_tilde_if_extant;

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

    #[test]
    fn expands_tile_into_home_dir() {
        let home_dir = std::env::var("HOME").unwrap();
        let some_dir = PathBuf::from(format!("{}/repositories", home_dir));
        assert_eq!(expand_tilde_if_extant("~/repositories"), Some(some_dir));
        assert_eq!(expand_tilde_if_extant("/foo/bar"), Some("/foo/bar".into()));
        assert_eq!(
            expand_tilde_if_extant("~goldmund/projects"),
            Some("~goldmund/projects".into())
        );
    }
}
