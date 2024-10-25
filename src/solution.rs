use regex::Regex;

pub fn character_from_ascii(c: u8) -> char {
  c as char
}

pub fn replace_dots(s: &str) -> String {
  Regex::new(r"\.").unwrap().replace_all(s, "-").to_string()
}
