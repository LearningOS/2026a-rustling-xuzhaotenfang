// generics1.rs
//
// This shopping list program isn't compiling! Use your knowledge of generics to
// fix it.
//
// Execute `rustlings hint generics1` or use the `hint` watch subcommand for a
// hint.


struct Item(String);

fn main() {
    let mut shopping_list: Vec<Item> = Vec::new();
    shopping_list.push(Item("milk".into()));
}
