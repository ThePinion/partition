use std::collections::BTreeSet;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::str::FromStr;
use std::{
    io::{self, BufRead},
    path::PathBuf,
};

use partition::fft::FFT;

use clap::{Args, Parser, Subcommand, ValueEnum};
use partition::helpers::{dynamic_programing_partition, naive_sumset};
use partition::NTT;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[arg(short, long, value_name = "FILE")]
    output: Option<PathBuf>,
    #[command(subcommand)]
    subcommand: Comands,
}
#[derive(Subcommand)]
enum Comands {
    #[command(about = "Approximate the sumset of a set of integers")]
    Sumset {
        epsilon: f64,
        #[arg(short, long, value_name = "FILE")]
        input: Option<PathBuf>,
    },
    #[command(about = "Approximate the partition of a set of integers")]
    Partition {
        epsilon: f64,
        #[arg(short, long, value_name = "FILE")]
        input: Option<PathBuf>,
    },
    #[command(about = "Benchmark the program")]
    Benchmark(BenchmarkOptions),
    #[command(about = "Benchmark the naive approach")]
    NaiveBenchmark(NaiveBenchmarkOptions),
    #[command(about = "Benchmark the dynamic programing approach")]
    DynamicProgramingBenchmark(DynamicProgramingBenchmarkOptions),
}

#[derive(Args, Default)]
struct BenchmarkOptions {
    epsilon_range_start: u64,
    epsilon_range_end: u64,
    epsilon_range_step: u64,
    input_length_range_start: usize,
    input_length_range_end: usize,
    input_length_range_step: usize,
    convoluter: Convoluter,
    #[arg(short, long, default_value = "1")]
    repetitions: usize,
}

impl BenchmarkOptions {
    pub fn epsilon_range(&self) -> StepRange<u64> {
        StepRange::new(
            self.epsilon_range_start,
            self.epsilon_range_end,
            self.epsilon_range_step,
        )
    }
    pub fn input_length_range(&self) -> StepRange<usize> {
        StepRange::new(
            self.input_length_range_start,
            self.input_length_range_end,
            self.input_length_range_step,
        )
    }
}

#[derive(Args, Default)]
struct NaiveBenchmarkOptions {
    input_length_range_start: usize,
    input_length_range_end: usize,
    input_length_range_step: usize,
    #[arg(short, long, default_value = "1")]
    repetitions: usize,
}

impl NaiveBenchmarkOptions {
    pub fn input_length_range(&self) -> StepRange<usize> {
        StepRange::new(
            self.input_length_range_start,
            self.input_length_range_end,
            self.input_length_range_step,
        )
    }
}

#[derive(Args, Default)]
struct DynamicProgramingBenchmarkOptions {
    input_length_range_start: usize,
    input_length_range_end: usize,
    input_length_range_step: usize,
    max_value_range_start: usize,
    max_value_range_end: usize,
    max_value_range_step: usize,
    #[arg(short, long, default_value = "1")]
    repetitions: usize,
}

impl DynamicProgramingBenchmarkOptions {
    pub fn input_length_range(&self) -> StepRange<usize> {
        StepRange::new(
            self.input_length_range_start,
            self.input_length_range_end,
            self.input_length_range_step,
        )
    }
    pub fn max_value_range(&self) -> StepRange<usize> {
        StepRange::new(
            self.max_value_range_start,
            self.max_value_range_end,
            self.max_value_range_step,
        )
    }
}

struct StepRange<T> {
    current: T,
    end: T,
    step: T,
}

impl<T> StepRange<T> {
    fn new(start: T, end: T, step: T) -> Self {
        Self {
            current: start,
            end,
            step,
        }
    }
}

impl<T> Iterator for StepRange<T>
where
    T: std::ops::AddAssign + PartialOrd + Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current <= self.end {
            let result = self.current;
            self.current += self.step;
            Some(result)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, ValueEnum, Default)]
pub enum Convoluter {
    #[default]
    FFT,
    NTT,
}

impl Display for Convoluter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Convoluter::FFT => write!(f, "FFT"),
            Convoluter::NTT => write!(f, "NTT"),
        }
    }
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    let output = match &cli.subcommand {
        Comands::Sumset { epsilon, input } => sumset_subcommand(input, epsilon),
        Comands::Partition { epsilon, input } => partition_subcommand(input, epsilon),
        Comands::Benchmark(options) => benchmark_subcommand(options),
        Comands::NaiveBenchmark(options) => naive_benchmark_subcommand(options),
        Comands::DynamicProgramingBenchmark(options) => {
            dynamic_programing_benchmark_subcommand(options)
        }
    }?;
    write_result(&cli.output, output)?;
    Ok(())
}

