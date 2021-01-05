pub fn escape(input: &str) -> String {
    if input == "type" {
        String::from("r#type")
    } else {
        String::from(input)
    }
}