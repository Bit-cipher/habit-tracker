use anchor_lang::prelude::*;

#[account]
pub struct HabitCounter {
    pub owner: Pubkey,      
    pub count: u64,              
    pub last_increment: i64,    
    pub habit_name: String,      
    pub created_at: i64,        
    pub bump: u8,              
}

impl HabitCounter {
    pub const MAX_NAME_LENGTH: usize = 50;
    
    pub const LEN: usize = 8 + 32 + 8 + 8 + 54 + 8 + 1;
    
    pub const COOLDOWN_PERIOD: i64 = 86400;
}