use itertools::unfold;
// use std::io;
// use std::io::prelude::*;

// pub fn run() {
//     let mut fibonacci = unfold((1_u32, 1_u32), |state| {
//         let (ref mut x1, ref mut x2) = state;

//         // Attempt to get the next Fibonacci number
//         let next = x1.saturating_add(*x2);

//         // Shift left: ret <- x1 <- x2 <- next
//         let ret = *x1;
//         *x1 = *x2;
//         *x2 = next;

//         // If addition has saturated at the maximum, we are finished
//         if ret == *x1 && ret > 1 {
//             return None;
//         }

//         Some(ret)
//     });

//     let mut power_of_twos = unfold((0_u32, 2_u32), |state| {
//         let (ref mut curr, ref mut next) = state;

//         let ret = next.saturating_mul(2);
//         let new_next = if *curr == 0 { 2 } else { ret };
//         *curr = *next;
//         *next = new_next;

//         if ret == *curr && ret > 1 {
//             return None;
//         }

//         return Some(*next);
//     });

//     let fib_vector: Vec<u32> = fibonacci
//         .by_ref()
//         .take_while(|&num| num < 4000000)
//         .filter(|&num| num % 2 == 1)
//         .collect();
//     println!("{:?}", fib_vector);
//     println!("{:?}", fib_vector.iter().sum::<u32>());

//     let twos: Vec<u32> = power_of_twos.by_ref().take(150).collect();
//     println!("{:?}", twos);
// }

fn bits(num: u64) {
    println!("{:b}", num);
}

pub fn run() {
    println!("TEST");
    let mut num = 8;
    bits(num);
    // set bit to 1
    num |= 1 << 2;
    bits(num);
    // clear bit
    num &= !(1 << 2);
    bits(num);
}
