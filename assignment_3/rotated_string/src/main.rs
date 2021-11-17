use log::*;
fn main() {
    env_logger::init();
    debug!("{}", is_rotated("abcd", "aaaa"));
}
fn is_rotated(string1: &str, string2: &str) -> i32 {
    if string1.is_empty() || string2.is_empty() {
        error!("String in empty");
        0
    } else if string1.len() != string2.len() {
        error!("Length of Strings are not same");
        0
    } else {
        let string_concat = format!("{}{}", string1, string2);
        debug!("{:?}", string_concat);
        if string_concat.contains(string2) {
            debug!("Strings are rotated");
            1
        } else {
            error!("Strings are not Rotated");
            0
        }
    }
}
