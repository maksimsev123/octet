**count_lines**

---

Каждая строка определяется \n
Rust представляет инструмент lines()

**Реализация**

---

text_stats.rs
```rust
//// Возвращает кол-во строк в тексте
pub fn count_lines(text:: &str) -> usize {
    text.lines().count()
}
```

|вход|результат|
|:---|--------:|
|"Hello"|2|
|"Hello\nWorld"|3|
|"a\nb\nc"|3|
|""|0|


**count_words**

```rust
split(|c: char | !c.is_alphanumberic())
```

|вход|результат|обьяснение|
|:---|--------:|---------:|
|"Hello Wortld"|2|два слова|
|"Hello World"|2| *ПРивет* *Rust*|
|"2025 year"|2|числа тоже слова|
|"a-b??c"|3|Разделители игноряться|
|"..."|0|нет букв\цифр|
|""|0|Пустой текст|

подключение к main.rs

```rust
mod text_stats;

fn main() {
    let text = "Hello, wordId!\nRust - безопасный язык.";
    let lines = text_stats::text_stats::count_lines(text);
    let words = text_stats::text_stats::count_words(text);
    println!("Строк: {}", lines);
    println!("Слов: {}", words);
}
```
Упр 1
"One"
"One\nTwo"
"One\nTwo\nThree\n"
""

Упр 2
"Rust language"
"Hello, wordl!!!"
"Мне 18 лет"
"123 456 789"
"..."

Упр 3
Придумать 3 строки которые проверяют
Правильность обработки Unicode
Большое количество разделителей
emoji знаки 

Rust❤️2025
Результат 2 слова(Rust, 2025)