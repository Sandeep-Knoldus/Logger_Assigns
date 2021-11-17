use log::*;
fn main() {
    env_logger::init();
    info!("Check Palindrome String");
    let string: &str = "racecar";
    let length = string.len();
    warn!("This loop never actually loops");
    for first_loop in 0..length {
        if string.as_bytes()[first_loop] != string.as_bytes()[length - first_loop - 1] {
            error!("Elements are not matching");
            debug!("Not Palindrome");
            break
        } else {
            debug!("Palindrome");
            break
        }
    }
}
