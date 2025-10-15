pub fn is_armstrong_number(num: u32) -> bool {
    let digit_count = num.checked_ilog10().unwrap_or(0) + 1;

    let result: u32 = num.to_string()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|d| d.pow(digit_count))
        .sum();
    result == num
}
