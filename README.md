# Moodle Course Builder

Transform parsed course CSVs into Moodle-ready bulk upload files.

## Quick start

```bash
cargo build --release
./target/release/moodle_course_builder \
  --in-path <INPUT_CSV> \
  --out-path <OUTPUT_CSV> \
  --campus <CAMPUS> \
  --start-date <YYYY-MM-DD> \
  --end-date <YYYY-MM-DD> \
  --dept <DEPARTMENT>
```

## Usage

You can also run via Cargo while iterating:

```bash
cargo run -- \
  --in-path <INPUT_CSV> \
  --out-path <OUTPUT_CSV> \
  --campus <CAMPUS> \
  --start-date <YYYY-MM-DD> \
  --end-date <YYYY-MM-DD> \
  --dept <DEPARTMENT>
```

## Arguments

| Argument | Description | Options / Example |
|----------|-------------|-------------------|
| `--in-path` | Input CSV to transform. | `parsed_courses/ucmn/ccs/bscs/bscs_courses.csv` |
| `--out-path` | Output CSV path. | `parsed_courses/ucmn/ccs/bscs/bscs_courses_cleaned.csv` |
| `--campus` | Campus identifier. | `ucmn`, `uclm`, `ucb`, `ucmetc`, `ucpt` |
| `--start-date` | Start of academic year. | `2025-08-01` |
| `--end-date` | End of academic year. | `2026-06-01` |
| `--dept` | Department name/code. | `ccs` |

## Input CSV requirements

The input CSV must contain these columns:

- `Course Code`
- `Descriptive Title`
- `Program`
- `Semester`

## Output CSV columns

The generated file includes these Moodle fields:

- `shortname`
- `fullname`
- `category_path`
- `startdate`
- `enddate`
- `visible`

## Example

```bash
cargo run -- \
  --in-path parsed_courses/ucmn/ccs/bscs/bscs_courses.csv \
  --out-path parsed_courses/ucmn/ccs/bscs/bscs_courses_cleaned.csv \
  --campus ucmn \
  --start-date 2025-08-01 \
  --end-date 2026-06-01 \
  --dept ccs
```

## Directory notes

- `parsed_courses/` stores CSVs extracted from PDFs.
- `pdfs/` holds the source curriculum documents.
- `course_mappings/` contains mapping files when needed.
