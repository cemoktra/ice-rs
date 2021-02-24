pub fn escape(input: &str) -> String {
    if input == "type" {
        String::from("r#type")
    } else if input == "Type" {
        String::from("r#Type")
    } else {
        String::from(input)
    }
}