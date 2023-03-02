pub fn is_armstrong_number(num: u32) -> bool {
    let digits = get_digits(num);
    let digit_count = digits.len();
    let mut sum = 0;

    for digit in digits {
        let value = 
    }

}

fn get_digits(num: u32) -> Vec<char> {
    let num_string = format!("{}", num);
    num_string.chars().collect()
}
