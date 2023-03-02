pub fn is_armstrong_number(num: u32) -> bool {
    let digits = extract_digits_from_integer(num);
    let exponent: u32 = digits.len().try_into().unwrap();
    let mut sum: u32 = 0;

    for digit in digits {
        sum += digit.pow(exponent);
    }

    num == sum
}

fn extract_digits_from_integer(input_int: u32) -> Vec<u32> {
    let input_str = input_int.to_string();
    let mut digits = vec![];

    for c in input_str.chars() {
        if let Some(digit) = c.to_digit(10) {
            digits.push(digit as u32);
        }
    }

    digits
}
