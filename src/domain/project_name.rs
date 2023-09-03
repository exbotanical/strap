use clap::ArgMatches;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct ProjectName(String);

const FORBIDDEN_CHARS: [char; 9] = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];

impl ProjectName {
    pub fn parse(s: String) -> Result<Self, String> {
        if s.trim().is_empty() {
            return Err(format!("{} is not a valid project name - empty string", s));
        }

        if s.graphemes(true).count() > 256 {
            return Err(format!("{} is not a valid project name - too long", s));
        }

        if s.chars().any(|g| FORBIDDEN_CHARS.contains(&g)) {
            return Err(format!("{} is not a valid project name - invalid char", s));
        }

        Ok(Self(s))
    }
}

impl AsRef<str> for ProjectName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl TryFrom<(&str, &ArgMatches)> for ProjectName {
    type Error = String;

    fn try_from(matches: (&str, &ArgMatches)) -> Result<Self, Self::Error> {
        let matched = matches
            .1
            .get_one::<String>("project_name")
            .ok_or("project_name not specified but was required")?;

        Ok(Self(matched.into()))
    }
}

#[cfg(test)]
mod tests {
    use claim::{assert_err, assert_ok};

    use crate::domain::ProjectName;

    #[test]
    fn a_256_grapheme_long_name_is_valid() {
        let name = "a".repeat(256);
        assert_ok!(ProjectName::parse(name));
    }

    #[test]
    fn a_name_longer_than_256_graphemes_is_rejected() {
        let name = "a".repeat(257);
        assert_err!(ProjectName::parse(name));
    }

    #[test]
    fn whitespace_only_names_are_rejected() {
        let name = " ".to_string();
        assert_err!(ProjectName::parse(name));
    }

    #[test]
    fn empty_string_is_rejected() {
        let name = "".to_string();
        assert_err!(ProjectName::parse(name));
    }

    #[test]
    fn names_containing_an_invalid_character_are_rejected() {
        for name in &['/', '(', ')', '"', '<', '>', '\\', '{', '}'] {
            let name = name.to_string();
            assert_err!(ProjectName::parse(name));
        }
    }

    #[test]
    fn a_valid_name_is_parsed_successfully() {
        let name = "SomeName".to_string();
        assert_ok!(ProjectName::parse(name));
    }
}
