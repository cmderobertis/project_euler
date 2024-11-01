pub fn is_prime(num: i32) -> bool {
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

pub fn sum_array(vector:Vec<i32>) -> i32 {
    return vector.iter().sum();
}
