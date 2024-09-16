use regex::Regex;

pub fn preprocess_json(json: String) -> String {
    // seperates each end of line where there is no comma and no end of object/array into two capture groups
    let check_missing_commas = Regex::new(r"(\x22)(\s+[^\s\]|\s\}])").unwrap();
    // matches instance where there is a comma after the end of an object/array
    let check_trailing_commas = Regex::new(r",(\s*[\}\]])").unwrap();
    // matches instance where there is a comment (//) at the beginning of a line
    let check_comments = Regex::new(r"(?m)(^\s*)//").unwrap();

    // pieces together the two groups seperated by the regex and adds a comma between them
    let mut processed_json = check_missing_commas.replace_all(&json, "$1,$2");

    let temp_1 = processed_json.clone();

    // replaces each full match with capture group (everything execept comma)
    processed_json = check_trailing_commas.replace_all(&temp_1, "$1");

    let temp_2 = processed_json.clone();

    // replaces each full match with capture group (everything except comment)
    processed_json = check_comments.replace_all(&temp_2, "$1");

    String::from(processed_json)
}

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