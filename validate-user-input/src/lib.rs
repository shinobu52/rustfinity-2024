pub fn validate_user(age: i32, email: &str) -> Result<(), String> {
    if age < 0 || age >= 120 {
        return Err("Invalid age".to_string());
    };

    if !email.contains("@") {
        return Err("Invalid email".to_string());
    }

    Ok(())
}
