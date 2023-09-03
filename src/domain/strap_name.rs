#[derive(Debug)]
pub struct StrapName(String);

impl AsRef<str> for StrapName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl StrapName {
    pub fn parse(s: String) -> Result<StrapName, String> {
        if s.trim().is_empty() {
            Err(format!("{} is not a valid project name - empty string", s))
        } else {
            Ok(Self(s))
        }
    }
}

#[cfg(test)]
pub mod tests {
    use claim::assert_err;

    use crate::args::StrapName;

    #[test]
    fn whitespace_only_names_are_rejected() {
        let name = " ".to_string();
        assert_err!(StrapName::parse(name));
    }

    #[test]
    fn empty_string_is_rejected() {
        let name = "".to_string();
        assert_err!(StrapName::parse(name));
    }
}
