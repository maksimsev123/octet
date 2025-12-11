/// модуль для анализа текста
pub mod text_stats {
    /// функция возвращает кол-во символов текста
    pub fn count_chars(text: &str) -> usize {
        text.chars().count()
    }
}