fn partition_subcommand(input: &Option<PathBuf>, epsilon: &f64) -> Result<String, io::Error> {
    Ok(
        partition::approximate_partition::<FFT>(&parse_input_as_vec(buf_reader(input)?)?, *epsilon)
            .to_string(),
    )
}

fn sumset_subcommand(input: &Option<PathBuf>, epsilon: &f64) -> Result<String, io::Error> {
    Ok(
        partition::approximate_sumset::<FFT>(&parse_input_as_vec(buf_reader(input)?)?, *epsilon)
            .into_iter()
            .collect::<BTreeSet<_>>()
            .into_iter()
            .map(|x| format!("{x} "))
            .collect::<String>(),
    )
}

fn benchmark_subcommand(options: &BenchmarkOptions) -> Result<String, io::Error> {
    use std::fmt::Write;
    let mut results = vec![];
    for epsilon in options.epsilon_range() {
        let epsilon = 1f64 / epsilon as f64;
        for input_length in options.input_length_range() {
            let config = BenchmarkConfig {
                epsilon: epsilon,
                input_length,
                convoluter: options.convoluter,
                repetitions: options.repetitions,
            };
            let result = config.benchmark_single()?;
            results.push(result);
        }
    }
    let mut output = String::new();
    writeln!(output, "{}", BenchmarkResult::HEADER).unwrap();
    for result in results {
        writeln!(output, "{}", result.to_cs_row()).unwrap();
    }
    Ok(output)
}

fn naive_benchmark_subcommand(options: &NaiveBenchmarkOptions) -> Result<String, io::Error> {
    use std::fmt::Write;
    let mut results = vec![];
    for input_length in options.input_length_range() {
        let config = NaiveBenchmarkConfig {
            input_length,
            repetitions: options.repetitions,
        };
        let result = config.benchmark_single()?;
        results.push(result);
    }
    let mut output = String::new();
    writeln!(output, "{}", NaiveBenchmarkResult::HEADER).unwrap();
    for result in results {
        writeln!(output, "{}", result.to_cs_row()).unwrap();
    }
    Ok(output)
}

fn dynamic_programing_benchmark_subcommand(
    options: &DynamicProgramingBenchmarkOptions,
) -> Result<String, io::Error> {
    use std::fmt::Write;
    let mut results = vec![];
    for input_length in options.input_length_range() {
        for max_value in options.max_value_range() {
            let config = DynamicProgramingBenchmarkConfig {
                input_length,
                max_value: max_value as u64,
                repetitions: options.repetitions,
            };
            let result = config.benchmark_single()?;
            results.push(result);
        }
    }
    let mut output = String::new();
    writeln!(output, "{}", DynamicProgramingBenchmarkResult::HEADER).unwrap();
    for result in results {
        writeln!(output, "{}", result.to_cs_row()).unwrap();
    }
    Ok(output)
}

struct BenchmarkConfig {
    epsilon: f64,
    input_length: usize,
    convoluter: Convoluter,
    repetitions: usize,
}

impl BenchmarkConfig {
    pub fn result(&self, times: Vec<u128>) -> BenchmarkResult {
        BenchmarkResult {
            epsilon: self.epsilon,
            input_length: self.input_length,
            convoluter: self.convoluter,
            times,
        }
    }
    fn benchmark_single(self) -> Result<BenchmarkResult, io::Error> {
        let mut times = Vec::new();
        for _ in 0..self.repetitions {
            let input = (0..self.input_length)
                .map(|_| rand::random::<u16>())
                .collect::<Vec<_>>();
            let start = std::time::Instant::now();
            match self.convoluter {
                Convoluter::FFT => partition::approximate_sumset::<FFT>(&input, self.epsilon),
                Convoluter::NTT => partition::approximate_sumset::<NTT>(&input, self.epsilon),
            };
            times.push(start.elapsed().as_nanos());
        }
        Ok(self.result(times))
    }
}

struct BenchmarkResult {
    epsilon: f64,
    input_length: usize,
    convoluter: Convoluter,
    times: Vec<u128>,
}

