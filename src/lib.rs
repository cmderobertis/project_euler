use std::time::{Duration, Instant};
mod utility_scripts;

// configure current problem
const CURRENT_PROBLEM: usize = 8;

// all problems
const PROBLEMS: [fn() -> i32; CURRENT_PROBLEM] = [pe1, pe2, pe3, pe4,pe5, pe6, pe7, pe8];

pub fn run_all() {
    PROBLEMS.iter().for_each(|problem| {
        run(problem);
    });
}

pub fn run(problem: &fn() -> i32) {
    let now: Instant = Instant::now();
    let time_delta: Duration;
    let return_value: i32 = problem();

    time_delta = now.elapsed();

    print!("{}", return_value);
    println!(" - {:?}", time_delta);
}

// Find the sum of all the multiples of 3 or 5 below 1000.
fn pe1() -> i32 {
    let mut sum: i32 = 0;
    let mut i: i32 = 1;

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
fn pe2() -> i32 {
    let mut array: [i32; 2] = [0, 1];
    let mut num: i32;
    let mut sum: i32 = 2;

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
fn pe3() -> i32 {
    let mut array: [i32; 2] = [0, 1];
    let mut num: i64 = 600851475143;
    let mut quotient: i64;
    let mut divisor: i64 = 2;
    let mut largest_prime_factor: i64;

    array[0] = 2;
    array[1] = 3;

    loop {
        quotient = num / divisor;
        println!("{num} / {divisor} = {quotient}");

        if quotient % 1 == 0 && num == quotient * divisor {
            print!("integer quotient: {} 😆 ", quotient);
            num = quotient;

            largest_prime_factor = divisor;

            println!("NEW LARGEST PRIME DETECTED: {} 🚨🚨", largest_prime_factor);

            // check break condition
            if largest_prime_factor > num {
                break;
            }
        } else {
            divisor += 1;
            println!("{} is not an integer...", quotient);
        }
    }

    return largest_prime_factor as i32;
}

// Find the largest palindrome made from the product of two 3-digit numbers.
fn pe4() -> i32 {
    let mut largest_palindrome: i32 = 1;
    let mut num1: i32 = 100;
    let mut num2: i32 = 100;
    let mut product: i32;
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
fn pe5() -> i32 {
    let mut smallest_dividend: i32 = 2520;
    let mut divisor: i32 = 11;

    loop {
        loop {
            if smallest_dividend % divisor == 0  {
                
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
fn pe6() -> i32 {
    let mut sum: i32 = 0;
    let mut sum_of_squares: i32 = 0;
    
    for i in 1..101 {
        sum += i;
        sum_of_squares += i*i;
    }

    return sum * sum - sum_of_squares;
}

// What is the 10,001st prime number?
fn pe7() -> i32 {
    let mut prime: i32 = 1;
    let mut index: i32 = 0;

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


fn pe8() -> i32 {
    let mut product: i32=0;
    let mut this_string: &str;
    let mut nums: Vec<i32>;
    let num_string: String = String::from("7316717653133062491922511967442657474235534919493496983520312774506326239578318016984801869478851843858615607891129494954595017379583319528532088055111254069874715852386305071569329096329522744304355766896648950445244523161731856403098711121722383113622298934233803081353362766142828064444866452387493035890729629049156044077239071381051585930796086670172427121883998797908792274921901699720888093776657273330010533678812202354218097512545405947522435258490771167055601360483958644670632441572215539753697817977846174064955149290862569321978468622482839722413756570560574902614079729686524145351004748216637048440319989000889524345065854122758866688116427171479924442928230863465674813919123162824586178664583591245665294765456828489128831426076900422421902267105562632111110937054421750694165896040807198403850962455444362981230987879927244284909188845801561660979191338754992005240636899125607176060588611646710940507754100225698315520005593572972571636269561882670428252483600823257530420752963450");

    for i in 0..1001 - 13 {
        this_string = &num_string[i..i+13];
        nums = this_string.bytes().into_iter().map(|b| b as i32 - 48).collect::<Vec<i32>>();
        product = utility_scripts::sum_array(nums)
    }

    return product;
}