pub fn dart_format(file_path: &str) {
    std::process::Command::new("dart").arg("format").arg(file_path).output().expect("failed to run dart format");
}

pub fn to_title_case(input: &str) -> String {
    let mut titlecase = String::new();
    let mut capitalize_next = true;

    for c in input.chars() {
        if c.is_alphabetic() {
            if capitalize_next {
                titlecase.push(c.to_ascii_uppercase());
                capitalize_next = false;
            } else {
                titlecase.push(c.to_ascii_lowercase());
            }
        } else {
            titlecase.push(c);
            capitalize_next = true;
        }
    }

    titlecase
}