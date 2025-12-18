mod text_stats;
use text_stats::text_stats::count_chars;

fn main() {
    // Упражнение 1
    println!("One: {} строк", "One".lines().count());
    println!("One\\nTwo: {} строк", "One\nTwo".lines().count());
    println!("One\\nTwo\\nThree\\n: {} строк", "One\nTwo\nThree\n".lines().count());
    println!("\"\": {} строк", if "".is_empty() { 0 } else { "".lines().count() });
    
    // Упражнение 2
    println!("Rust language: {} слов", "Rust language".split(' ').count());
    println!("Hello, world!!!: {} слов", "Hello, world!!!".split(' ').count());
    println!("Мне 18 лет: {} слов", "Мне 18 лет".split(' ').count());
    println!("123 456 789: {} слов", "123 456 789".split(' ').count());
    println!("...: {} слов", "...".split(' ').count());
    
    // Упражнение 3
    println!("Rust❤️2025: {} слов", "Rust❤️2025".split(' ').count());
    println!("Привет,   мир!   😀🌍: {} слов", "Привет,   мир!   😀🌍".split(' ').filter(|s| !s.is_empty()).count());
    println!("a\\t\\t\\tb\\n\\n\\nc🚀🚀🚀: {} слов", "a\t\t\tb\n\n\nc🚀🚀🚀".split_ascii_whitespace().count());
}