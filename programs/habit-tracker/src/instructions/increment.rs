use anchor_lang::prelude::*;
use crate::state::HabitCounter;
use crate::errors::HabitTrackerError;

#[derive(Accounts)]
pub struct IncrementCounter<'info> {
    #[account(
        mut,
        seeds = [
            b"habit_counter",
            owner.key().as_ref(),
            habit_counter.habit_name.as_bytes()
        ],
        bump = habit_counter.bump,
        has_one = owner
    )]
    pub habit_counter: Account<'info, HabitCounter>,
    
    pub owner: Signer<'info>,
}

pub fn handler(ctx: Context<IncrementCounter>) -> Result<()> {
    let habit_counter = &mut ctx.accounts.habit_counter;
    let clock = Clock::get()?;
    let current_time = clock.unix_timestamp;
    
    // Check cooldown period (24 hours)
    let time_since_last = current_time - habit_counter.last_increment;
    require!(
        time_since_last >= HabitCounter::COOLDOWN_PERIOD || habit_counter.last_increment == 0,
        HabitTrackerError::CooldownNotElapsed
    );
    
    // Increment counter with overflow check
    habit_counter.count = habit_counter.count
        .checked_add(1)
        .ok_or(HabitTrackerError::CounterOverflow)?;
    
    habit_counter.last_increment = current_time;
    
    msg!(
        "Habit '{}' checked in! Total count: {}",
        habit_counter.habit_name,
        habit_counter.count
    );
    
    Ok(())
}