// fn main () {
//     for n in 1..101 {
//         if n % 15 == 0 {
//             println!("fizzbuzz");
//         } else if n % 5 == 0 {
//             println!("buzz");
//         } else if n % 3 == 0 {
//             println!("fizz");
//         } else {
//             println!("{}", n);
//         }
//     }
// }

// fn main () {
//     for n in 1..=100 {
//         if n % 15 == 0 {
//             println!("fizzbuzz");
//         } else if n % 5 == 0 {
//             println!("buzz");
//         } else if n % 3 == 0 {
//             println!("fizz");
//         } else {
//             println!("{}", n);
//         }
//     }
// }

// fn main() {
//     for n in 1..=100 {
//         let can_div_by_five = n % 5 == 0;
//         let can_div_by_three = n % 3 == 0;

//         if can_div_by_five && can_div_by_three {
//             println!("fizzbuzz");
//         } else if can_div_by_five {
//             println!("buzz");
//         } else if can_div_by_three {
//             println!("fizz");
//         } else {
//             println!("{}", n)
//         }
//     }
// }

fn main() {
    for n in 1..=100 {
        let can_div_by_five = n % 5 == 0;
        let can_div_by_three = n % 3 == 0;

        match (can_div_by_five, can_div_by_three) {
            (true, true) => println!("fizzbuzz"),
            (true, false) => println!("buzz"),
            (false, true) => println!("fizz"),
            (false, false) => println!("{}", n),
        }
    }
}
