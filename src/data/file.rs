use super::time::Time;

pub fn read_time_from_csv() -> Vec<Time> {
    let rdr = csv::Reader::from_path("time.csv");
    if rdr.is_err() {
        return Vec::new();
    }
    let mut times = Vec::new();
    for result in rdr.unwrap().deserialize() {
        let time: Time = result.unwrap();
        times.push(time);
    }
    times
}

pub fn save_time_to_csv(times: Vec<Time>) {
    let mut wtr = csv::Writer::from_path("time.csv").unwrap();
    for time in times {
        wtr.serialize(time).unwrap();
    }
}
