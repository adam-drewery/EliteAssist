#[derive(Default, Clone, Debug)]
pub struct Powerplay {
    pub power: Option<Box<str>>,      // Current pledged power (if any)
    pub rank: Option<u8>,             // Current powerplay rank (0-5/10 depending on schema)
    pub merits: u64,                  // Current merits tally
    pub time_pledged: u64,            // Seconds pledged (as provided by journal)
    pub last_salary: Option<u64>,     // Last salary amount received (if any)
}
