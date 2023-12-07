#[derive(Debug)]
struct RangeMap {
    dest_range_start: u64,
    source_range_start: u64,
    range_len: u64,
}

impl From<&str> for RangeMap {
    fn from(value: &str) -> Self {
        let parsed_vals: Vec<_> = value
            .split_whitespace()
            .map(|v| v.parse().unwrap())
            .collect();
        let (dest_range_start, source_range_start, range_len) =
            (parsed_vals[0], parsed_vals[1], parsed_vals[2]);
        RangeMap {
            dest_range_start,
            source_range_start,
            range_len,
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct Mapping {
    name: String,
    ranges: Vec<RangeMap>,
}

impl From<&str> for Mapping {
    fn from(value: &str) -> Self {
        let mut lines = value.lines();
        let name = lines.next().unwrap().to_string();
        let ranges = lines.map(|l| l.into()).collect();
        Mapping { name, ranges }
    }
}

impl RangeMap {
    fn get_dest(&self, source: &u64) -> Option<u64> {
        if (self.source_range_start..=self.source_range_start + self.range_len).contains(source) {
            Some(self.dest_range_start + (source - self.source_range_start))
        } else {
            None
        }
    }
}

impl Mapping {
    fn get_dest(&self, source: u64) -> u64 {
        self.ranges
            .iter()
            .find_map(|rangemap| rangemap.get_dest(&source))
            .unwrap_or(source)
    }
}

fn main() {
    let mut data = include_str!("../day5/input.txt").split("\n\n");
    let seeds: Vec<u64> = data
        .next()
        .unwrap()
        .strip_prefix("seeds:")
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    // luckily the maps are in-order so we don't need to parse the names to get that.
    let mappings: Vec<Mapping> = data.map(|block| block.into()).collect();
    // big enough lol
    let mut lowest_loc = 1_000_000_000;
    for s in seeds.chunks(2).flat_map(|vals| vals[0]..=vals[0] + vals[1]) {
        let mut tracing = s;
        for m in &mappings {
            tracing = m.get_dest(tracing);
        }
        lowest_loc = lowest_loc.min(tracing);
    }
    println!("Day 5 result: {lowest_loc}");
}
