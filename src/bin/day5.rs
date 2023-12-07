use std::ops::RangeInclusive;
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

type RangeResult = (
    Option<RangeInclusive<u64>>,
    Option<RangeInclusive<u64>>,
    Option<RangeInclusive<u64>>,
);

impl RangeMap {
    /// Possible cases here:
    /// 1. Source range entirely contains rangemap => 3 results; "before" range, "mapped" range,
    ///    "after" range
    /// 2. Source range is entirely contained by rangemap => 1 result; "mapped" range
    /// 3. Source range starts before rangemap, ends inside => 2 results; "before" range, "mapped"
    ///    range
    /// 4. Source range starts inside rangemap, ends after => 2 results; "mapped" range, "after"
    ///    range
    /// 5. Source range is entirely before rangemap => 1 result; "before" range
    /// 6. Source range is entirely after rangemap => 1 result; "after" range
    /// "before" and "after" ranges are always unchanged from source, but of course might be
    /// shorter
    /// "before" and "after" ranges need to be checked against other rangemaps
    fn get_dest_range(&self, source: &RangeInclusive<u64>) -> RangeResult {
        let self_source_range_end = self.source_range_start + self.range_len;
        if source.start() < &self.source_range_start && source.end() > &self_source_range_end {
            // case 1
            (
                Some(*source.start()..=self.source_range_start - 1),
                Some(self.dest_range_start..=self.dest_range_start + self.range_len), // whole dest range mapped
                Some(self_source_range_end + 1..=*source.end()),
            )
        } else if source.start() > &self.source_range_start && source.end() < &self_source_range_end
        {
            // case 2
            (
                None,
                Some(
                    self.dest_range_start + (source.start() - self.source_range_start)
                        ..=(self.dest_range_start + self.range_len)
                            - (self_source_range_end - source.end()),
                ),
                None,
            )
        } else if source.start() < &self.source_range_start
            && source.end() > &self.source_range_start
        {
            // case 3
            (
                Some(*source.start()..=self.source_range_start - 1),
                Some(
                    self.dest_range_start
                        ..=(self.dest_range_start + self.range_len)
                            - (self_source_range_end - source.end()),
                ),
                None,
            )
        } else if source.start() < &self_source_range_end && source.end() > &self_source_range_end {
            // case 4
            (
                None,
                Some(
                    self.dest_range_start + (source.start() - self.source_range_start)
                        ..=self.dest_range_start + self.range_len,
                ),
                Some(self_source_range_end + 1..=*source.end()),
            )
        } else if source.end() < &self.source_range_start {
            // cases 5
            (Some(source.clone()), None, None)
        } else {
            // case 6
            (None, None, Some(source.clone()))
        }
    }
}

impl Mapping {
    fn get_range_dests(&self, sources: Vec<RangeInclusive<u64>>) -> Vec<RangeInclusive<u64>> {
        let mut unprocessed = sources;
        let mut results = vec![];
        for range in &self.ranges {
            let mut to_add = vec![];
            for r in &unprocessed {
                let result = range.get_dest_range(r);
                if let Some(before) = result.0 {
                    to_add.push(before);
                }
                if let Some(mapped) = result.1 {
                    results.push(mapped);
                }
                if let Some(after) = result.2 {
                    to_add.push(after);
                }
            }
            unprocessed = to_add
        }
        results.append(&mut unprocessed);
        results
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
    for s_range in seeds.chunks(2).map(|vals| vals[0]..=vals[0] + vals[1]) {
        let mut tracing = vec![s_range];
        for m in &mappings {
            tracing = m.get_range_dests(tracing);
        }
        lowest_loc = lowest_loc.min(*tracing.iter().min_by_key(|&r| r.start()).unwrap().start());
    }
    println!("Day 5 result: {lowest_loc}");
}
