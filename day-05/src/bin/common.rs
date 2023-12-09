use std::ops::Range;

pub fn split_lines(input: &str) -> Vec<&str> {
    input.split("\n").collect()
}

#[derive(Debug, Eq, PartialEq)]
pub struct Layer {
    pub source: Range<u64>,
    pub destination: Range<u64>,
}

impl Layer {
    pub fn flip(&self) -> Layer {
        return Layer {
            source: self.destination.clone(),
            destination: self.source.clone(),
        };
    }

    pub fn transform(&self, value: u64) -> u64 {
        return self.destination.start + (value - self.source.start);
    }

    pub fn is_contains_in_source(&self, value: u64) -> bool {
        return self.source.contains(&value);
    }
}

pub fn parse_mapper_to_layer(input: &Vec<&str>) -> Vec<Vec<Layer>> {
    let mut cloned_input = input.clone();
    let result = cloned_input.drain(2..).collect::<Vec<_>>()
        .join("\n")
        .split("\n\n")
        .map(|s|
            s.split("\n")
                .collect::<Vec<_>>()
                .drain(1..)
                .into_iter()
                .map(|line| row_to_layer(&line))
                .collect::<Vec<_>>()
        )
        .collect::<Vec<_>>();
    return result;
}

fn row_to_layer(row: &str) -> Layer {
    let layer_result = row.split(" ")
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    return Layer {
        source: Range { start: layer_result[1], end: layer_result[1] + layer_result[2] },
        destination: Range { start: layer_result[0], end: layer_result[0] + layer_result[2] },
    };
}