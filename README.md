# Robots.txt Tester

A command-line tool for testing robots.txt files against a set of URLs to verify if they are allowed or disallowed.

## Overview

This Rust-based application allows you to validate whether specific URLs are allowed or disallowed according to the rules defined in a robots.txt file. It's useful for:

- Testing your robots.txt configuration against multiple user agents
- Verifying crawler access to specific URLs
- Ensuring your SEO strategy is correctly implemented at the robots.txt level
- Testing how different user agents (like Googlebot, Bingbot, etc.) interact with your robots.txt rules

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) and Cargo (Rust's package manager)

### Building from Source

1. Clone the repository:
   ```
   git clone git@github.com:cgrantnz/robots-txt-tester.git
   cd robots-txt-tester
   ```

2. Build the application:
   ```
   cargo build
   ```

For a release build with optimizations:
```
cargo build --release
```

## Usage

### Basic Usage

```
./target/debug/robots-txt-tester --robots-text-file-path <path-to-robots-txt> --test-case-file-path <path-to-test-cases>
```

Example:
```
./target/debug/robots-txt-tester --robots-text-file-path sample_robots_txt.txt --test-case-file-path test_cases.csv
```

### Generating Test Reports

Add the `--generate-test-report` flag to create a JUnit XML report:

```
./target/debug/robots-txt-tester --robots-text-file-path sample_robots_txt.txt --test-case-file-path test_cases.csv --generate-test-report
```

This will generate a file named `<test-case-filename>.robots-test-results.xml` in the current directory.

## File Formats

### robots.txt

The application accepts standard robots.txt files with directives like:

```
Allow: /path/to/allow
Disallow: /path/to/disallow
```

Example (sample_robots_txt.txt):
```
Allow: /a/motors/cars*
Disallow: /a/motors/cars/$
Disallow: /a/motors/cars/*/$
Disallow: /a/motors/cars/0*
...
```

### Test Cases (CSV)

Test cases should be provided in CSV format:

```
user_agent,url,expected_result
```

Example (test_cases.csv):
```
googlebot,https://example.com/a/motors/cars/listing,true
googlebot,https://example.com/a/motors/cars/,false
bingbot,https://example.com/a/motors/cars/listing,true
bingbot,https://example.com/a/motors/cars/,false
...
```

Where:
- `user_agent`: The user agent to test (e.g., "googlebot", "bingbot", etc.)
- `url`: The URL to test against the robots.txt rules
- `expected_result`: `true` if the URL should be allowed, `false` if it should be disallowed

The application will test each URL with the specified user agent, allowing you to verify how different crawlers would interact with your robots.txt rules.

## Command-Line Options

| Option | Description |
|--------|-------------|
| `--robots-text-file-path`, `-r` | Path to the robots.txt file (required) |
| `--test-case-file-path`, `-t` | Path to the test cases CSV file (required) |
| `--generate-test-report`, `-g` | Generate a JUnit XML report (optional) |

## Output

The application outputs:
- Number of test cases run
- Number of passed tests
- Number of failed tests
- Execution time

Example:
```
Test cases run: 9
Passed tests: 9
Failed tests: 0
Elapsed time 5ms
```

## License

This project is licensed under the terms specified in the LICENSE file.
