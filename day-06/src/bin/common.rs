pub fn split_lines(input: &str) -> Vec<&str> {
    input.split("\n").collect()
}

pub struct MaxRaceRecord {
    pub time: u128,
    pub distance: u128,
}

impl MaxRaceRecord {

    pub fn possible_ways_to_beat(&self) -> u128 {
        let mut spend_time = 0_u128;
        let mut speed = 0_u128;
        let mut counter = 0_u128;

        while spend_time < self.time {
            let calculate_distance = (self.time - spend_time) * speed;
            if self.distance < calculate_distance {
                counter += 1;
            }
            spend_time += 1;
            speed += 1;
        }
        return counter;
    }
}

pub fn read_input_to_max_record_list(input: Vec<&str>) -> Vec<MaxRaceRecord> {
    let time_input = input.iter().nth(0).unwrap()
        .strip_prefix("Time:").unwrap()
        .split(" ")
        .filter(|n| !n.is_empty())
        .collect::<Vec<_>>();
    let distance_input = input.iter().nth(1).unwrap()
        .strip_prefix("Distance:").unwrap()
        .split(" ")
        .filter(|n| !n.is_empty())
        .collect::<Vec<_>>();
    return time_input.iter()
        .zip(distance_input.iter())
        .map(|(time, distance)| {
            MaxRaceRecord {
                time: time.parse::<u128>().unwrap(),
                distance: distance.parse::<u128>().unwrap(),
            }
        })
        .collect::<Vec<_>>();
}

pub fn read_input_fix_kerning_record(input: Vec<&str>) -> MaxRaceRecord {
    let time = input.iter().nth(0).unwrap()
        .strip_prefix("Time:").unwrap()
        .split(" ")
        .filter(|n| !n.is_empty())
        .collect::<Vec<_>>()
        .join("");
    let distance = input.iter().nth(1).unwrap()
        .strip_prefix("Distance:").unwrap()
        .split(" ")
        .filter(|n| !n.is_empty())
        .collect::<Vec<_>>()
        .join("");
    return MaxRaceRecord {
        time: time.parse::<u128>().unwrap(),
        distance: distance.parse::<u128>().unwrap(),
    }
}