/// модуль для анализа текста
pub mod text_stats {
    use std::collections::{HashMap, HashSet};

    /// функция возвращает кол-во символов текста
    pub fn count_chars(text: &str) -> usize {
        text.chars().count()
    }

    /// функция возвращает кол-во слов в тексте
    pub fn count_words(text: &str) -> usize {
        if text.trim().is_empty() {
            0
        } else {
            text.split_whitespace().count()
        }
    }

    /// Zadacha 1: частотный словарь слов
    pub fn word_frequency(text: &str) -> HashMap<String, usize> {
        let mut freq = HashMap::new();
        for word in text.split_whitespace().map(|w| w.to_lowercase()) {
            *freq.entry(word).or_insert(0) += 1;
        }
        freq
    }

    /// Zadacha 2: уникальные слова
    pub fn unique_words(text: &str) -> HashSet<String> {
        text.split_whitespace()
            .map(|w| w.to_lowercase())
            .collect::<HashSet<_>>()
    }

    /// Zadacha 3: самое длинное слово
    pub fn longest_word(text: &str) -> Option<String> {
        text.split_whitespace()
            .map(|w| w.to_lowercase())
            .max_by_key(|w| w.chars().count())
    }

    /// Zadacha 4: топ N слов по частоте
    pub fn top_n_words(text: &str, n: usize) -> Vec<(String, usize)> {
        let mut freq: Vec<_> = word_frequency(text).into_iter().collect();
        freq.sort_by(|a, b| b.1.cmp(&a.1));
        freq.into_iter().take(n).collect()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        // Тесты count_chars
        #[test]
        fn test_count_chars_basic() {
            assert_eq!(count_chars("hello"), 5);
        }

        #[test]
        fn test_count_chars_unicode() {
            assert_eq!(count_chars("Привет"), 6);
        }

        #[test]
        fn test_count_chars_empty() {
            assert_eq!(count_chars(""), 0);
        }

        // Тесты count_words
        #[test]
        fn test_count_words_basic() {
            assert_eq!(count_words("hello world"), 2);
        }

        #[test]
        fn test_count_words_spaces() {
            assert_eq!(count_words("hello   world"), 2);
        }

        #[test]
        fn test_count_words_empty() {
            assert_eq!(count_words(""), 0);
        }

        // Тесты word_frequency
        #[test]
        fn test_word_frequency_case() {
            let freq = word_frequency("Hello hello HELLO");
            assert_eq!(freq.get("hello"), Some(&3));
        }

        #[test]
        fn test_word_frequency_normal() {
            let freq = word_frequency("cat dog cat");
            assert_eq!(freq.get("cat"), Some(&2));
            assert_eq!(freq.get("dog"), Some(&1));
        }

        // Тесты unique_words
        #[test]
        fn test_unique_words_normal() {
            let words = unique_words("hello world hello");
            assert_eq!(words.len(), 2);
        }

        #[test]
        fn test_unique_words_all_same() {
            let words = unique_words("hello hello hello");
            assert_eq!(words.len(), 1);
        }

        // Тесты longest_word
        #[test]
        fn test_longest_word_basic() {
            assert_eq!(longest_word("cat dog horse"), Some("horse".to_string()));
        }

        #[test]
        fn test_longest_word_unicode() {
            assert_eq!(longest_word("я люблю Rust"), Some("люблю".to_string()));
        }

        #[test]
        fn test_longest_word_empty() {
            assert_eq!(longest_word(""), None);
        }
    }
}