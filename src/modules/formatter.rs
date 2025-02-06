pub fn format_string(template: String, args: &[impl ToString]) -> String {
    let mut result = String::from(template);
    for arg in args {
        let placeholder = "{}";
        result = result.replacen(placeholder, &arg.to_string(), 1);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_string() {
        let template = "Hello, {}! Welcome to {}.";
        let args = ["Alice".to_string(), "Rust".to_string()];
        let formatted = format_string(template.to_string(), &args);
        assert_eq!(formatted, "Hello, Alice! Welcome to Rust.");
    }
}