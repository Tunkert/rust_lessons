use rand::Rng;

fn main() { // main is a function bro
    say_hello(); // call say_hello function
    let seven_squared = square(7); // a statement
    println!("Seven squared is {seven_squared}.");
    student_info(1, 'A', "mathematics");
    let my_rando = rando(200);
    println!("My rando is {my_rando}.");
}

fn say_hello() {
    println!("Hay, bro.");
}

fn rando(upper_range: i32) -> i32 {
    let r = rand::thread_rng().gen_range(1..=upper_range);
    r
}

fn student_info(id: i32, grade: char, class: &str) {
    println!("Student #{id} currently has a grade of {grade} in {class}.");
}

fn square(num: i32) -> i32 {
    num * num
}

/*
parameters when defining a function
arguments when calling a function
 */