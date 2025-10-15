/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    // check symbols
    if !code.chars().all(|c| c.is_numeric() || c.is_whitespace()) {
        return false;
    }
    
    let digits: Vec<u32> = code.chars()
        .filter(|c| c.is_numeric())
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    
    if digits.len() <= 1 {
        return false;
    }
    
    let len = digits.len();
    let sum: u32 = digits.iter()
        .enumerate()
        .map(|(i, &d)| {
            if (len - i - 1) % 2 == 1 {
                let doubled = d * 2;
                if doubled > 9 { doubled - 9 } else { doubled }
            } else {
                d
            }
        })
        .sum();
    
    sum % 10 == 0
}
