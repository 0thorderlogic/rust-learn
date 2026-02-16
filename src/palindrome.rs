pub fn palindrome(input: String) -> bool {
    let input: String = input.trim().to_lowercase();
    let chars: Vec<char> = input.chars().collect();
    let length = chars.len();
    let mut is_palindrome: bool = true;

    for i in 0..length / 2 {
        if chars[i] != chars[length - 1 - i] {
            is_palindrome = false;
            break;
        }
    }
    return is_palindrome;
}
