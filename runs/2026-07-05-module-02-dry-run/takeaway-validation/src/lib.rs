/// Reads directly through the borrowed slice - no clone at all, since nothing
/// here needs to outlive the borrow. See `../README.md` for the naive version
/// this replaced and the playbook steps that found the fix.
pub fn total_word_count(paragraphs: &[String]) -> usize {
    paragraphs.iter().map(|p| p.split_whitespace().count()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_words_across_paragraphs() {
        let paragraphs = vec![
            "the quick brown fox".to_string(),
            "jumps over".to_string(),
            "the lazy dog".to_string(),
        ];
        assert_eq!(total_word_count(&paragraphs), 9);
    }

    #[test]
    fn empty_slice_is_zero() {
        let paragraphs: Vec<String> = vec![];
        assert_eq!(total_word_count(&paragraphs), 0);
    }

    #[test]
    fn paragraphs_still_usable_after_the_call() {
        let paragraphs = vec!["one two".to_string()];
        let count = total_word_count(&paragraphs);
        assert_eq!(count, 2);
        assert_eq!(paragraphs.len(), 1);
    }
}
