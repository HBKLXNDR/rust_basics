// src/main.rs
use std::io;
use std::collections::HashMap;

// Similar to a JS object/class with methods
struct User {
    name: String,
    age: u32,
    // Similar to Map in JS, but typed
    tags: HashMap<String, String>,
    active: bool,
}

// Implementing methods (like methods on a JS class)
impl User {
    // Constructor (like a constructor in JS)
    fn new(name: &str, age: u32) -> Self {
        let mut tags = HashMap::new();
        tags.insert(String::from("role"), String::from("user"));

        User {
            name: String::from(name),
            age,
            tags,
            active: true,
        }
    }

    // Method (like a class method in JS)
    fn describe(&self) -> String {
        format!("{} is {} years old", self.name, self.age)
    }

    // Mutable method (changes state, similar to this.property = value in JS)
    fn set_active(&mut self, status: bool) {
        self.active = status;
    }

    // Return a borrowed reference (similar to returning an object in JS)
    fn get_tags(&self) -> &HashMap<String, String> {
        &self.tags
    }
}

// Enum - similar to union types in TypeScript
enum MessageType {
    Text(String),
    Number(i32),
    Boolean(bool),
}

// Function to process our enum (like a function handling different types in JS)
fn process_message(msg: MessageType) {
    // Pattern matching (similar to switch but more powerful)
    match msg {
        MessageType::Text(text) => println!("Received text: {}", text),
        MessageType::Number(num) => println!("Received number: {}", num),
        MessageType::Boolean(b) => println!("Received boolean: {}", b),
    }
}

fn main() {
    println!("Simple Rust App for JS Developers!");

    // Variables (like let/const in JS)
    let name = "Alice";  // Immutable (like const in JS)
    let mut age = 30;    // Mutable (like let in JS)
    age += 1;            // Can change mutable variables

    // String type (similar to JS strings but UTF-8 encoded)
    let greeting = String::from("Hello");
    let full_greeting = format!("{}, {}! You are {} years old.", greeting, name, age);
    println!("{}", full_greeting);

    // Arrays and Vectors (similar to arrays in JS)
    let fixed_array = [1, 2, 3, 4, 5];  // Fixed size
    let mut numbers = vec![1, 2, 3];    // Dynamic size (like JS array)
    numbers.push(4);                    // Just like array.push() in JS

    // Looping (similar to for...of in JS)
    println!("\nNumbers in the vector:");
    for num in &numbers {
        println!("- {}", num);
    }

    // Creating a struct instance (like creating an object in JS)
    let mut user = User::new("Bob", 25);

    // Using methods
    println!("\n{}", user.describe());

    // Modifying the struct
    user.set_active(false);
    println!("User active status: {}", user.active);

    // Accessing the HashMap
    println!("\nUser tags:");
    for (key, value) in user.get_tags() {
        println!("- {}: {}", key, value);
    }

    // Using our enum with different types
    println!("\nHandling different message types:");
    process_message(MessageType::Text(String::from("Hello, world!")));
    process_message(MessageType::Number(42));
    process_message(MessageType::Boolean(true));

    // Using closures (similar to arrow functions in JS)
    let add = |a: i32, b: i32| a + b;
    println!("\nResult from closure: {}", add(5, 7));

    // Error handling (similar to try/catch in JS but with Result type)
    println!("\nLet's get user input!");
    println!("Enter a number: ");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Result handling with match (similar to try/catch but more explicit)
    let number: Result<i32, _> = input.trim().parse();
    match number {
        Ok(n) => println!("You entered: {}. Double that is: {}", n, n * 2),
        Err(_) => println!("That's not a valid number!"),
    }

    println!("\nThanks for trying Rust!");
}