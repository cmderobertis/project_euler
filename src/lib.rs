use std::{collections::HashMap, time::{Duration, Instant}};
use utility_scripts::{is_prime, count_factors};
mod utility_scripts;

// configure current problem
pub const PROBLEM_COUNT: usize = 16;

// all problems
const PROBLEMS: [fn() -> i64; PROBLEM_COUNT] =
    [pe1, pe2, pe3, pe4, pe5, pe6, pe7, pe8, pe9, pe10, pe11, pe12, pe13, pe14, pe15, pe17];

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
    const GRID: [[i64; 20]; 20] = [
        [
            08, 02, 22, 97, 38, 15, 00, 40, 00, 75, 04, 05, 07, 78, 52, 12, 50, 77, 91, 08,
        ],
        [
            49, 49, 99, 40, 17, 81, 18, 57, 60, 87, 17, 40, 98, 43, 69, 48, 04, 56, 62, 00,
        ],
        [
            81, 49, 31, 73, 55, 79, 14, 29, 93, 71, 40, 67, 53, 88, 30, 03, 49, 13, 36, 65,
        ],
        [
            52, 70, 95, 23, 04, 60, 11, 42, 69, 24, 68, 56, 01, 32, 56, 71, 37, 02, 36, 91,
        ],
        [
            22, 31, 16, 71, 51, 67, 63, 89, 41, 92, 36, 54, 22, 40, 40, 28, 66, 33, 13, 80,
        ],
        [
            24, 47, 32, 60, 99, 03, 45, 02, 44, 75, 33, 53, 78, 36, 84, 20, 35, 17, 12, 50,
        ],
        [
            32, 98, 81, 28, 64, 23, 67, 10, 26, 38, 40, 67, 59, 54, 70, 66, 18, 38, 64, 70,
        ],
        [
            67, 26, 20, 68, 02, 62, 12, 20, 95, 63, 94, 39, 63, 08, 40, 91, 66, 49, 94, 21,
        ],
        [
            24, 55, 58, 05, 66, 73, 99, 26, 97, 17, 78, 78, 96, 83, 14, 88, 34, 89, 63, 72,
        ],
        [
            21, 36, 23, 09, 75, 00, 76, 44, 20, 45, 35, 14, 00, 61, 33, 97, 34, 31, 33, 95,
        ],
        [
            78, 17, 53, 28, 22, 75, 31, 67, 15, 94, 03, 80, 04, 62, 16, 14, 09, 53, 56, 92,
        ],
        [
            16, 39, 05, 42, 96, 35, 31, 47, 55, 58, 88, 24, 00, 17, 54, 24, 36, 29, 85, 57,
        ],
        [
            86, 56, 00, 48, 35, 71, 89, 07, 05, 44, 44, 37, 44, 60, 21, 58, 51, 54, 17, 58,
        ],
        [
            19, 80, 81, 68, 05, 94, 47, 69, 28, 73, 92, 13, 86, 52, 17, 77, 04, 89, 55, 40,
        ],
        [
            04, 52, 08, 83, 97, 35, 99, 16, 07, 97, 57, 32, 16, 26, 26, 79, 33, 27, 98, 66,
        ],
        [
            88, 36, 68, 87, 57, 62, 20, 72, 03, 46, 33, 67, 46, 55, 12, 32, 63, 93, 53, 69,
        ],
        [
            04, 42, 16, 73, 38, 25, 39, 11, 24, 94, 72, 18, 08, 46, 29, 32, 40, 62, 76, 36,
        ],
        [
            20, 69, 36, 41, 72, 30, 23, 88, 34, 62, 99, 69, 82, 67, 59, 85, 74, 04, 36, 16,
        ],
        [
            20, 73, 35, 29, 78, 31, 90, 01, 74, 31, 49, 71, 48, 86, 81, 16, 23, 57, 05, 54,
        ],
        [
            01, 70, 54, 71, 83, 51, 54, 69, 16, 92, 33, 48, 61, 43, 52, 01, 89, 19, 67, 48,
        ],
    ];
    const MAX_FOUND: i64 = 51267216; //51 million

    let mut product: i64;
    let mut max_product: i64 = 34;
    let mut smallest_allowable_num: i64 = MAX_FOUND / 99 / 99 / 99 + 1;
    let mut arr: [i64; 4];

    fn check_arr(arr: [i64; 4], smallest_allowable_num: i64) -> bool {
        let mut pass: bool = true;

        for num in arr {
            if num < smallest_allowable_num {
                pass = false;
                break;
            }
        }

        return pass;
    }

    //Hypothesis: the tetrad with the largest sum will also have the largest product
    println!("{smallest_allowable_num}");
    for row in 0..20 {
        for column in 0..20 {
            //NORTH
            if row >= 3 {
                arr = [
                    GRID[row][column],
                    GRID[row - 1][column],
                    GRID[row - 2][column],
                    GRID[row - 3][column],
                ];

                if check_arr(arr, smallest_allowable_num) {
                    product = utility_scripts::multiply_array(Vec::<i64>::from(arr));

                    if product > max_product {
                        max_product = product;
                        smallest_allowable_num = max_product / 99 / 99 / 99 + 1;
                    }
                }
            }

            //NORTHEAST
            if row >= 3 && column < 17 {
                arr = [
                    GRID[row][column],
                    GRID[row - 1][column + 1],
                    GRID[row - 2][column + 2],
                    GRID[row - 3][column + 3],
                ];

                if check_arr(arr, smallest_allowable_num) {
                    product = utility_scripts::multiply_array(Vec::<i64>::from(arr));

                    if product > max_product {
                        max_product = product;
                        smallest_allowable_num = max_product / 99 / 99 / 99 + 1;
                    }
                }
            }

            //EAST
            if column < 17 {
                arr = [
                    GRID[row][column],
                    GRID[row][column + 1],
                    GRID[row][column + 2],
                    GRID[row][column + 3],
                ];

                if check_arr(arr, smallest_allowable_num) {
                    product = utility_scripts::multiply_array(Vec::<i64>::from(arr));

                    if product > max_product {
                        max_product = product;
                        smallest_allowable_num = max_product / 99 / 99 / 99 + 1;
                    }
                }
            }

            //SOUTHEAST
            if row < 17 && column < 17 {
                arr = [
                    GRID[row][column],
                    GRID[row + 1][column + 1],
                    GRID[row + 2][column + 2],
                    GRID[row + 3][column + 3],
                ];

                if check_arr(arr, smallest_allowable_num) {
                    product = utility_scripts::multiply_array(Vec::<i64>::from(arr));

                    if product > max_product {
                        max_product = product;
                        smallest_allowable_num = max_product / 99 / 99 / 99 + 1;
                    }
                }
            }

            //SOUTH
            if row < 17 {
                arr = [
                    GRID[row][column],
                    GRID[row + 1][column],
                    GRID[row + 2][column],
                    GRID[row + 3][column],
                ];

                if check_arr(arr, smallest_allowable_num) {
                    product = utility_scripts::multiply_array(Vec::<i64>::from(arr));

                    if product > max_product {
                        max_product = product;
                        smallest_allowable_num = max_product / 99 / 99 / 99 + 1;
                    }
                }
            }

            //SOUTHWEST
            if row < 17 && column >= 3 {
                arr = [
                    GRID[row][column],
                    GRID[row + 1][column - 1],
                    GRID[row + 2][column - 2],
                    GRID[row + 3][column - 3],
                ];

                if check_arr(arr, smallest_allowable_num) {
                    product = utility_scripts::multiply_array(Vec::<i64>::from(arr));

                    if product > max_product {
                        max_product = product;
                        smallest_allowable_num = max_product / 99 / 99 / 99 + 1;
                    }
                }
            }

            //WEST
            if column >= 3 {
                arr = [
                    GRID[row][column],
                    GRID[row][column - 1],
                    GRID[row][column - 2],
                    GRID[row][column - 3],
                ];

                if check_arr(arr, smallest_allowable_num) {
                    product = utility_scripts::multiply_array(Vec::<i64>::from(arr));

                    if product > max_product {
                        max_product = product;
                        smallest_allowable_num = max_product / 99 / 99 / 99 + 1;
                    }
                }
            }

            //NORTHWEST
            if row >= 3 && column >= 3 {
                arr = [
                    GRID[row][column],
                    GRID[row - 1][column - 1],
                    GRID[row - 2][column - 2],
                    GRID[row - 3][column - 3],
                ];

                if check_arr(arr, smallest_allowable_num) {
                    product = utility_scripts::multiply_array(Vec::<i64>::from(arr));

                    if product > max_product {
                        max_product = product;
                        smallest_allowable_num = max_product / 99 / 99 / 99 + 1;
                    }
                }
            }
        }
    }

    return max_product;
}

