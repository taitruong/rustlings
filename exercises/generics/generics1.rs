// This shopping list program isn't compiling!
// Use your knowledge of generics to fix it.

// Execute `rustlings hint generics1` or use the `hint` watch subcommand for a hint.


fn main() {
    // solution 1
    let mut shopping_list: Vec<&str> = Vec::new();
    shopping_list.push("milk");

    // solution 2
    // let mut shopping_list: Vec<String> = Vec::new();
    // shopping_list.push("milk".to_string());
}
