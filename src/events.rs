// ----- Events ------------------------------------------------------------- //

pub struct GameOver {
    pub final_score: u32,
}

pub struct PlayerHit {
    pub remaining_health: u8,
}