// What is the value of the first triangle number to have over five hundred divisors?
pub fn pe12() -> i64 {
    let mut triangle_number: i64 = 0;
    let mut i: i64 = 1;
    let mut max_factor_count: i64 = 0;
    let mut factor_count: i64;

    loop {
        triangle_number += i;

        if is_prime(triangle_number) {
            i += 1;
            continue;
        }

        factor_count = count_factors(triangle_number);

        if factor_count > max_factor_count {
            max_factor_count = factor_count;
        }
        println!("{triangle_number} is the {i}th triangle number. Factor count [max: {max_factor_count}, current: {factor_count}]");
        if factor_count > 500 {
            break;
        }

        i += 1;
    }

    return triangle_number;
}

// Work out the first ten digits of the sum of the following one-hundred 50-digit numbers (NUMS).
pub fn pe13() -> i64 {
    const NUMBER_OF_DIGITS: usize = 50;
    const NUMBER_OF_NUMS: usize = 100;
    const NUMS: [[i64; NUMBER_OF_DIGITS]; NUMBER_OF_NUMS] = [
        [3,7,1,0,7,2,8,7,5,3,3,9,0,2,1,0,2,7,9,8,7,9,7,9,9,8,2,2,0,8,3,7,5,9,0,2,4,6,5,1,0,1,3,5,7,4,0,2,5,0],
        [4,6,3,7,6,9,3,7,6,7,7,4,9,0,0,0,9,7,1,2,6,4,8,1,2,4,8,9,6,9,7,0,0,7,8,0,5,0,4,1,7,0,1,8,2,6,0,5,3,8],
        [7,4,3,2,4,9,8,6,1,9,9,5,2,4,7,4,1,0,5,9,4,7,4,2,3,3,3,0,9,5,1,3,0,5,8,1,2,3,7,2,6,6,1,7,3,0,9,6,2,9],
        [9,1,9,4,2,2,1,3,3,6,3,5,7,4,1,6,1,5,7,2,5,2,2,4,3,0,5,6,3,3,0,1,8,1,1,0,7,2,4,0,6,1,5,4,9,0,8,2,5,0],
        [2,3,0,6,7,5,8,8,2,0,7,5,3,9,3,4,6,1,7,1,1,7,1,9,8,0,3,1,0,4,2,1,0,4,7,5,1,3,7,7,8,0,6,3,2,4,6,6,7,6],
        [8,9,2,6,1,6,7,0,6,9,6,6,2,3,6,3,3,8,2,0,1,3,6,3,7,8,4,1,8,3,8,3,6,8,4,1,7,8,7,3,4,3,6,1,7,2,6,7,5,7],
        [2,8,1,1,2,8,7,9,8,1,2,8,4,9,9,7,9,4,0,8,0,6,5,4,8,1,9,3,1,5,9,2,6,2,1,6,9,1,2,7,5,8,8,9,8,3,2,7,3,8],
        [4,4,2,7,4,2,2,8,9,1,7,4,3,2,5,2,0,3,2,1,9,2,3,5,8,9,4,2,2,8,7,6,7,9,6,4,8,7,6,7,0,2,7,2,1,8,9,3,1,8],
        [4,7,4,5,1,4,4,5,7,3,6,0,0,1,3,0,6,4,3,9,0,9,1,1,6,7,2,1,6,8,5,6,8,4,4,5,8,8,7,1,1,6,0,3,1,5,3,2,7,6],
        [7,0,3,8,6,4,8,6,1,0,5,8,4,3,0,2,5,4,3,9,9,3,9,6,1,9,8,2,8,9,1,7,5,9,3,6,6,5,6,8,6,7,5,7,9,3,4,9,5,1],
        [6,2,1,7,6,4,5,7,1,4,1,8,5,6,5,6,0,6,2,9,5,0,2,1,5,7,2,2,3,1,9,6,5,8,6,7,5,5,0,7,9,3,2,4,1,9,3,3,3,1],
        [6,4,9,0,6,3,5,2,4,6,2,7,4,1,9,0,4,9,2,9,1,0,1,4,3,2,4,4,5,8,1,3,8,2,2,6,6,3,3,4,7,9,4,4,7,5,8,1,7,8],
        [9,2,5,7,5,8,6,7,7,1,8,3,3,7,2,1,7,6,6,1,9,6,3,7,5,1,5,9,0,5,7,9,2,3,9,7,2,8,2,4,5,5,9,8,8,3,8,4,0,7],
        [5,8,2,0,3,5,6,5,3,2,5,3,5,9,3,9,9,0,0,8,4,0,2,6,3,3,5,6,8,9,4,8,8,3,0,1,8,9,4,5,8,6,2,8,2,2,7,8,2,8],
        [8,0,1,8,1,1,9,9,3,8,4,8,2,6,2,8,2,0,1,4,2,7,8,1,9,4,1,3,9,9,4,0,5,6,7,5,8,7,1,5,1,1,7,0,0,9,4,3,9,0],
        [3,5,3,9,8,6,6,4,3,7,2,8,2,7,1,1,2,6,5,3,8,2,9,9,8,7,2,4,0,7,8,4,4,7,3,0,5,3,1,9,0,1,0,4,2,9,3,5,8,6],
        [8,6,5,1,5,5,0,6,0,0,6,2,9,5,8,6,4,8,6,1,5,3,2,0,7,5,2,7,3,3,7,1,9,5,9,1,9,1,4,2,0,5,1,7,2,5,5,8,2,9],
        [7,1,6,9,3,8,8,8,7,0,7,7,1,5,4,6,6,4,9,9,1,1,5,5,9,3,4,8,7,6,0,3,5,3,2,9,2,1,7,1,4,9,7,0,0,5,6,9,3,8],
        [5,4,3,7,0,0,7,0,5,7,6,8,2,6,6,8,4,6,2,4,6,2,1,4,9,5,6,5,0,0,7,6,4,7,1,7,8,7,2,9,4,4,3,8,3,7,7,6,0,4],
        [5,3,2,8,2,6,5,4,1,0,8,7,5,6,8,2,8,4,4,3,1,9,1,1,9,0,6,3,4,6,9,4,0,3,7,8,5,5,2,1,7,7,7,9,2,9,5,1,4,5],
        [3,6,1,2,3,2,7,2,5,2,5,0,0,0,2,9,6,0,7,1,0,7,5,0,8,2,5,6,3,8,1,5,6,5,6,7,1,0,8,8,5,2,5,8,3,5,0,7,2,1],
        [4,5,8,7,6,5,7,6,1,7,2,4,1,0,9,7,6,4,4,7,3,3,9,1,1,0,6,0,7,2,1,8,2,6,5,2,3,6,8,7,7,2,2,3,6,3,6,0,4,5],
        [1,7,4,2,3,7,0,6,9,0,5,8,5,1,8,6,0,6,6,0,4,4,8,2,0,7,6,2,1,2,0,9,8,1,3,2,8,7,8,6,0,7,3,3,9,6,9,4,1,2],
        [8,1,1,4,2,6,6,0,4,1,8,0,8,6,8,3,0,6,1,9,3,2,8,4,6,0,8,1,1,1,9,1,0,6,1,5,5,6,9,4,0,5,1,2,6,8,9,6,9,2],
        [5,1,9,3,4,3,2,5,4,5,1,7,2,8,3,8,8,6,4,1,9,1,8,0,4,7,0,4,9,2,9,3,2,1,5,0,5,8,6,4,2,5,6,3,0,4,9,4,8,3],
        [6,2,4,6,7,2,2,1,6,4,8,4,3,5,0,7,6,2,0,1,7,2,7,9,1,8,0,3,9,9,4,4,6,9,3,0,0,4,7,3,2,9,5,6,3,4,0,6,9,1],
        [1,5,7,3,2,4,4,4,3,8,6,9,0,8,1,2,5,7,9,4,5,1,4,0,8,9,0,5,7,7,0,6,2,2,9,4,2,9,1,9,7,1,0,7,9,2,8,2,0,9],
        [5,5,0,3,7,6,8,7,5,2,5,6,7,8,7,7,3,0,9,1,8,6,2,5,4,0,7,4,4,9,6,9,8,4,4,5,0,8,3,3,0,3,9,3,6,8,2,1,2,6],
        [1,8,3,3,6,3,8,4,8,2,5,3,3,0,1,5,4,6,8,6,1,9,6,1,2,4,3,4,8,7,6,7,6,8,1,2,9,7,5,3,4,3,7,5,9,4,6,5,1,5],
        [8,0,3,8,6,2,8,7,5,9,2,8,7,8,4,9,0,2,0,1,5,2,1,6,8,5,5,5,4,8,2,8,7,1,7,2,0,1,2,1,9,2,5,7,7,6,6,9,5,4],
        [7,8,1,8,2,8,3,3,7,5,7,9,9,3,1,0,3,6,1,4,7,4,0,3,5,6,8,5,6,4,4,9,0,9,5,5,2,7,0,9,7,8,6,4,7,9,7,5,8,1],
        [1,6,7,2,6,3,2,0,1,0,0,4,3,6,8,9,7,8,4,2,5,5,3,5,3,9,9,2,0,9,3,1,8,3,7,4,4,1,4,9,7,8,0,6,8,6,0,9,8,4],
        [4,8,4,0,3,0,9,8,1,2,9,0,7,7,7,9,1,7,9,9,0,8,8,2,1,8,7,9,5,3,2,7,3,6,4,4,7,5,6,7,5,5,9,0,8,4,8,0,3,0],
        [8,7,0,8,6,9,8,7,5,5,1,3,9,2,7,1,1,8,5,4,5,1,7,0,7,8,5,4,4,1,6,1,8,5,2,4,2,4,3,2,0,6,9,3,1,5,0,3,3,2],
        [5,9,9,5,9,4,0,6,8,9,5,7,5,6,5,3,6,7,8,2,1,0,7,0,7,4,9,2,6,9,6,6,5,3,7,6,7,6,3,2,6,2,3,5,4,4,7,2,1,0],
        [6,9,7,9,3,9,5,0,6,7,9,6,5,2,6,9,4,7,4,2,5,9,7,7,0,9,7,3,9,1,6,6,6,9,3,7,6,3,0,4,2,6,3,3,9,8,7,0,8,5],
        [4,1,0,5,2,6,8,4,7,0,8,2,9,9,0,8,5,2,1,1,3,9,9,4,2,7,3,6,5,7,3,4,1,1,6,1,8,2,7,6,0,3,1,5,0,0,1,2,7,1],
        [6,5,3,7,8,6,0,7,3,6,1,5,0,1,0,8,0,8,5,7,0,0,9,1,4,9,9,3,9,5,1,2,5,5,7,0,2,8,1,9,8,7,4,6,0,0,4,3,7,5],
        [3,5,8,2,9,0,3,5,3,1,7,4,3,4,7,1,7,3,2,6,9,3,2,1,2,3,5,7,8,1,5,4,9,8,2,6,2,9,7,4,2,5,5,2,7,3,7,3,0,7],
        [9,4,9,5,3,7,5,9,7,6,5,1,0,5,3,0,5,9,4,6,9,6,6,0,6,7,6,8,3,1,5,6,5,7,4,3,7,7,1,6,7,4,0,1,8,7,5,2,7,5],
        [8,8,9,0,2,8,0,2,5,7,1,7,3,3,2,2,9,6,1,9,1,7,6,6,6,8,7,1,3,8,1,9,9,3,1,8,1,1,0,4,8,7,7,0,1,9,0,2,7,1],
        [2,5,2,6,7,6,8,0,2,7,6,0,7,8,0,0,3,0,1,3,6,7,8,6,8,0,9,9,2,5,2,5,4,6,3,4,0,1,0,6,1,6,3,2,8,6,6,5,2,6],
        [3,6,2,7,0,2,1,8,5,4,0,4,9,7,7,0,5,5,8,5,6,2,9,9,4,6,5,8,0,6,3,6,2,3,7,9,9,3,1,4,0,7,4,6,2,5,5,9,6,2],
        [2,4,0,7,4,4,8,6,9,0,8,2,3,1,1,7,4,9,7,7,7,9,2,3,6,5,4,6,6,2,5,7,2,4,6,9,2,3,3,2,2,8,1,0,9,1,7,1,4,1],
        [9,1,4,3,0,2,8,8,1,9,7,1,0,3,2,8,8,5,9,7,8,0,6,6,6,9,7,6,0,8,9,2,9,3,8,6,3,8,2,8,5,0,2,5,3,3,3,4,0,3],
        [3,4,4,1,3,0,6,5,5,7,8,0,1,6,1,2,7,8,1,5,9,2,1,8,1,5,0,0,5,5,6,1,8,6,8,8,3,6,4,6,8,4,2,0,0,9,0,4,7,0],
        [2,3,0,5,3,0,8,1,1,7,2,8,1,6,4,3,0,4,8,7,6,2,3,7,9,1,9,6,9,8,4,2,4,8,7,2,5,5,0,3,6,6,3,8,7,8,4,5,8,3],
        [1,1,4,8,7,6,9,6,9,3,2,1,5,4,9,0,2,8,1,0,4,2,4,0,2,0,1,3,8,3,3,5,1,2,4,4,6,2,1,8,1,4,4,1,7,7,3,4,7,0],
        [6,3,7,8,3,2,9,9,4,9,0,6,3,6,2,5,9,6,6,6,4,9,8,5,8,7,6,1,8,2,2,1,2,2,5,2,2,5,5,1,2,4,8,6,7,6,4,5,3,3],
        [6,7,7,2,0,1,8,6,9,7,1,6,9,8,5,4,4,3,1,2,4,1,9,5,7,2,4,0,9,9,1,3,9,5,9,0,0,8,9,5,2,3,1,0,0,5,8,8,2,2],
        [9,5,5,4,8,2,5,5,3,0,0,2,6,3,5,2,0,7,8,1,5,3,2,2,9,6,7,9,6,2,4,9,4,8,1,6,4,1,9,5,3,8,6,8,2,1,8,7,7,4],
        [7,6,0,8,5,3,2,7,1,3,2,2,8,5,7,2,3,1,1,0,4,2,4,8,0,3,4,5,6,1,2,4,8,6,7,6,9,7,0,6,4,5,0,7,9,9,5,2,3,6],
        [3,7,7,7,4,2,4,2,5,3,5,4,1,1,2,9,1,6,8,4,2,7,6,8,6,5,5,3,8,9,2,6,2,0,5,0,2,4,9,1,0,3,2,6,5,7,2,9,6,7],
        [2,3,7,0,1,9,1,3,2,7,5,7,2,5,6,7,5,2,8,5,6,5,3,2,4,8,2,5,8,2,6,5,4,6,3,0,9,2,2,0,7,0,5,8,5,9,6,5,2,2],
        [2,9,7,9,8,8,6,0,2,7,2,2,5,8,3,3,1,9,1,3,1,2,6,3,7,5,1,4,7,3,4,1,9,9,4,8,8,9,5,3,4,7,6,5,7,4,5,5,0,1],
        [1,8,4,9,5,7,0,1,4,5,4,8,7,9,2,8,8,9,8,4,8,5,6,8,2,7,7,2,6,0,7,7,7,1,3,7,2,1,4,0,3,7,9,8,8,7,9,7,1,5],
        [3,8,2,9,8,2,0,3,7,8,3,0,3,1,4,7,3,5,2,7,7,2,1,5,8,0,3,4,8,1,4,4,5,1,3,4,9,1,3,7,3,2,2,6,6,5,1,3,8,1],
        [3,4,8,2,9,5,4,3,8,2,9,1,9,9,9,1,8,1,8,0,2,7,8,9,1,6,5,2,2,4,3,1,0,2,7,3,9,2,2,5,1,1,2,2,8,6,9,5,3,9],
        [4,0,9,5,7,9,5,3,0,6,6,4,0,5,2,3,2,6,3,2,5,3,8,0,4,4,1,0,0,0,5,9,6,5,4,9,3,9,1,5,9,8,7,9,5,9,3,6,3,5],
        [2,9,7,4,6,1,5,2,1,8,5,5,0,2,3,7,1,3,0,7,6,4,2,2,5,5,1,2,1,1,8,3,6,9,3,8,0,3,5,8,0,3,8,8,5,8,4,9,0,3],
        [4,1,6,9,8,1,1,6,2,2,2,0,7,2,9,7,7,1,8,6,1,5,8,2,3,6,6,7,8,4,2,4,6,8,9,1,5,7,9,9,3,5,3,2,9,6,1,9,2,2],
        [6,2,4,6,7,9,5,7,1,9,4,4,0,1,2,6,9,0,4,3,8,7,7,1,0,7,2,7,5,0,4,8,1,0,2,3,9,0,8,9,5,5,2,3,5,9,7,4,5,7],
        [2,3,1,8,9,7,0,6,7,7,2,5,4,7,9,1,5,0,6,1,5,0,5,5,0,4,9,5,3,9,2,2,9,7,9,5,3,0,9,0,1,1,2,9,9,6,7,5,1,9],
        [8,6,1,8,8,0,8,8,2,2,5,8,7,5,3,1,4,5,2,9,5,8,4,0,9,9,2,5,1,2,0,3,8,2,9,0,0,9,4,0,7,7,7,0,7,7,5,6,7,2],
        [1,1,3,0,6,7,3,9,7,0,8,3,0,4,7,2,4,4,8,3,8,1,6,5,3,3,8,7,3,5,0,2,3,4,0,8,4,5,6,4,7,0,5,8,0,7,7,3,0,8],
        [8,2,9,5,9,1,7,4,7,6,7,1,4,0,3,6,3,1,9,8,0,0,8,1,8,7,1,2,9,0,1,1,8,7,5,4,9,1,3,1,0,5,4,7,1,2,6,5,8,1],
        [9,7,6,2,3,3,3,1,0,4,4,8,1,8,3,8,6,2,6,9,5,1,5,4,5,6,3,3,4,9,2,6,3,6,6,5,7,2,8,9,7,5,6,3,4,0,0,5,0,0],
        [4,2,8,4,6,2,8,0,1,8,3,5,1,7,0,7,0,5,2,7,8,3,1,8,3,9,4,2,5,8,8,2,1,4,5,5,2,1,2,2,7,2,5,1,2,5,0,3,2,7],
        [5,5,1,2,1,6,0,3,5,4,6,9,8,1,2,0,0,5,8,1,7,6,2,1,6,5,2,1,2,8,2,7,6,5,2,7,5,1,6,9,1,2,9,6,8,9,7,7,8,9],
        [3,2,2,3,8,1,9,5,7,3,4,3,2,9,3,3,9,9,4,6,4,3,7,5,0,1,9,0,7,8,3,6,9,4,5,7,6,5,8,8,3,3,5,2,3,9,9,8,8,6],
        [7,5,5,0,6,1,6,4,9,6,5,1,8,4,7,7,5,1,8,0,7,3,8,1,6,8,8,3,7,8,6,1,0,9,1,5,2,7,3,5,7,9,2,9,7,0,1,3,3,7],
        [6,2,1,7,7,8,4,2,7,5,2,1,9,2,6,2,3,4,0,1,9,4,2,3,9,9,6,3,9,1,6,8,0,4,4,9,8,3,9,9,3,1,7,3,3,1,2,7,3,1],
        [3,2,9,2,4,1,8,5,7,0,7,1,4,7,3,4,9,5,6,6,9,1,6,6,7,4,6,8,7,6,3,4,6,6,0,9,1,5,0,3,5,9,1,4,6,7,7,5,0,4],
        [9,9,5,1,8,6,7,1,4,3,0,2,3,5,2,1,9,6,2,8,8,9,4,8,9,0,1,0,2,4,2,3,3,2,5,1,1,6,9,1,3,6,1,9,6,2,6,6,2,2],
        [7,3,2,6,7,4,6,0,8,0,0,5,9,1,5,4,7,4,7,1,8,3,0,7,9,8,3,9,2,8,6,8,5,3,5,2,0,6,9,4,6,9,4,4,5,4,0,7,2,4],
        [7,6,8,4,1,8,2,2,5,2,4,6,7,4,4,1,7,1,6,1,5,1,4,0,3,6,4,2,7,9,8,2,2,7,3,3,4,8,0,5,5,5,5,6,2,1,4,8,1,8],
        [9,7,1,4,2,6,1,7,9,1,0,3,4,2,5,9,8,6,4,7,2,0,4,5,1,6,8,9,3,9,8,9,4,2,2,1,7,9,8,2,6,0,8,8,0,7,6,8,5,2],
        [8,7,7,8,3,6,4,6,1,8,2,7,9,9,3,4,6,3,1,3,7,6,7,7,5,4,3,0,7,8,0,9,3,6,3,3,3,3,0,1,8,9,8,2,6,4,2,0,9,0],
        [1,0,8,4,8,8,0,2,5,2,1,6,7,4,6,7,0,8,8,3,2,1,5,1,2,0,1,8,5,8,8,3,5,4,3,2,2,3,8,1,2,8,7,6,9,5,2,7,8,6],
        [7,1,3,2,9,6,1,2,4,7,4,7,8,2,4,6,4,5,3,8,6,3,6,9,9,3,0,0,9,0,4,9,3,1,0,3,6,3,6,1,9,7,6,3,8,7,8,0,3,9],
        [6,2,1,8,4,0,7,3,5,7,2,3,9,9,7,9,4,2,2,3,4,0,6,2,3,5,3,9,3,8,0,8,3,3,9,6,5,1,3,2,7,4,0,8,0,1,1,1,1,6],
        [6,6,6,2,7,8,9,1,9,8,1,4,8,8,0,8,7,7,9,7,9,4,1,8,7,6,8,7,6,1,4,4,2,3,0,0,3,0,9,8,4,4,9,0,8,5,1,4,1,1],
        [6,0,6,6,1,8,2,6,2,9,3,6,8,2,8,3,6,7,6,4,7,4,4,7,7,9,2,3,9,1,8,0,3,3,5,1,1,0,9,8,9,0,6,9,7,9,0,7,1,4],
        [8,5,7,8,6,9,4,4,0,8,9,5,5,2,9,9,0,6,5,3,6,4,0,4,4,7,4,2,5,5,7,6,0,8,3,6,5,9,9,7,6,6,4,5,7,9,5,0,9,6],
        [6,6,0,2,4,3,9,6,4,0,9,9,0,5,3,8,9,6,0,7,1,2,0,1,9,8,2,1,9,9,7,6,0,4,7,5,9,9,4,9,0,1,9,7,2,3,0,2,9,7],
        [6,4,9,1,3,9,8,2,6,8,0,0,3,2,9,7,3,1,5,6,0,3,7,1,2,0,0,4,1,3,7,7,9,0,3,7,8,5,5,6,6,0,8,5,0,8,9,2,5,2],
        [1,6,7,3,0,9,3,9,3,1,9,8,7,2,7,5,0,2,7,5,4,6,8,9,0,6,9,0,3,7,0,7,5,3,9,4,1,3,0,4,2,6,5,2,3,1,5,0,1,1],
        [9,4,8,0,9,3,7,7,2,4,5,0,4,8,7,9,5,1,5,0,9,5,4,1,0,0,9,2,1,6,4,5,8,6,3,7,5,4,7,1,0,5,9,8,4,3,6,7,9,1],
        [7,8,6,3,9,1,6,7,0,2,1,1,8,7,4,9,2,4,3,1,9,9,5,7,0,0,6,4,1,9,1,7,9,6,9,7,7,7,5,9,9,0,2,8,3,0,0,6,9,9],
        [1,5,3,6,8,7,1,3,7,1,1,9,3,6,6,1,4,9,5,2,8,1,1,3,0,5,8,7,6,3,8,0,2,7,8,4,1,0,7,5,4,4,4,9,7,3,3,0,7,8],
        [4,0,7,8,9,9,2,3,1,1,5,5,3,5,5,6,2,5,6,1,1,4,2,3,2,2,4,2,3,2,5,5,0,3,3,6,8,5,4,4,2,4,8,8,9,1,7,3,5,3],
        [4,4,8,8,9,9,1,1,5,0,1,4,4,0,6,4,8,0,2,0,3,6,9,0,6,8,0,6,3,9,6,0,6,7,2,3,2,2,1,9,3,2,0,4,1,4,9,5,3,5],
        [4,1,5,0,3,1,2,8,8,8,0,3,3,9,5,3,6,0,5,3,2,9,9,3,4,0,3,6,8,0,0,6,9,7,7,7,1,0,6,5,0,5,6,6,6,3,1,9,5,4],
        [8,1,2,3,4,8,8,0,6,7,3,2,1,0,1,4,6,7,3,9,0,5,8,5,6,8,5,5,7,9,3,4,5,8,1,4,0,3,6,2,7,8,2,2,7,0,3,2,8,0],
        [8,2,6,1,6,5,7,0,7,7,3,9,4,8,3,2,7,5,9,2,2,3,2,8,4,5,9,4,1,7,0,6,5,2,5,0,9,4,5,1,2,3,2,5,2,3,0,6,0,8],
        [2,2,9,1,8,8,0,2,0,5,8,7,7,7,3,1,9,7,1,9,8,3,9,4,5,0,1,8,0,8,8,8,0,7,2,4,2,9,6,6,1,9,8,0,8,1,1,1,9,7],
        [7,7,1,5,8,5,4,2,5,0,2,0,1,6,5,4,5,0,9,0,4,1,3,2,4,5,8,0,9,7,8,6,8,8,2,7,7,8,9,4,8,7,2,1,8,5,9,6,1,7],
        [7,2,1,0,7,8,3,8,4,3,5,0,6,9,1,8,6,1,5,5,4,3,5,6,6,2,8,8,4,0,6,2,2,5,7,4,7,3,6,9,2,2,8,4,5,0,9,5,1,6],
        [2,0,8,4,9,6,0,3,9,8,0,1,3,4,0,0,1,7,2,3,9,3,0,6,7,1,6,6,6,8,2,3,5,5,5,2,4,5,2,5,2,8,0,4,6,0,9,7,2,2],
        [5,3,5,0,3,5,3,4,2,2,6,4,7,2,5,2,4,2,5,0,8,7,4,0,5,4,0,7,5,5,9,1,7,8,9,7,8,1,2,6,4,3,3,0,3,3,1,6,9,0]
    ];
    let first_ten: i64;
    let mut answer: Vec<i64> = vec![];
    let mut column_sum: i64 = 0;
    let mut carry_the: i64;
    let mut row: usize;
    let mut column: usize;
    let mut digit: i64;
    
    for i in 0..50 {
        column = 50 - (i + 1);

        for j in 0..100 {
            row = j;
            digit = NUMS[row][column];
            
            column_sum += digit;
        }

        digit = column_sum % 10;
        carry_the = (column_sum - digit) / 10;
        answer.insert(0, digit);
        println!("digit: {digit}| column_sum: {column_sum} carry_the: {carry_the}");
        column_sum = carry_the;
        
        if i == 49 {
            print!("{carry_the}");
        }
    }

    for num in answer {
        print!("{num}");
    }
    first_ten = 1;

    return first_ten;
}

