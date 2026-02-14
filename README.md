# Moodle Course Builder

Transform parsed course CSVs into Moodle-ready bulk upload files.

## Quick start

### Course Mapping Mode (default)

```bash
cargo build --release
./target/release/moodle_course_builder \
  --mode course \
  --in-path <INPUT_CSV> \
  --out-path <OUTPUT_CSV> \
  --campus <CAMPUS> \
  --start-date <YYYY-MM-DD> \
  --end-date <YYYY-MM-DD> \
  --dept <DEPARTMENT> \
  --dry-run \
  --dry-run-show 5
```

### User Seeding Mode

```bash
cargo build --release
./target/release/moodle_course_builder \
  --mode user-seed \
  --out-path <OUTPUT_CSV> \
  --campus <CAMPUS> \
  --count <NUMBER_OF_USERS> \
  --dry-run \
  --dry-run-show 5
```

### User Enrollment Mode

```bash
cargo build --release
./target/release/moodle_course_builder \
  --mode user-enrol \
  --in-path <INPUT_CSV_OF_USERS> \
  --out-path <OUTPUT_CSV> \
  --courses <COURSE_SHORTNAME_1> <COURSE_SHORTNAME_2> ... \
  --dry-run \
  --dry-run-show 5
```

## Usage

You can also run via Cargo while iterating:

### Course Mapping Mode (default)

```bash
cargo run -- \
  --mode course \
  --in-path <INPUT_CSV> \
  --out-path <OUTPUT_CSV> \
  --campus <CAMPUS> \
  --start-date <YYYY-MM-DD> \
  --end-date <YYYY-MM-DD> \
  --dept <DEPARTMENT> \
  --dry-run \
  --dry-run-show 5
```

### User Seeding Mode

```bash
cargo run -- \
  --mode user-seed \
  --out-path <OUTPUT_CSV> \
  --campus <CAMPUS> \
  --count <NUMBER_OF_USERS> \
  --dry-run \
  --dry-run-show 5
```

### User Enrollment Mode

```bash
cargo run -- \
  --mode user-enrol \
  --in-path <INPUT_CSV_OF_USERS> \
  --out-path <OUTPUT_CSV> \
  --courses <COURSE_SHORTNAME_1> <COURSE_SHORTNAME_2> ... \
  --dry-run \
  --dry-run-show 5
```

## Modes

The tool operates in different modes, specified by the `--mode` argument:

*   `course` (default): Transforms raw course CSVs into Moodle-compatible course upload files. Requires `--in-path`.
*   `user-seed`: Generates a specified number of fake user records into a CSV file. Requires `--count`.
*   `user-enrol`: Takes an input CSV of users and enrolls them into specified courses. Requires `--in-path` and `--courses`.

## Arguments

| Argument | Description | Options / Example |
|----------|-------------|-------------------|
| `--mode` | Operation mode of the tool. | `course`, `user-seed`, `user-enrol` |
| `--in-path` | Input CSV to transform (required for `course` and `user-enrol` modes). | `parsed_courses/ucmn/ccs/bscs/bscs_courses.csv` |
| `--out-path` | Output CSV path. | `output.csv` |
| `--campus` | Campus identifier. | `ucmn`, `uclm`, `ucb`, `ucmetc`, `ucpt` |
| `--start-date` | Start of academic year (relevant for `course` mode). | `2025-08-01` |
| `--end-date` | End of academic year (relevant for `course` mode). | `2026-06-01` |
| `--dept` | Department name/code (relevant for `course` mode). | `ccs` |
| `--count` | Number of users to generate (required for `user-seed` mode). | `100` |
| `--courses` | List of course short names for user enrollment (required for `user-enrol` mode). | `BSCS101-2526-UCMN`, `BSIT203-2526-UCMN` |
| `--dry-run` | Run transforms without writing output. | |
| `--dry-run-show` | Show first N output records (dry run only). | `5` |

## Input CSV requirements

The input CSV must contain these columns (for `course` mode):

- `Course Code`
- `Descriptive Title`
- `Program`
- `Semester`

For `user-enrol` mode, the input CSV should contain user details like:
- `username`
- `firstname`
- `lastname`
- `password`

## Output CSV columns

The generated file includes these Moodle fields (for `course` mode):

- `shortname`
- `fullname`
- `category_path`
- `startdate`
- `enddate`
- `visible`

For `user-seed` mode, the output includes:
- `username`
- `firstname`
- `lastname`
- `email`
- `password`

For `user-enrol` mode, the output includes user details plus dynamically generated `courseX` and `roleX` columns.

## Example

### Course Mapping Example

```bash
cargo run -- \
  --mode course \
  --in-path parsed_courses/ucmn/ccs/bscs/bscs_courses.csv \
  --out-path parsed_courses/ucmn/ccs/bscs/bscs_courses_cleaned.csv \
  --campus ucmn \
  --start-date 2025-08-01 \
  --end-date 2026-06-01 \
  --dept ccs \
  --dry-run \
  --dry-run-show 5
```

### User Seeding Example

```bash
cargo run -- \
  --mode user-seed \
  --out-path parsed_users/ucmn/ccs/bscs/bscs_seeded_users.csv \
  --campus ucmn \
  --count 100 \
  --dry-run \
  --dry-run-show 5
```

### User Enrollment Example

```bash
cargo run -- \
  --mode user-enrol \
  --in-path parsed_users/ucmn/ccs/bscs/bscs_seeded_users.csv \
  --out-path enrolled_users.csv \
  --courses BSCS101-2526-UCMN BSIT203-2526-UCMN \
  --dry-run \
  --dry-run-show 5
```

## Directory notes

- `parsed_courses/` stores CSVs extracted from PDFs.
- `pdfs/` holds the source curriculum documents.
- `course_mappings/` contains mapping files when needed.
- `parsed_users/` stores generated or transformed user CSVs.
