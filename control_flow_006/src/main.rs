fn main() {
    // create a variable for age
    let age = 18;

    // decide if they can go to the bar
    if age >= 21 {
        println!("You can enter the bar, bro!");
    } else if age >= 18 {
        println!("You can't enter the bar even though you're an adult, bro.");
    } else {
        println!("You're a minor, bro!");
    }

    // truthy stuff - works with booleans
    let is_student = false;

    if is_student {
        println!("You are a student!");
    } else {
        println!("You are not a student!");
    }

    let is_teacher = true;

    if is_teacher {
        println!("You are a teacher!");
    } else {
        println!("You are not a teacher!");
    }
/*
    if 1 {
        println!("True");
    } else {
        print!("False");
    }
    this code block will cause an error, rust explicitly needs a boolean
 */

    // ternary type shenanigans
    let pay_enough = if is_teacher { "does not make enough" } else { "does make enough" };
    println!("A teacher {pay_enough}.");
}
