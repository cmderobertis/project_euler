use std::{
    iter::Map,
    time::{Duration, Instant},
};
use utility_scripts::{count_factors, factorize, is_prime, sum_array};
mod utility_scripts;

// configure current problem
pub const CURRENT_PROBLEM: usize = 12;

// all problems
const PROBLEMS: [fn() -> i64; CURRENT_PROBLEM] = [
    pe1, pe2, pe3, pe4, pe5, pe6, pe7, pe8, pe9, pe10, pe11, pe12,
];

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
    let mut factor_count2: i64;
    let mut factors: Vec<i64>;

    loop {
        triangle_number += i;

        if is_prime(triangle_number) {
            i += 1;
            continue;
        }

        //factors = factorize(triangle_number);
        factor_count = count_factors(triangle_number);
        //factor_count2 = factors.len() as i64;

        // if factor_count != factor_count2 {
        //     println!("Error: contradicting returns from factor functions!");
        //     println!("count_factors(): {factor_count}, factorize().len(): {factor_count2}");
        //     break;
        // }

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

// Work out the first ten digits of the sum of the following one-hundred 50-digit numbers.
pub fn pe13() -> i64 {
    let first_ten: i64;
    let nums: [[i64; 50]; 100];

    first_ten = 1;

    return first_ten;
}

// What is the sum of the digits of the number 2^1000?
pub fn pe17() -> i64 {
    let mut current_count: i64;
    let mut total_count: i64 = 0;

    let digit_map: [i64; 10] = [3, 3, 5, 4, 4, 3, 5, 5, 4, 3];
    let tens_map: [i64; 10] = [0, 0, 6, 6, 5, 5, 5, 7, 6, 6];
    let hundreds_map: [i64; 10] = [0,3, 3, 5, 4, 4, 3, 5, 5, 4];

    for num in 1..1001 {
        current_count = 0;

        if num == 1000 {
            total_count += 11;
            continue;
        } else if num > 100 {
            current_count += hundreds_map.get(num / 100).expect("should be a number");
            current_count += 10; // "hundred and"
        }

        total_count += current_count;
    }

    return total_count;
}
