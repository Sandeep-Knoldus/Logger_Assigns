use log::*;
fn main() {
    env_logger::init();
    info!("Hello World");
    info!("{:?}", "Generating Fibonacci Series");
    let mut loop1 = 1;
    let mut first = 0;
    let mut second = 1;
    let mut result;

    while loop1 < 10 {
        debug!("{}", first);
        result = first + second;
        first = second;
        second = result;
        loop1 += 1;
    }
}