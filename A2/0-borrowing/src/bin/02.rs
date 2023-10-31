//! You can't change anything except adding or removing references.

fn main() {
    // declare the variable as mutable to be able to
    // borrow it mutable
    let mut data = "Rust is great!".to_string();

    // borrow immuntably data to get_char
    get_char(&data);

    // borrow it mutable
    string_uppercase(&mut data);
}

// Should not take ownership
// it only needs to borrow data, not own it
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
// this function needs a mutable reference (&mut String)
// instead of a mutable variable (mut data: String)
fn string_uppercase(data: &mut String) {
    // modify the mutable borrowed value
    *data = data.to_uppercase();

    println!("{}", data);
}
