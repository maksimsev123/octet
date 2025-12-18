```rust
let mut freq = HashMap::new();
for word in words {
    *freq.entry(word).or_insert(0) += 1;
}
```

map filter fold collect

map - преобразует каждый элемент итератора по функции
filter - отбирает элементы, где предикат возвращает
fold - сворачивает коллекцию в одно значение с аккумулятором
collect - который собирает элементы в коллекцию

отличие между String и &str то, что String изменяемый, а &str не изменяемый

разбор слова по Unicode(chars)

#### Паттерны проектирования
----
* когда возвращать Result<T,E>
* когда использовать Option<T>
* как настроить функии без побочных эффектов
* разница между public и private функциями модуля

Result<T,E> мы используем для проверки ошибок и чтоб знать какая ошибка
Option нужен для того, чтобы значение могло отсутствовать
если нет I/O
pub видно только внутри модуля, а private видно снаружи

----
#### Zadacha 1
```rust
pub fn word_frequency(text: &str) -> HashMap<String, usize>
```

Ожидаемок поведение:

* разбить текст на слова
* привести слова к нижнему регистру
* исключить пустые строки
* составить частотный словарь

```rust
.split_whitespace()
.to.lowercase()
HashMap::entry()
```
----
#### Zadacha 2
```rust
pub fn unique_words(text: &str) -> HashMap<String>
```

collect::<HashSet<_>>():
либо пройти циклом
----
#### ZAdacha 3
```rust
pub fn unique_word(text: &str) -> HashSet<String>
```

Требования
* если слово есть -> вернуть самое длинное слово
* если текст пустой или слов нет -> вернуть None
```rust
Iterator::max_by_key
chars().count()
```
----
#### Zadacha 4

```rust
pub fn top_n_words(text: &str, n: uzise) -> Vec<(String, usize)>
.sort_by(|a, b| b.1.cmp(&a.1)
```

----
БУДЕТ ПРОВЕРЯТЬСЯ
корректность реализации трех обязательных функций

unwrap panic





