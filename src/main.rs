mod text_stats;
use text_stats::text_stats::count_chars;

fn main() {
    let text = "Sevostyanov Maksim Pavlovich!";
    let chars = count_chars(text);
    println!("Количество символов: {}", chars);
}
