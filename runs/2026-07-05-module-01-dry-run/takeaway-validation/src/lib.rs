/// Return the first line starting with `prefix`, or `None`.
///
/// Naive version, before applying the ownership-move-checklist Skill: borrows
/// `lines` and clones the matching line to return it, even though `lines` is
/// owned outright and never used again after this loop.
pub fn first_matching_line(lines: Vec<String>, prefix: &str) -> Option<String> {
    lines.into_iter().find(|line| line.starts_with(prefix))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_first_matching_line() {
        let lines = vec![
            "INFO starting up".to_string(),
            "ERROR disk full".to_string(),
            "ERROR retry failed".to_string(),
        ];
        assert_eq!(first_matching_line(lines, "ERROR"), Some("ERROR disk full".to_string()));
    }

    #[test]
    fn returns_none_when_no_match() {
        let lines = vec!["INFO ok".to_string()];
        assert_eq!(first_matching_line(lines, "ERROR"), None);
    }

    #[test]
    fn empty_input_returns_none() {
        assert_eq!(first_matching_line(vec![], "ERROR"), None);
    }
}
