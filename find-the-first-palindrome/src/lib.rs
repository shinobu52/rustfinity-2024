pub fn find_first_palindrome(start: i32, end: i32) -> Option<i32> {
    let (start, end) = (std::cmp::min(start, end), std::cmp::max(start, end));

    for num in start..=end {
        let num_str = num.to_string();

        let num_chars: Vec<char> = num_str.chars().collect();
        let mut flag = true;

        for i in 0..(num_str.len() / 2) {
            if num_chars[i] != num_chars[num_chars.len() - 1 - i] {
                flag = false;
                break;
            }
        }

        if flag {
            return Some(num);
        }
    }

    None
}
