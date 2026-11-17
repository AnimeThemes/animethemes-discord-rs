pub mod slugs;

/// Joins an array of strings with a separator, using a last separator for the last element.
pub fn join_with_last_separator(array: &[String], separator: &str, last_separator: &str) -> String {
    match array {
        [] => String::new(),
        [only] => only.clone(),
        [rest @ .., last] => {
            format!("{}{}{}", rest.join(separator), last_separator, last)
        }
    }
}
