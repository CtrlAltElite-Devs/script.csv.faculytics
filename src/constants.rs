// Input CSV Headers
pub const HEADER_PROGRAM: &str = "Program";
pub const HEADER_SEMESTER: &str = "Semester";
pub const HEADER_COURSE_CODE: &str = "Course Code";
pub const HEADER_DESCRIPTIVE_TITLE: &str = "Descriptive Title";

// Output CSV Headers
pub const HEADER_SHORTNAME: &str = "shortname";
pub const HEADER_FULLNAME: &str = "fullname";
pub const HEADER_CATEGORY_PATH: &str = "category_path";
pub const HEADER_STARTDATE: &str = "startdate";
pub const HEADER_ENDDATE: &str = "enddate";
pub const HEADER_VISIBLE: &str = "visible";

// Default Values
pub const DEFAULT_PROGRAM: &str = "GEN";
pub const VALUE_VISIBLE_TRUE: &str = "1";

// Semester Identifiers
pub const SEMESTER_1: &str = "1";
pub const SEMESTER_2: &str = "2";
pub const KNOWN_SEMESTER_TAGS: [&str; 1] = ["S22526"];

// Date Components (MM-DD)
pub const DATE_SEM1_START: &str = "08-01";
pub const DATE_SEM1_END: &str = "12-18";
pub const DATE_SEM2_START: &str = "01-20";
pub const DATE_SEM2_END: &str = "06-01";
