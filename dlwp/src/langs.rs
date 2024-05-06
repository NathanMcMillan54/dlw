/// Checks if a ``String`` is human readable text along with other characters
pub fn is_human_readable_including(text: String, include: Vec<char>) -> bool {
    for c in text.chars() {
        if c.is_alphanumeric() == false {
            if include.contains(&c) == false {
                return false;
            }
        }
    }

    return true;
}

/// Checks if a ``String`` is human readable
pub fn is_human_readable(text: String) -> bool {
    is_human_readable_including(text, vec![])
}
