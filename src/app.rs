use crate::prelude::Part;
use clap::{ArgSettings, Args, Parser, Subcommand};
use colored::Colorize;
use lazy_static::lazy_static;
use reqwest::Client;
use std::{any::Any, fmt::Display, time::Instant};
use tokio::runtime::Runtime;

lazy_static! {
    static ref IMPLEMENTED: Vec<Day> = crate::solution::IMPLEMENTED
        .iter()
        .copied()
        .map(|(day, part)| Day::new(day, part))
        .collect();
}

#[derive(Debug, Parser)]
#[clap(name = "aoc")]
pub struct App {
    #[clap(skip)]
    pub(crate) client: Client,
    #[clap(long, env = "AOC_SESSION", setting = ArgSettings::HideEnvValues)]
    pub(crate) session: String,
    #[clap(subcommand)]
    pub(crate) command: Command,
}

impl App {
    pub fn run() -> reqwest::Result<()> {
        let app = App::parse();

        match app.command {
            Command::Run(Run::All) => {
                println!("Running all solutions...");
                app.run_all()?;
            }
            Command::Run(Run::Latest) => match IMPLEMENTED.last().copied() {
                Some(day) => app.run_specific(day)?,
                None => println!("{}! No solutions exist.", "Failed".red().bold()),
            },
            Command::Run(Run::Specific(day)) => app.run_specific(day)?,
        }

        Ok(())
    }

    fn run_all(&self) -> reqwest::Result<()> {
        println!("Fetching inputs...");

        let runtime = Runtime::new().unwrap();
        let fetch_inputs = IMPLEMENTED.iter().copied().map(|day| self.fetch_input(day));
        let inputs = runtime.block_on(async {
            futures::future::join_all(fetch_inputs)
                .await
                .into_iter()
                .collect::<Result<Vec<_>, _>>()
        })?;

        for (&day, input) in IMPLEMENTED.iter().zip(inputs) {
            self.run_solution(day, input, true)
        }

        Ok(())
    }

    fn run_specific(&self, day: Day) -> reqwest::Result<()> {
        println!("{}: Fetching input...", day);

        let runtime = Runtime::new().unwrap();
        let input = runtime.block_on(async { self.fetch_input(day).await })?;
        self.run_solution(day, input, false);

        Ok(())
    }

    async fn fetch_input(&self, day: Day) -> reqwest::Result<String> {
        self.client
            .get(format!(
                "https://adventofcode.com/2021/day/{}/input",
                day.number
            ))
            .header("Cookie", format!("session={}", self.session))
            .send()
            .await
            .and_then(|response| response.error_for_status())?
            .text()
            .await
    }

    fn run_solution(&self, day: Day, input: String, suppress: bool) {
        if !IMPLEMENTED.contains(&day) {
            println!(
                "{}: {}! Solution does not exist",
                day,
                "Failed".red().bold(),
            );

            return;
        }

        if !suppress {
            println!("{}: Running solution...", day);
        }

        let now = Instant::now();

        // SAFETY: We've already checked that this is a valid solution.
        match _run(day, input).unwrap() {
            Ok(value) => {
                println!("{}: {}! Took {}ms", day, "Ok".green().bold(), now.elapsed().as_millis());
                println!("{}", value.to_string().italic());
            }
            Err(err) => {
                println!("{}: {} with message {}", day, "Failed".red().bold(), err);
            }
        }
    }
}

#[derive(Debug, Subcommand)]
pub enum Command {
    #[clap(subcommand)]
    Run(Run),
}

#[derive(Debug, Subcommand)]
/// Run solutions. You may choose to run all solutions, only the latest solution, or a specific solution.
pub enum Run {
    /// Run all solutions
    All,
    /// Run the latest solution
    Latest,
    /// Run a specific solution
    Specific(Day),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Args)]
pub struct Day {
    #[clap(short, long = "number", long = "day", name = "day")]
    pub(crate) number: u8,
    #[clap(long = "part", default_value = "1", parse(try_from_str = parse_part))]
    pub(crate) part: Part,
}

impl Day {
    pub fn new(number: u8, part: Part) -> Self {
        Self { number, part }
    }
}

impl Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            format!("Day {} part {}", self.number, self.part as u8).bold()
        )
    }
}

fn parse_part(value: &str) -> Result<Part, &'static str> {
    match value.to_lowercase().as_str() {
        "1" | "one" => Ok(Part::One),
        "2" | "two" => Ok(Part::Two),
        _ => Err("expected `1`, `one`, `2` or `two`"),
    }
}

fn _run(day: Day, input: String) -> Option<Result<String, PanicMessage>> {
    let run = move || {
        let input = input;
        let result = crate::solution::run(day.number, day.part, &input)?;

        Some(result.to_string())
    };

    let result = std::panic::catch_unwind(run)
        .map_err(|message| PanicMessage::new(message))
        .transpose();

    result
}

// The following is some unfortunate panic glue

struct PanicMessage {
    message: Box<dyn Any + Send + 'static>,
}

impl PanicMessage {
    fn new(message: Box<dyn Any + Send + 'static>) -> Self {
        Self { message }
    }
}

impl Display for PanicMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let downcast = self
            .message
            .downcast_ref::<&str>()
            .map(|&value| value)
            .or_else(|| self.message.downcast_ref::<String>().map(String::as_str));

        match downcast {
            Some(message) => write!(f, r#""{}""#, message),
            None => f.write_str("<message of unknown type>"),
        }
    }
}
