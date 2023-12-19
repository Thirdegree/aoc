#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::{cmp::Ordering, collections::HashMap, ops::RangeInclusive};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
enum RuleResult {
    Accept,
    Reject,
    Workflow(String),
}

impl From<&str> for RuleResult {
    fn from(value: &str) -> Self {
        match value {
            "A" => Self::Accept,
            "R" => Self::Reject,
            v => Self::Workflow(v.to_string()),
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

#[allow(dead_code)]
impl Part {
    const fn rating(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

const CURLIES: &[char] = &['{', '}'];

#[allow(clippy::many_single_char_names, clippy::fallible_impl_from)]
impl From<&str> for Part {
    fn from(value: &str) -> Self {
        let value = value.trim_matches(CURLIES);
        let (mut x, mut m, mut a, mut s) = (None, None, None, None);
        for v in value.split(',') {
            let (k, v) = v.split_once('=').unwrap();
            match k {
                "x" => x = Some(v.parse().unwrap()),
                "m" => m = Some(v.parse().unwrap()),
                "a" => a = Some(v.parse().unwrap()),
                "s" => s = Some(v.parse().unwrap()),
                _ => unreachable!(),
            }
        }
        Self {
            x: x.unwrap(),
            m: m.unwrap(),
            a: a.unwrap(),
            s: s.unwrap(),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct RuleCond {
    field: String,
    op: Ordering,
    val: usize,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Rule {
    condition: Option<RuleCond>,
    result: RuleResult,
}

#[allow(dead_code)]
impl Rule {
    fn eval(&self, part: &Part) -> bool {
        self.condition.as_ref().map_or(true, |cond| {
            let v = match &cond.field[..] {
                "x" => part.x,
                "m" => part.m,
                "a" => part.a,
                "s" => part.s,
                _ => unreachable!(),
            };
            match cond.op {
                Ordering::Less => v < cond.val,
                Ordering::Greater => v > cond.val,
                Ordering::Equal => unreachable!(),
            }
        })
    }
}

#[allow(clippy::fallible_impl_from)]
impl From<&str> for Rule {
    fn from(value: &str) -> Self {
        if let Some((condition, result)) = value.split_once(':') {
            let (field, op, val) = if condition.contains('>') {
                let (v, n) = condition.split_once('>').unwrap();
                let v = v.to_string();
                let n = n.parse().unwrap();
                (v, Ordering::Greater, n)
            } else if condition.contains('<') {
                let (v, n) = condition.split_once('<').unwrap();
                let v = v.to_string();
                let n = n.parse().unwrap();
                (v, Ordering::Less, n)
            } else {
                unreachable!()
            };
            Self {
                condition: Some(RuleCond { field, op, val }),
                result: result.into(),
            }
        } else {
            Self {
                condition: None,
                result: value.into(),
            }
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

#[allow(dead_code)]
impl Workflow {
    fn check_part(&self, part: &Part) -> &RuleResult {
        for rule in &self.rules {
            if rule.eval(part) {
                return &rule.result;
            }
        }
        unreachable!()
    }
}

#[allow(clippy::fallible_impl_from)]
impl From<&str> for Workflow {
    fn from(value: &str) -> Self {
        let (name, rules) = value.split_once('{').unwrap();
        let rules = rules.strip_suffix('}').unwrap().split(',');
        Self {
            name: name.to_string(),
            rules: rules.map(Into::into).collect(),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct ValueRanges {
    x: RangeInclusive<usize>,
    s: RangeInclusive<usize>,
    m: RangeInclusive<usize>,
    a: RangeInclusive<usize>,
}

impl ValueRanges {
    fn num_cases(&self) -> usize {
        (self.x.end() - self.x.start() + 1)
            * (self.s.end() - self.s.start() + 1)
            * (self.m.end() - self.m.start() + 1)
            * (self.a.end() - self.a.start() + 1)
    }

    #[allow(clippy::range_minus_one)]
    fn split_on_restriction(&self, condition: &RuleCond) -> (Self, Self) {
        let mut matches = self.clone();
        let mut not_matches = self.clone();
        match (condition.op, &condition.field[..]) {
            (Ordering::Greater, "x") => {
                matches.x = condition.val + 1..=*self.x.end();
                not_matches.x = *self.x.start()..=condition.val;
            }
            (Ordering::Greater, "s") => {
                matches.s = condition.val + 1..=*self.s.end();
                not_matches.s = *self.s.start()..=condition.val;
            }
            (Ordering::Greater, "m") => {
                matches.m = condition.val + 1..=*self.m.end();
                not_matches.m = *self.m.start()..=condition.val;
            }
            (Ordering::Greater, "a") => {
                matches.a = condition.val + 1..=*self.a.end();
                not_matches.a = *self.a.start()..=condition.val;
            }

            (Ordering::Less, "x") => {
                not_matches.x = condition.val..=*self.x.end();
                matches.x = *self.x.start()..=condition.val - 1;
            }
            (Ordering::Less, "s") => {
                not_matches.s = condition.val..=*self.s.end();
                matches.s = *self.s.start()..=condition.val - 1;
            }
            (Ordering::Less, "m") => {
                not_matches.m = condition.val..=*self.m.end();
                matches.m = *self.m.start()..=condition.val - 1;
            }
            (Ordering::Less, "a") => {
                not_matches.a = condition.val..=*self.a.end();
                matches.a = *self.a.start()..=condition.val - 1;
            }

            (_, _) => unreachable!(),
        };
        (matches, not_matches)
    }
}

fn solve_part_2(
    workflows: &HashMap<String, Workflow>,
    next_workflow: &String,
    mut ranges: ValueRanges,
) -> usize {
    // Ok so, important fact that I needed to verify: for ANY workspace, it is mentioned in exactly
    // 1 other workspace's rules (except in, which is the entry point). SO we can say that any
    //   given workspace narrows the possible accepts (ValueRanges) by each rule, passing that
    //   acceptable ranges to the child workspace of that rule and using the unacceptable range for
    //   the remaining rules, for each rule. Then we can just add up the product of the 4 acceptable ranges for each
    //   workspace, eventually getting the total count of acceptable values.
    let mut found_accepts = 0;
    for rule in &workflows[next_workflow].rules {
        if let Some(cond) = &rule.condition {
            let (matches, not_matches) = ranges.split_on_restriction(cond);
            match &rule.result {
                RuleResult::Accept => found_accepts += matches.num_cases(),
                RuleResult::Reject => (),
                RuleResult::Workflow(next) => {
                    found_accepts += solve_part_2(workflows, next, matches);
                }
            }
            ranges = not_matches;
        } else {
            match &rule.result {
                RuleResult::Accept => found_accepts += ranges.num_cases(),
                RuleResult::Reject => (),
                RuleResult::Workflow(next) => {
                    found_accepts += solve_part_2(workflows, next, ranges.clone());
                }
            }
        }
    }
    found_accepts
}

fn main() {
    let (workflows, _parts) = aoc_helpers::include_data!(day19)
        .split_once("\n\n")
        .unwrap();

    let workflows = workflows
        .lines()
        .map(Into::into)
        .map(|w: Workflow| (w.name.clone(), w))
        .collect::<HashMap<String, _>>();
    println!(
        "Day 19 result: {}",
        solve_part_2(
            &workflows,
            &"in".to_string(),
            ValueRanges {
                x: 1..=4000,
                m: 1..=4000,
                s: 1..=4000,
                a: 1..=4000,
            }
        )
    );
}
