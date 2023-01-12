pub enum TimeKind {
    Workday { times: Vec<String> },
    Holiday,
    Illness,
}

/// Entity for saving Time data
///  ```
/// ```
///
pub struct Time {
    Kind: TimeKind,
}

impl Time {
    pub fn save() {}
}
