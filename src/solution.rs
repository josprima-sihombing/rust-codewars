use regex::Regex;

pub fn character_from_ascii(c: u8) -> char {
  c as char
}

pub fn replace_dots(s: &str) -> String {
  Regex::new(r"\.").unwrap().replace_all(s, "-").to_string()
}

pub fn are_you_playing_banjo(name: &str) -> String {
  let first_char = name.chars().nth(0);

  if first_char == Some('R') || first_char == Some('r') {
    return String::from(name.to_owned() + " plays banjo");
  }

  String::from(name.to_owned() + " does not play banjo")
}
