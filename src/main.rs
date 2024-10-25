mod solution;

fn main() {
    assert_eq!(solution::character_from_ascii(65), 'A');
    assert_eq!(solution::character_from_ascii(97), 'a');
    assert_eq!(solution::character_from_ascii(48), '0');

    assert_eq!(solution::replace_dots(""), "");
    assert_eq!(solution::replace_dots("no dots"), "no dots");
    assert_eq!(solution::replace_dots("one.two.three"), "one-two-three");
    assert_eq!(solution::replace_dots("........"), "--------");
}
