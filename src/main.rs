use chrono::NaiveDate;

fn main() {
    let birthday = NaiveDate::from_ymd_opt(1993, 4, 12);
    println!("{birthday:?}");
    println!("{}", birthday.unwrap());

    let birthday2 = "1993-07-10";
    println!(
        "{:?}",
        birthday2
            .parse::<NaiveDate>()
            .expect("unable tp parse Navidate date from string")
    );
}
