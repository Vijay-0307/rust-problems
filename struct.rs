struct User {
    first_name: String,
    last_name : String,
    age: i32,
}

fn main() {
    let user1 = User {
        first_name: String::from("Vijay"),
        last_name: String::from("Sivaraman"),
        age: 22,
    };

    println!("The first name of the user is {}", user1.first_name);
    println!("The last name of the user is {}", user1.last_name);
    println!("The age of the user is {}", user1.age);
}