use chrono::TimeDelta;

fn main() {
    let five_seconds = TimeDelta::new(5, 0);
    println!("{five_seconds:?}");

    let five_minutes = TimeDelta::minutes(5);
    println!("{:?}", five_minutes);

    let five_hours = TimeDelta::hours(5);
    println!("{five_hours:?}");

    let five_days = TimeDelta::days(5);
    println!("{five_days:?}");

    let five_weeks = TimeDelta::weeks(5);
    println!("{five_weeks:?}");

    println!("{}", five_weeks.num_days());

    let total_duration = five_weeks + five_days + five_hours + five_minutes;
    println!("total duration : {:?}", total_duration);
}
