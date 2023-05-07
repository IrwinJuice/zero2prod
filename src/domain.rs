use unicode_segmentation::UnicodeSegmentation;

pub struct NewSubscriber {
    pub email: String,
    pub name: SubscriberName,
}

pub struct SubscriberName(String);

impl SubscriberName {
    pub fn parse(s: String) -> SubscriberName {
        if Self::is_valid_name(&s) {
            Self(s)
        } else {
            panic!("{} is not a valid subscriber name.", s)
        }
    }
    /// Return `true` if the input satisfies all our validation constraints
    /// on subscriber names, `false` otherwise.
    fn is_valid_name(s: &str) -> bool {
        let is_empty_or_whitespace = s.trim().is_empty();

        // A grapheme is defined by the Unicode standard as a "user-perceived"
        // character: `å` is a single grapheme, but it is composed of two characters
        // (`a` and `̊`).
        //
        // `graphemes` returns an iterator over the graphemes in the input `s`.
        // `true` specifies that we want to use the extended grapheme definition set,
        // the recommended one.
        let is_too_long = s.graphemes(true).count() > 256;
        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let contains_forbidden_characters = s.chars().any(|c| forbidden_characters.contains(&c));

        !(is_empty_or_whitespace || is_too_long || contains_forbidden_characters)
    }
}

impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
       &self.0
    }
}
