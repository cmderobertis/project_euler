use std::array;
use std::time::Duration;
use std::time::Instant;
mod lib;

fn main() {
    lib::run_all();
}

fn is_prime(num: i32) -> bool {
    if num <= 1 {
        return false;
    }
    for i in 2..num {
        if num % i == 0 {
            return false;
        }
    }
    return true;
}

fn sum_array(array: [i32;13]) -> i32 {
    return array.iter().sum();
}

