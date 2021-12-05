use std::{fmt::Display, path::Path, str::FromStr};

use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Part {
    One,
    Two,
}

/// An error which can be returned when parsing a `Part` from a string.
#[derive(Debug)]
struct InvalidPartError;

impl FromStr for Part {
    type Err = InvalidPartError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "one" | "1" => Ok(Part::One),
            "two" | "2" => Ok(Part::Two),
            _ => Err(InvalidPartError),
        }
    }
}

impl Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Part::One => f.write_str("One"),
            Part::Two => f.write_str("Two"),
        }
    }
}

fn main() {
    let pattern = Regex::new(r"day(?P<day>\d+)_?(?:part_?(?P<part>1|2))?").unwrap();
    let mut solutions: Vec<_> = std::fs::read_dir("src/solutions")
        .expect("no solutions directory")
        .filter_map(Result::ok)
        .map(|entry| {
            entry
                .file_name()
                .to_string_lossy()
                .trim_end_matches(".rs")
                .to_owned()
        })
        .filter_map(|filename| {
            let captures = pattern.captures(&filename)?;
            // SAFETY: The regular expression ensures that these values may safely be parsed into their respective
            // types.
            let day: u32 = captures["day"].parse().unwrap();
            let part: Part = captures
                .name("part")
                .map_or("1", |m| m.as_str())
                .parse()
                .unwrap();

            Some((day, part))
        })
        .collect();

    solutions.sort();

    let arms: String = solutions
        .iter()
        .copied()
        .map(|(day, part)| {
            format!(
                "({0}, Part::{1}) => Some(Day::<{0}, {{ Part::{1} }}>::run(input)),",
                day, part
            )
        })
        .collect();

    let implemented: String = solutions
        .iter()
        .copied()
        .map(|(day, part)| format!("({0}, Part::{1}),", day, part))
        .collect();

    let out = std::env::var_os("OUT_DIR").unwrap();
    let destination = Path::new(&out).join("run.rs");

    std::fs::write(
        &destination,
        format!(
            "pub fn run<'a>(day: u8, part: Part, input: &'a str) -> Option<Box<dyn std::fmt::Display + 'a>> {{
                match (day, part) {{
                    {}
                    _ => None,
                }}
            }}
            
            pub const IMPLEMENTED: &'static [(u8, Part)] = &[{}];",
            arms,
            implemented,
        ),
    ).unwrap();

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=solutions/");
}