// Which starting number, under one million, produces the longest chain in the Collatz sequence?
// if EVEN n -> n/2
// if ODD n -> 3n + 1
pub fn pe14() -> i64 {
    let mut starting_number: i64 = 2;
    let mut current_number: i64;
    let mut chain_log: HashMap<i64, i64> = HashMap::new();
    let mut chain_length: i64;
    let mut max_chain_length: i64 = 0;
    let mut longest_chain_starting_number: i64 = 0;

    loop {
        current_number = starting_number;
        chain_length = 1;
        
        loop {

            if current_number == 1 {
                // base case
                break;
            } else if chain_log.contains_key(&current_number) {
                // already logged
                chain_length += chain_log.get(&current_number).expect("not a number?");
                chain_log.insert(starting_number, chain_length);
                break;
            } else if current_number % 2 == 0 {
                // even (n/2)
                current_number = current_number / 2;
            } else {
                // odd (3n+1)
                current_number = 3 * current_number + 1;
            }

            chain_length += 1;
        }
        
        if chain_length > max_chain_length {
            max_chain_length = chain_length;
            longest_chain_starting_number = starting_number;
        }

        starting_number += 1;

        if starting_number == 1000000 {
            break;
        }
    }
    
    return longest_chain_starting_number;
}

// How many such routes are there through a 20x20 grid?
pub fn pe15() -> i64 {
    let memo: HashMap<[usize;2],i64> = HashMap::new();

    fn count_routes(row: usize, column: usize, memo: HashMap<[usize; 2], i64>) -> i64 {
        let mut memo: HashMap<[usize;2],i64> = memo.clone();
        let coords: [usize;2] = [row,column];
        let up_one: [usize;2] = [row - 1, column];
        let left_one: [usize;2] = [row, column - 1];
        println!("row: {row} | column: {column}");
        let mut routes: i64 = 0;

        if row == 0 {
            return 1;
        } else if column == 0 {
            return  1;
        } else {
            if memo.contains_key(&up_one) {
                routes += memo.get(&up_one).expect("should be a number");
            } else {
                routes += count_routes(row - 1, column, memo.clone());
            }
            if memo.contains_key(&left_one) {
                routes += memo.get(&left_one).expect("should be a number");
            } else {
                routes += count_routes(row, column - 1, memo.clone());
            }

            memo.insert(coords, routes);
            return routes;
        }
    }

    return count_routes(20, 20, memo);
}

