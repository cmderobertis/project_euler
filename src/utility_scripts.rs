use std::vec;


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

pub fn multiply_array(vector:Vec<i64>) -> i64 {
    return vector.iter().product();
}

pub fn sum_array(vector:Vec<i64>) -> i64 {
    return vector.iter().sum();
}

pub fn factorize(num: i64) -> Vec<i64> {
    let mut vector: Vec<i64> = vec![];
    let mut i: i64 = 1;

    if num <= 0 {
        return vector;
    } else if num == 1 {
        return vec![1];
    }

    // append evenly divisible integers
    loop {
        if i > num {
            break;
        }

        if num % i == 0 {
            vector.push(i);
        }
        
        i += 1;
    }

    return vector;
}

pub fn count_factors(num: i64) -> i64 {
    let mut count: i64 = 0;

    for i in 1..(num as f64).sqrt() as i64 + 1 {
        if num % i == 0 {
            count += 2;
        }
    }

    return count;
}
