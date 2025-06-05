// Tuples cannot be iterated because they are heterogeneous collections,
// meaning each element can have a different type (e.g., (i32, &str, bool)).
// Rust requires iteration to operate on elements of the same type,
// so it does not implement the Iterator trait for tuples.
// This is different from arrays or vectors, which are homogeneous (all elements same type)
// and thus iterable.

fn main() {
    println!("--- Exercise 1: Tuple Declaration and Access ---");
    let person: (&str, i32, f64) = ("Alice", 30, 5.6);
    println!("Name: {}", person.0);
    println!("Age: {}", person.1);
    println!("Height: {}\n", person.2);

    println!("--- Exercise 2: Destructuring Tuples ---");
    let point = (10, 20);
    let (x, y) = point;
    println!("x = {}, y = {}\n", x, y);

    println!("--- Exercise 3: Tuple in Function Return ---");
    let user = get_user();
    println!("User: {}, ID: {}\n", user.0, user.1);

    println!("--- Exercise 4: Manual Iteration via Destructuring ---");
    let scores = (100, 90, 80);
    let (a, b, c) = scores;
    let arr = [a, b, c]; // Convert to array
    for val in arr.iter() {
        println!("Score: {}", val);
    }
    println!();

    println!("--- Exercise 5: Tuple of Mixed Types ---");
    let mixed = ("Rust", true, 3.14);
    println!("Language: {}", mixed.0);
    println!("IsAwesome: {}", mixed.1);
    println!("Version: {}\n", mixed.2);
}

fn get_user() -> (&'static str, i32) {
    ("Ramesh", 999)
}
