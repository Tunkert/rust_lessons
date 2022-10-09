fn main() {
    let sum = 7 + 2;

    let difference = 10 - 2;

    let product = 2 * 20;

    let quotient = 5 / 2;
    let quotient2 = 5.0 / 2.0;

    let remainder = 5 % 2;

    // let exponent = 2 ** 3; this will cause an error
    let exponent = i32::pow(2, 3);

    println!("The value of the sum is {sum}.");
    println!("The value of the difference is {difference}.");
    println!("The value of the product is {product}.");
    println!("The value of the quotient is {quotient}.");
    println!("The value of the 2nd quotient is {quotient2}");
    println!("The value of the remainder is {remainder}.");
    println!("The value of the exponent is {exponent}.");
}
