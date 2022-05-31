// fn main() {
//     let s1 = String::from("hello");

//     // &reference to s1 but does not own it
//     let len = calculate_length(&s1);

//     println!("The length of '{}' is {}.", s1, len);
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// } // s out of scope here but it is ok bc it does not own it

// fn main() {
//     // let s = String::from("hello");
//     let mut s = String::from("hello");

//     // change(&s);
//     change(&mut s);
// }

// // fn change(some_string: &String) {
// //     some_string.push_str(", world");
// // }

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }


// // =========You can only pass one mutable reference to a particular piece of
// // data at a time
// [This code does not compile!] 
// let mut s = String::from("hello");

// let r1 = &mut s;
// let r2 = &mut s;

// println!("{}, {}", r1, r2);

// ============== condition for data racing ==============
// more than one pointers access the same data at same t
// at least one of the pointers is being used to write to the data
// no mechanism to synchronise access to the data

// fn main() {
//     let mut s = String::from("hello");

//     // {
//     //     let r1 = &mut s;
//     // } // r1 goes out of scope here, so we can make a new reference with no problems.

//     // let r2 = &mut s;

//     // let mut s = String::from("hello");

//     // let r1 = &s; // no problem
//     // let r2 = &s; // no problem
//     // let r3 = &mut s; // BIG PROBLEM

//     // println!("{}, {}, and {}", r1, r2, r3);

//     // let mut s = String::from("hello");

//     let r1 = &s; // no problem
//     let r2 = &s; // no problem
//     println!("{} and {}", r1, r2);
//     // variables r1 and r2 will not be used after this point

//     let r3 = &mut s; // no problem
//     println!("{}", r3);
//     println!("{}", r1);
// }

fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    // &s // we return a reference to the String, s
    s // soluion
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!

