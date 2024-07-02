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
/// Main entry point for the command-line interface, which parses the arguments and delegates to the appropriate subcommand.
struct Cli {
    /// Specifies the output file path. If not provided, the output will be printed to the standard output.
    #[arg(short, long, value_name = "FILE")]
    output: Option<PathBuf>,
    /// The subcommand to execute.
    #[command(subcommand)]
    subcommand: Comands,
}

#[derive(Subcommand)]

enum Comands {
    /// Approximates the partition of a set of integers using a specified approximation parameter.
    Partition {
        /// The approximation parameter.
        epsilon: f64,
        /// Specifies the input file path. If not provided, input is read from the standard input.
        /// The input file or stdin should contain a list of u16 integers separated by whitespace.
        #[arg(short, long, value_name = "FILE")]
        input: Option<PathBuf>,
    },
    /// Runs a benchmark with specified options.
    /// The benchmark is run on a randomly generated data for each combination of epsilon and input length specified.
    Benchmark(BenchmarkOptions),
    /// Runs a benchmark using the naive approach
    NaiveBenchmark(NaiveBenchmarkOptions),
    /// Runs a benchmark using the dynamic programming approach
    DynamicProgramingBenchmark(DynamicProgramingBenchmarkOptions),
}

#[derive(Args, Default)]
/// Options for benchmarking, including ranges for epsilon and input lengths, and the convoluter type.
struct BenchmarkOptions {
    /// The start of the inverse of epsilon values to use for the benchmark. Eg. start=2, end=6, step=2 will use epsilon values 1/2, 1/4, 1/6.
    epsilon_inverse_range_start: u64,
    /// The end of the range of the inverse of epsilon values.
    epsilon_inverse_range_end: u64,
    /// The step size for the above.
    epsilon_inverse_range_step: u64,
    /// The start range of input lengths to use for the benchmark. Eg. start=50, end=150, step=50 will use input lengths 50, 100, 150.
    input_length_range_start: usize,
    /// The end of the range of input lengths.
    input_length_range_end: usize,
    /// The step size for the above.
    input_length_range_step: usize,
    convoluter: Convoluter,
    /// Number of repetitions for each benchmark.
    #[arg(short, long, default_value = "1")]
    repetitions: usize,
}

impl BenchmarkOptions {
    pub fn epsilon_range(&self) -> StepRange<u64> {
        StepRange::new(
            self.epsilon_inverse_range_start,
            self.epsilon_inverse_range_end,
            self.epsilon_inverse_range_step,
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
/// Options for benchmarking the naive approach, including ranges for input lengths.
struct NaiveBenchmarkOptions {
    /// The start range of input lengths to use for the benchmark. Eg. start=50, end=150, step=50 will use input lengths 50, 100, 150.
    input_length_range_start: usize,
    /// The end of the range of input lengths.
    input_length_range_end: usize,
    /// The step size for the above.
    input_length_range_step: usize,
    /// Number of repetitions for each benchmark.
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
/// Options for benchmarking the dynamic programming approach, including ranges for input lengths and maximum values.
struct DynamicProgramingBenchmarkOptions {
    /// The start range of input lengths to use for the benchmark. Eg. start=50, end=150, step=50 will use input lengths 50, 100, 150.
    input_length_range_start: usize,
    /// The end of the range of input lengths.
    input_length_range_end: usize,
    /// The step size for the above.
    input_length_range_step: usize,
    /// Number of repetitions for each benchmark. This can be used to get more accurate results.
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
/// Enum representing the types of convoluters used in the benchmarking process.
pub enum Convoluter {
    #[default]
    /// Fast Fourier Transform
    FFT,
    /// Number Theoretic Transform
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
        let config = DynamicProgramingBenchmarkConfig {
            input_length,
            max_value: u16::MAX as u64,
            repetitions: options.repetitions,
        };
        let result = config.benchmark_single()?;
        results.push(result);
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
