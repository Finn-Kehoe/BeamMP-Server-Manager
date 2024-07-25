use regex::Regex;

pub fn preprocess_json(json: String) -> String {
    // seperates each end of line where there is no comma and no end of object/array into two capture groups
    let check_missing_commas = Regex::new(r"(\x22)(\s+[^\s\]|\s\}])").unwrap();
    // matches instance where there is a comma after the end of an object/array
    let check_trailing_commas = Regex::new(r",(\s*[\}\]])").unwrap();

    // pieces together the two groups seperated by the regex and adds a comma between them
    let mut processed_json = check_missing_commas.replace_all(&json, "$1,$2");

    let temp_json = processed_json.clone();

    // replaces each full match with capture group (everything execept comma)
    processed_json = check_trailing_commas.replace_all(&temp_json, "$1");

    String::from(processed_json)
}