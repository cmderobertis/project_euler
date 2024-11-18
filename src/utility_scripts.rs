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

pub fn sum_array(vector:Vec<i64>) -> i64 {
    return vector.iter().sum();
}

pub fn multiply_array(vector:Vec<i64>) -> i64 {
    return vector.iter().product();
}
