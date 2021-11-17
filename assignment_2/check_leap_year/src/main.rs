use log::*;
fn main() {
    env_logger::init();
    info!("Check for Leap Year");
    let years_arr: [i32; 11] = [
        2000, 2001, 2002, 2003, 2004, 2005, 2006, 2007, 2008, 2009, 2010,
    ];
    let mut count = 0;
    for year in years_arr {
        if year % 4 == 0 {
            if year % 100 == 0 {
                if year % 400 == 0 {
                    count += 1;
                    debug!("{} is a Leap Year", year);
                }
            } else {
                count += 1;
                debug!("{} is a Leap Year", year);
            }
        }
    }
    debug!("Count of Leap Years is: {}", count);
}
