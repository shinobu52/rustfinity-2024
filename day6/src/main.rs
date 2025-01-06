// Write a function that returns the reference to the longer string
// without any new allocations
pub fn longer_wish<'a>(s1: &'a str, s2: &'a str) -> Option<&'a str> {
    // Your code here
    let trimmed_s1 = s1.trim();
    let trimmed_s2 = s2.trim();

    match trimmed_s1.chars().count().cmp(&trimmed_s2.chars().count()) {
        std::cmp::Ordering::Less => Some(trimmed_s2),
        std::cmp::Ordering::Greater => Some(trimmed_s1),
        std::cmp::Ordering::Equal => None,
    }
}
