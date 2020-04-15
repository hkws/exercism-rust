pub fn square_of_sum(n: u32) -> u32 {
    // u32であるため、(1/2)*n*(n+1)では正しく計算できない。
    // （1/2）= 0になってしまい、sumが0になる。
    let sum = n * (n + 1) / 2;
    sum * sum
}

pub fn sum_of_squares(n: u32) -> u32 {
    n * (n + 1) * (2 * n + 1) / 6
}

pub fn difference(n: u32) -> u32 {
    let sq_of_sum = square_of_sum(n);
    let sum_of_sq = sum_of_squares(n);
    sq_of_sum - sum_of_sq
}
