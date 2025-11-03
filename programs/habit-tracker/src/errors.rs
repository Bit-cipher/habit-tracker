use anchor_lang::prelude::*;

#[error_code]
pub enum HabitTrackerError {
    #[msg("Habit name is too long. Maximum 50 characters.")]
    HabitNameTooLong,
    
    #[msg("Must wait 24 hours between check-ins")]
    CooldownNotElapsed,
    
    #[msg("Counter overflow detected")]
    CounterOverflow,
}