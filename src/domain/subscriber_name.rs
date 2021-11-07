use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)] // required for `claim` to provide nice error messages
pub struct SubscriberName(String);

impl SubscriberName {
    /// returns a `SubscriberName` instance if `s` satisfies all requirements for it,
    /// otherwise, it returns an error message - this is the only way to create an instance outside this module!
    pub fn parse(s: String) -> Result<SubscriberName, String> {
        let is_empty_or_whitespace = s.trim().is_empty();
        let is_too_long = s.graphemes(true).count() > 256;

        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let contains_forbidden_characters = s.chars().any(|g| forbidden_characters.contains(&g));

        if is_empty_or_whitespace {
            Err(format!("Subscriber names cannot be empty, but '{}' is.", s))
        } else if is_too_long {
            Err(format!(
                "Subscriber names can only be 256 characters or less, but '{}' is longer",
                s
            ))
        } else if contains_forbidden_characters {
            Err(format!(
                "Subscriber names cannot contain special characters, but '{}' does",
                s
            ))
        } else {
            Ok(Self(s))
        }

        /*else if is_empty_or_whitespace || is_too_long || contains_forbidden_characters {
        Err(format!("{} is not a valid subscriber name.", s))*/
    }
}

impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::SubscriberName;
    use claim::{assert_err, assert_ok};

    #[test]
    fn a_256_grapheme_long_name_is_valid() {
        let name = "a".repeat(256);
        assert_ok!(SubscriberName::parse(name));
    }

    #[test]
    fn a_name_longer_than_256_graphemes_is_rejected() {
        let name = "a".repeat(257);
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn whitespace_only_names_are_rejected() {
        let name = " ".to_string();
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn empty_string_is_rejected() {
        let name = "".to_string();
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn names_containing_an_invalid_character_are_rejected() {
        for name in &['/', '(', ')', '"', '<', '>', '\\', '{', '}'] {
            let name = name.to_string();
            assert_err!(SubscriberName::parse(name));
        }
    }

    #[test]
    fn a_valid_name_is_parsed_successfully() {
        let name = "Ursula Le Guin".to_string();
        assert_ok!(SubscriberName::parse(name));
    }
}
