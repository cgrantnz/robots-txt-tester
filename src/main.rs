use std::io::Write;
use std::path::Path;
use std::{fs::{self, File}, error::Error, process};
use clap::Parser;
use junit_report::{Duration, ReportBuilder, TestCase, TestCaseBuilder, TestSuiteBuilder};

use lenient_bool::LenientBool;
use texting_robots::Robot;

/// Simple program to validate robots.txt files against test cases
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// robots.txt file content path
    #[arg(short, long)]
    robots_text_file_path: String,

    /// test cases file content path
    #[arg(short, long)]
    test_case_file_path: String,
    
    /// generate test report
    #[arg(short, long, default_value_t = false)]
    generate_test_report: bool,
}

fn main() { 
    let start = std::time::Instant::now();
    let args = Args::parse();
    let robots_content = fs::read_to_string(args.robots_text_file_path).expect("Unable to read robots.txt file");

    let r = Robot::new("googlebot", robots_content.as_bytes()).unwrap();

    let test_cases = match get_test_cases(&args.test_case_file_path) {
        Ok(test_cases)  => test_cases,
        Err(e) => {
            println!("error getting test cases: {}", e);
            process::exit(1);
        },
    };

    let test_results: Vec<TestCaseOutput> = test_cases.into_iter()
        .map(|test| {
            let matcher_result = r.allowed(&test.url);
            // println!("Expected result: {}, result: {}", test.expected_result, matcher_result);
            TestCaseOutput {
                result: matcher_result == test.expected_result,
                expected_result: test.expected_result,
                url: test.url,
                user_agent: test.user_agent
            }
        })
        .collect();

    // Generate JUnit XML
    if args.generate_test_report {
        let test_case_input_file_name = Path::new(&args.test_case_file_path)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .replace(".csv", "");

        generate_test_report(&test_results, &test_case_input_file_name);
    }

    println!("Test cases run: {}", test_results.len());

    println!("Passed tests: {}", test_results.iter().filter(|n| n.result).count());

    let failed_test_count = test_results.iter().filter(|m| m.result).count();
    println!("Failed tests: {}", failed_test_count);
    println!("Elapsed time {:.2}ms", start.elapsed().as_millis());

    // Exit with an error code if any of the tests failed
    if failed_test_count > 0 {
        process::exit(1);
    }
}

fn generate_test_report(test_results: &[TestCaseOutput], test_suite_name: &str) {

    let mut test_cases: Vec<TestCase> = Vec::new();

    for result in test_results {
        let test_case_name = get_test_case_name(result);

        match result.result {
            true => {
                let test_success = TestCaseBuilder::success(&test_case_name, Duration::seconds(0))
                .build();
                test_cases.push(test_success);
            }
            false => {
                    let test_failure: TestCase = TestCase::failure(
                        &test_case_name,
                        Duration::seconds(0),
                        "assert_eq",
                        "not equal",
                    );
                    test_cases.push(test_failure);
                }
        }
    }

    let test_suite = TestSuiteBuilder::new(test_suite_name)
        .add_testcases(test_cases)
        .build();

    let r = ReportBuilder::new()
        .add_testsuite(test_suite)
        .build();

    let mut file = File::create(format!("./{}.robots-test-results.xml", &test_suite_name)).unwrap();
    r.write_xml(&mut file).unwrap();
    file.flush().unwrap();
    file.sync_all().unwrap();
}

fn get_test_case_name(result: &TestCaseOutput) -> String {
    let expected_result_label = if result.expected_result { "allowed" } else { "denied" };
    format!("Accessing URL: {} as {} should be {}", result.url, result.user_agent, expected_result_label)
}

struct TestCaseDefinition {
    user_agent: String,
    url: String,
    expected_result: bool
}

struct TestCaseOutput {
    user_agent: String,
    url: String,
    expected_result: bool,
    result: bool
}

fn get_test_cases(file_path: &str) -> Result<Vec<TestCaseDefinition>, Box<dyn Error>> {
    let test_case_content = fs::read_to_string(file_path)?;
    let mut test_cases: Vec<TestCaseDefinition> = Vec::new();
    let mut rdr = csv::Reader::from_reader(test_case_content.as_bytes());

    for result in rdr.records() {
        let record = result?;

        let test_case = TestCaseDefinition {
            user_agent: record[0].to_string(),
            url: record[1].to_string(),
            expected_result: record[2].parse::<LenientBool>().unwrap().into(),
        };

        test_cases.push(test_case);
    }
    Ok(test_cases)
}
