// TODO: Fix the compiler error in the `main` function without changing this function.
fn is_a_color_word(attempt: &str) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}

fn main() {
    let word = String::from("green"); // Don't change this line.

    if is_a_color_word(&word[..3]) {
        println!("{} is a color word I know!", &word[..3]);
    } else {
        println!("{} is not a color word I know.", &word[..3]);
    }
}
