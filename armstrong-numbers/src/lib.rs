pub fn is_armstrong_number(num: u32) -> bool {
    let mut cnt = 0;
    let numstr = num.to_string();
    let length: u32 = numstr.chars().count() as u32;
    for s in numstr.chars() {
        cnt += s.to_digit(10).unwrap().pow(length);
    }
    num == cnt
    // 模範解答
    // let num_str = num.to_string();
    // let num_len = num_str.len() as u32;

    // let sum = num_str.chars()
    //     .map(|c| c.to_digit(10).unwrap())
    //     .map(|d| d.pow(num_len))
    //     .sum();

    // num == sum

}
