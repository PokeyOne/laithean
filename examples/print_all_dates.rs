use laithean::{
    Date,
    month::{Month, MONTHS}
};

fn main() {
    let year = 2022;

    for m in MONTHS {
        for d in 0..(m.day_count(year)) {
            let date = Date::new(year, m, d);
            println!("{date}");
        }
    }
}
