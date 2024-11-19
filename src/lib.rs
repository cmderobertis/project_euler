use std::{iter::Product, time::{Duration, Instant}};
use utility_scripts::is_prime;
mod utility_scripts;

// configure current problem
pub const CURRENT_PROBLEM: usize = 11;

// all problems
const PROBLEMS: [fn() -> i64; CURRENT_PROBLEM] =
    [pe1, pe2, pe3, pe4, pe5, pe6, pe7, pe8, pe9, pe10, pe11];

pub fn run_current() {
    run(&PROBLEMS[PROBLEMS.len() - 1]);
}

pub fn run_all() {
    let mut count: i64 = 0;

    PROBLEMS.iter().for_each(|problem| {
        count += 1;
        print!("{}: ", count);
        run(problem);
    });
}

pub fn run(problem: &fn() -> i64) {
    let now: Instant = Instant::now();
    let time_delta: Duration;
    let return_value: i64 = problem();

    time_delta = now.elapsed();

    println!("{} ({:?})", return_value, time_delta);
}

// Find the sum of all the multiples of 3 or 5 below 1000.
pub fn pe1() -> i64 {
    let mut sum: i64 = 0;
    let mut i: i64 = 1;

    loop {
        i += 1;

        if i == 1000 {
            break;
        } else if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
    }

    return sum;
}

//By considering the terms in the Fibonacci sequence whose values do not exceed four million, find the sum of the even-valued terms.
pub fn pe2() -> i64 {
    let mut array: [i64; 2] = [0, 1];
    let mut num: i64;
    let mut sum: i64 = 2;

    array[0] = 2;
    array[1] = 3;

    loop {
        num = array[0] + array[1];

        if num >= 4000000 {
            break;
        } else if num % 2 == 0 {
            sum += num;
        }

        array[0] = array[1];
        array[1] = num;
    }

    return sum;
}

// What is the largest prime factor of the number 600851475143?
pub fn pe3() -> i64 {
    let mut array: [i64; 2] = [0, 1];
    let mut num: i64 = 600851475143;
    let mut quotient: i64;
    let mut divisor: i64 = 2;
    let mut largest_prime_factor: i64;

    array[0] = 2;
    array[1] = 3;

    loop {
        quotient = num / divisor;
        //println!("{num} / {divisor} = {quotient}");

        if quotient % 1 == 0 && num == quotient * divisor {
            //print!("integer quotient: {} 😆 ", quotient);
            num = quotient;

            largest_prime_factor = divisor;

            //println!("NEW LARGEST PRIME DETECTED: {} 🚨🚨", largest_prime_factor);

            // check break condition
            if largest_prime_factor > num {
                break;
            }
        } else {
            divisor += 1;
            //println!("{} is not an integer...", quotient);
        }
    }

    return largest_prime_factor as i64;
}

// Find the largest palindrome made from the product of two 3-digit numbers.
pub fn pe4() -> i64 {
    let mut largest_palindrome: i64 = 1;
    let mut num1: i64 = 100;
    let mut num2: i64 = 100;
    let mut product: i64;
    let mut string: String;

    loop {
        loop {
            product = num1 * num2;
            //println!("is {} a palindrome?",product);
            // is_palindrome?
            string = product.to_string();

            loop {
                if string.chars().next() == string.chars().last() {
                    string.drain(..1);
                    string.pop();
                    if string.len() == 0 {
                        // found it
                        if product > largest_palindrome {
                            largest_palindrome = product;
                            //println!("num1: {num1}, num2: {num2}, largest_palindrome: {largest_palindrome}");
                        }
                        break;
                    }
                } else {
                    break;
                }
            }

            num2 += 1;

            if num2 >= 1000 {
                break;
            }
        }

        num1 += 1;
        num2 = 100;

        if num1 >= 1000 {
            break;
        }
    }

    return largest_palindrome;
}

// What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
pub fn pe5() -> i64 {
    let mut smallest_dividend: i64 = 2520;
    let mut divisor: i64 = 11;

    loop {
        loop {
            if smallest_dividend % divisor == 0 {
                if divisor == 20 {
                    return smallest_dividend;
                } else {
                    divisor += 1;
                }
            } else {
                break;
            }
        }

        smallest_dividend += 1;
        divisor = 11;
    }
}

// Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.
pub fn pe6() -> i64 {
    let mut sum: i64 = 0;
    let mut sum_of_squares: i64 = 0;

    for i in 1..101 {
        sum += i;
        sum_of_squares += i * i;
    }

    return sum * sum - sum_of_squares;
}

// What is the 10,001st prime number?
pub fn pe7() -> i64 {
    let mut prime: i64 = 1;
    let mut index: i64 = 0;

    loop {
        if utility_scripts::is_prime(prime) {
            index += 1;
            if index == 10001 {
                break;
            }
        }

        prime += 1;
    }

    return prime;
}

pub fn pe8() -> i64 {
    let mut max_product: i64 = 0;
    let mut product: i64;
    let mut this_string: &str;
    let mut nums: Vec<i64>;
    let num_string: String = String::from("7316717653133062491922511967442657474235534919493496983520312774506326239578318016984801869478851843858615607891129494954595017379583319528532088055111254069874715852386305071569329096329522744304355766896648950445244523161731856403098711121722383113622298934233803081353362766142828064444866452387493035890729629049156044077239071381051585930796086670172427121883998797908792274921901699720888093776657273330010533678812202354218097512545405947522435258490771167055601360483958644670632441572215539753697817977846174064955149290862569321978468622482839722413756570560574902614079729686524145351004748216637048440319989000889524345065854122758866688116427171479924442928230863465674813919123162824586178664583591245665294765456828489128831426076900422421902267105562632111110937054421750694165896040807198403850962455444362981230987879927244284909188845801561660979191338754992005240636899125607176060588611646710940507754100225698315520005593572972571636269561882670428252483600823257530420752963450");

    for i in 0..1001 - 13 {
        this_string = &num_string[i..i + 13];
        nums = this_string
            .bytes()
            .into_iter()
            .map(|b| b as i64 - 48)
            .collect::<Vec<i64>>();
        product = utility_scripts::multiply_array(nums);
        //println!("{}: {}",this_string,product);
        if product > max_product {
            max_product = product;
        }
    }

    return max_product;
}

pub fn pe9() -> i64 {
    let mut product: i64 = 0;
    let mut remainder: i64;
    let perimeter: i64 = 1000;

    for i in 0..500 {
        for j in 0..413 {
            remainder = perimeter - i - j;

            if remainder.pow(2) + j.pow(2) == i.pow(2) {
                //println!("{i}^2 == {j}^2 + {remainder}^2");
                product = i * j * remainder;
            }
        }
    }

    return product;
}

// Find the sum of all the primes below two million.
pub fn pe10() -> i64 {
    let mut sum: i64 = 0;

    for i in 0..2000000 {
        if is_prime(i) {
            sum += i;
            println!("{i} is prime");
        }
    }

    return sum;
}

// What is the greatest product of four adjacent numbers in the same direction (up, down, left, right, or diagonally) in the 20x20 grid?
pub fn pe11() -> i64 {
    let product: i64;
    let max_found_manually: i64 = 13956768;
    let max_possible: i64 = 96059601;

    //Assumption: the tetrad with the largest sum must also have the largest product

    product = max_possible;
    return product;
}