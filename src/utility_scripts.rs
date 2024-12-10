
pub fn is_prime(num: i64) -> bool {
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

pub fn multiply_array(vector: Vec<i64>) -> i64 {
    return vector.iter().product();
}

pub fn large_sum(vectors: Vec<&Vec<i64>>) -> Vec<i64> {
    let mut answer: Vec<i64> = vec![];
    let mut column_sum: i64 = 0;
    let mut carry_the: i64;
    let mut row: usize;
    let mut column: usize;
    let mut digit: i64;
    let rows: usize = vectors.len();
    let columns: usize = vectors[0].len();



    for i in 0..columns {
        column = columns - (i + 1);

        for j in 0..rows {
            row = j;
            digit = vectors[row][column];
            
            column_sum += digit;
        }

        digit = column_sum % 10;
        carry_the = (column_sum - digit) / 10;
        answer.insert(0, digit);
        //println!("digit: {digit}| column_sum: {column_sum} carry_the: {carry_the}");
        column_sum = carry_the;

        if i == columns - 1 {
            if carry_the > 9 {
                answer.extend([carry_the / 10, carry_the % 10]);
            } else {
                answer.insert(0, carry_the);
            }
            
        }
    }

    let first_non_zero: usize = answer.iter().position(|&x| x != 0).unwrap_or(answer.len());
    answer.drain(0..first_non_zero);
    answer
}

pub fn sum_array(vector:Vec<i64>) -> i64 {
    return vector.iter().sum();
}

// pub fn factorize(num: i64) -> Vec<i64> {
//     let mut vector: Vec<i64> = vec![];
//     let mut i: i64 = 1;

//     if num <= 0 {
//         return vector;
//     } else if num == 1 {
//         return vec![1];
//     }

//     // append evenly divisible integers
//     loop {
//         if i > num {
//             break;
//         }

//         if num % i == 0 {
//             vector.push(i);
//         }
        
//         i += 1;
//     }

//     return vector;
// }

pub fn count_factors(num: i64) -> i64 {
    let mut count: i64 = 0;

    for i in 1..(num as f64).sqrt() as i64 + 1 {
        if num % i == 0 {
            count += 2;
        }
    }

    return count;
}
