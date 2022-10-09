fn main() {
    // make a loop that loops 8 times
    // let mut i = 0;
    // loop {
    //     println!("I'm looping, bro.");
    //     i = i + 1;
    //     if i > 7 {
    //         break;
    //     }
    // }

    // make a loop that returns something
    // let mut index = 0;
    // let a_stupid_way_to_square_7 = loop {
    //     if index == 7 {
    //         break index * 7;
    //     }
    //     index += 1;
    // };
    // println!("A really stupid way to square 7 yields {a_stupid_way_to_square_7}.");

    // make a loop that prints numbers 1 through 10 without 7
    // break and continue are a little convoluted
    // let's try this way
    // let mut index = 0;
    // while index < 10 {
    //     index += 1;
    //     if index != 7 {
    //         println!("{}", index);
    //     }
    // }

    // loop through an array
    let combination = [1, 2, 3, 4, 5];
    let mut index = 0;
    while index < 5 {
        println!("{}", combination[index]);
        index += 1;
    }
}