// What is the sum of the digits of the number 2^1000?
//pub fn pe16() -> i64 {
//}

//If all the numbers from 1 to 1000 inclusive were written out in words, how many letters would be used?
pub fn pe17() -> i64 {
    let mut current_count: i64;
    let mut total_count: i64 = 0;

    let hundreds_map: [&str; 10] = ["", "onehundredand", "twohundredand", "threehundredand", "fourhundredand", "fivehundredand", "sixhundredand", "sevenhundredand", "eighthundredand", "ninehundredand"];
    let tens_map: [&str; 10] = ["", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety"];
    let digit_map: [&str; 10] = ["", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let teens_map: [&str; 10] = ["ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen"];

    let mut hundreds: usize;
    let mut tens: usize;
    let mut ones: usize;
    let mut hundreds_string: &str;
    let mut tens_string: &str;
    let mut ones_string: &str;
    let mut number: String;

    for num in 1..1001 {
        
        if num == 1000 {

            print!("onethousand");
            current_count = 11;

        } else {

            hundreds = num / 100;
            tens = (num - (hundreds * 100)) / 10;
            ones = num % 10;

            //println!("{hundreds} hundreds, {tens} tens, and {ones} ones");

            hundreds_string = hundreds_map.get(hundreds).expect("string");
            if tens == 1 {
                tens_string = teens_map.get(ones).expect("string");
                number = hundreds_string.to_owned() + tens_string;
            } else {
                tens_string = tens_map.get(tens).expect("string");
                ones_string = digit_map.get(ones).expect("string");
                number = hundreds_string.to_owned() + tens_string + ones_string;
            }
            print!("{number}");
            

            current_count = number.len() as i64;

            if num % 100 == 0 {
                current_count -= 3;
            }
        }

        total_count += current_count;
        println!(" current: {current_count} total: {total_count}");
    }

    return total_count;
}