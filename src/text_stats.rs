/// модуль для анализа текста
pub mod text_stats {
    use std::collections::{HashMap, HashSet};

    /// функция возвращает кол-во символов текста
    pub fn count_chars(text: &str) -> usize {
        text.chars().count()
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
}