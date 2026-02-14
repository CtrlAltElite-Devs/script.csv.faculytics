use clap::ValueEnum;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Campus {
    Ucmn,
    Uclm,
    Ucb,
    Ucmetc,
    Ucpt,
}

impl std::fmt::Display for Campus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Campus::Ucmn => "ucmn",
            Campus::Uclm => "uclm",
            Campus::Ucb => "ucb",
            Campus::Ucmetc => "ucmetc",
            Campus::Ucpt => "ucpt",
        };
        write!(f, "{}", s)
    }
}
