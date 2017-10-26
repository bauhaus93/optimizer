
pub enum Condition {
    MINT,
    NEAR_MINT,
    EXCELLENT,
    GOOD ,
    LIGHT_PLAYED,
    PLAYED,
    POOR
}

pub fn get_condition_string(condition: Condition) -> &'static str {
    match condition {
        Condition::MINT => "MT",
        Condition::NEAR_MINT => "NM",
        Condition::EXCELLENT => "EX",
        Condition::GOOD => "GD",
        Condition::LIGHT_PLAYED => "LP",
        Condition::PLAYED => "PL",
        Condition::POOR => "PO"
    }
}
