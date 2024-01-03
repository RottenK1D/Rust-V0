#[derive(Debug)]
struct Person(String, Option<u8>);

fn main() {
    let person = Person("John".to_string(), Some(30));

    match person {
        Person(name, Some(age)) => println!("{} is {} years old", name, age),
        Person(name, None) => println!("{}'s age is unknown", name),
    }
}
