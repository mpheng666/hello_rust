// fn main() {
//     let y = {
//         let x = 3;
//         // note that there is no semicolon here
//         x + 1
//     };

//     println!("The value of y is: {}", y);
// }

// statement vs expression
// statement does not return but expression does
// expression does not include a semicolon at the end

// declare return type with ->
// fn five() -> i32 {
//     // can return explicitly
//     return 50;
//     // return implicitly at the end expression
//     5
// }

// fn main() {
//     let x = five();

//     println!("The value of x is: {}", x);
// }

fn main() {
    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
