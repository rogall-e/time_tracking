pub fn parse_time(time: &str) -> (i32, i32) {
    let mut split = time.split(':');
    let hour = split.next().unwrap().parse().unwrap();
    let minutes = split.next().unwrap().parse().unwrap();
    (hour, minutes)
}

pub fn calc_endtime(hour: i32, minutes: i32) -> (i32, i32) {
    let mut hour_tmp = hour + 7;
    let mut minutes_tmp = minutes + 80;
    while minutes_tmp >= 60 {
        hour_tmp += 1;
        minutes_tmp -= 60;
    }
    (hour_tmp, minutes_tmp)
}
