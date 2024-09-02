use chrono::{Datelike, Timelike};
use rand::Rng;

pub fn get_structured_refference() -> String {
    // +++240/9024/87267+++
    // +++<year last 2 digits><month first digit>/<month second digit><day first digit><day second digit><hour of day recalcuated to 0-9>/<entropy 4 digit number>+++

    let now = chrono::Local::now();

    let year = now.year() % 100;
    let month_first_digit = now.month() / 10;
    let month_second_digit = now.month() % 10;
    let day_first_digit = now.day() / 10;
    let day_second_digit = now.day() % 10;
    let hour_recalculated = now.hour() % 9;

    let entropy = get_random_4_digit_number();

    format!(
        "+++{}{}/{}{}{}{}/{}+++",
        year,
        month_first_digit,
        month_second_digit,
        day_first_digit,
        day_second_digit,
        hour_recalculated,
        entropy
    )
}

fn get_random_4_digit_number() -> String {
    let mut rng = rand::thread_rng();
    let number: u32 = rng.gen_range(1000..9999);
    number.to_string()
}
