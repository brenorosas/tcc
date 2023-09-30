use regex::Regex;

pub fn is_valid_email(email: &str) -> bool {
    // Define a regular expression pattern for a basic email validation
    let re = Regex::new(r"^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$").unwrap();

    // Use the regular expression to check if the email matches the pattern
    re.is_match(email)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_email() {
        assert!(is_valid_email("teste@teste.com"));
        assert!(!is_valid_email("teste@teste"));
        assert!(!is_valid_email("teste.com"));
        assert!(!is_valid_email("teste@teste."));
    }
}
