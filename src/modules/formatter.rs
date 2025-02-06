pub fn format_string(template: String, args: &[String]) -> String {
    let mut result = String::from(template);
    for (_i, arg) in args.iter().enumerate() {
        let placeholder = format!("{{}}");
        result = result.replacen(placeholder.as_str(), arg.as_str(), 1);
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