impl BenchmarkResult {
    fn average_time(&self) -> f64 {
        self.times.iter().map(|x| *x as f64).sum::<f64>() / self.times.len() as f64
    }
    const HEADER: &'static str = "epsilon, input_length, convoluter, average_time";
    fn to_cs_row(&self) -> String {
        format!(
            "{}, {}, {}, {}",
            self.epsilon,
            self.input_length,
            self.convoluter,
            self.average_time()
        )
    }
}

struct NaiveBenchmarkConfig {
    input_length: usize,
    repetitions: usize,
}

impl NaiveBenchmarkConfig {
    pub fn result(&self, times: Vec<u128>) -> NaiveBenchmarkResult {
        NaiveBenchmarkResult {
            input_length: self.input_length,
            times,
        }
    }
    fn benchmark_single(self) -> Result<NaiveBenchmarkResult, io::Error> {
        let mut times = Vec::new();
        for _ in 0..self.repetitions {
            let input = (0..self.input_length)
                .map(|_| rand::random::<u16>())
                .collect::<Vec<_>>();
            let start = std::time::Instant::now();
            naive_sumset(&input.iter().copied().map(u64::from).collect::<Vec<_>>());
            times.push(start.elapsed().as_nanos());
        }
        Ok(self.result(times))
    }
}

struct NaiveBenchmarkResult {
    input_length: usize,
    times: Vec<u128>,
}

impl NaiveBenchmarkResult {
    fn average_time(&self) -> f64 {
        self.times.iter().map(|x| *x as f64).sum::<f64>() / self.times.len() as f64
    }
    const HEADER: &'static str = "input_length,  average_time";
    fn to_cs_row(&self) -> String {
        format!("{}, {}", self.input_length, self.average_time())
    }
}

struct DynamicProgramingBenchmarkConfig {
    input_length: usize,
    max_value: u64,
    repetitions: usize,
}

impl DynamicProgramingBenchmarkConfig {
    pub fn result(&self, times: Vec<u128>) -> DynamicProgramingBenchmarkResult {
        DynamicProgramingBenchmarkResult {
            input_length: self.input_length,
            max_value: self.max_value,
            times,
        }
    }
    fn benchmark_single(self) -> Result<DynamicProgramingBenchmarkResult, io::Error> {
        let mut times = Vec::new();
        for _ in 0..self.repetitions {
            let input = (0..self.input_length)
                .map(|_| rand::random::<u64>() % self.max_value + 1)
                .collect::<Vec<_>>();
            let start = std::time::Instant::now();
            dynamic_programing_partition(&input);
            times.push(start.elapsed().as_nanos());
        }
        Ok(self.result(times))
    }
}

struct DynamicProgramingBenchmarkResult {
    input_length: usize,
    max_value: u64,
    times: Vec<u128>,
}

impl DynamicProgramingBenchmarkResult {
    fn average_time(&self) -> f64 {
        self.times.iter().map(|x| *x as f64).sum::<f64>() / self.times.len() as f64
    }
    const HEADER: &'static str = "input_length, max_value,  average_time";
    fn to_cs_row(&self) -> String {
        format!(
            "{}, {}, {}",
            self.input_length,
            self.max_value,
            self.average_time()
        )
    }
}

fn buf_reader(input: &Option<PathBuf>) -> io::Result<Box<dyn BufRead>> {
    match input {
        Some(path) => {
            let file = std::fs::File::open(path)?;
            Ok(Box::new(io::BufReader::new(file)))
        }
        None => Ok(Box::new(io::BufReader::new(io::stdin()))),
    }
}

fn parse_input_as_vec<R: BufRead>(reader: R) -> io::Result<Vec<u16>> {
    let mut values = Vec::new();
    for line in reader.lines() {
        let line = line?;
        for word in line.split_whitespace() {
            match u16::from_str(word) {
                Ok(value) => values.push(value),
                Err(_) => eprintln!("Warning: Skipping invalid value '{}'", word),
            }
        }
    }
    Ok(values)
}

fn write_result(output: &Option<PathBuf>, result: String) -> Result<(), io::Error> {
    let mut writer: Box<dyn Write> = match output {
        Some(path) => Box::new(BufWriter::new(File::create(path)?)),
        None => Box::new(BufWriter::new(io::stdout())),
    };
    writeln!(writer, "{result}")?;
    Ok(())
}
