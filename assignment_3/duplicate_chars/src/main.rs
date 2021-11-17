use log::*;
fn main() {
    env_logger::init();
    info!("Printing Duplicate Chars");
    let string: &str = "hello world";
    let length = string.len();
    let mut count: i32;
    let mut char_vec: Vec<char> = string.chars().collect();

    debug!("Printing the duplicate chars");
    for first_loop in 0..length {
        count = 1;
        for second_loop in first_loop + 1..length {
            if char_vec[first_loop] == char_vec[second_loop] && char_vec[first_loop] != ' ' {
                count += 1;
                char_vec[second_loop] = '0';
            }
        }
        if count > 1 && char_vec[first_loop] != '0' {
            debug!("{}", char_vec[first_loop]);
        }
    }
}
