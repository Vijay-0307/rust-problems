fn main() {
    println!("{}", is_even(11)); //{} - print a dynamic variable
}

fn is_even(num: i32) -> bool { //num: i32 are argument
    if num % 2 == 0 {
        return true;
    }
    return false;
}