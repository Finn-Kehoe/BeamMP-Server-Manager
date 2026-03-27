pub fn get_value_from_json(json: &String, key: String) -> String {
    let value_end_delims = r#""}]"#;

    let key_location: Vec<_> = json.match_indices(&key).map(|(i, _)| i ).collect();
    let start_location = key_location[0] + key.len() + 1;

    let mut value = String::new();
    let mut val_has_started = false;
    for chr in json[start_location..].chars() {
        if val_has_started == false {
            if chr == '"' {
                val_has_started = true;
                continue;
            }
        } else if val_has_started {
            if value_end_delims.contains(chr) {
                break;
            } else {
                value.push(chr);
            }
        }
    }

    value

}