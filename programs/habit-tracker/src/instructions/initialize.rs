use anchor_lang::prelude::*;
use crate::state::HabitCounter;
use crate::errors::HabitTrackerError;

#[derive(Accounts)]
#[instruction(habit_name: String)]
pub struct InitializeCounter<'info> {
    #[account(
        init,
        payer = owner,
        space = HabitCounter::LEN,
        seeds = [
            b"habit_counter",
            owner.key().as_ref(),
            habit_name.as_bytes()
        ],
        bump
    )]
    pub habit_counter: Account<'info, HabitCounter>,
    
    #[account(mut)]
    pub owner: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitializeCounter>, habit_name: String) -> Result<()> {
    require!(
        habit_name.len() <= HabitCounter::MAX_NAME_LENGTH,
        HabitTrackerError::HabitNameTooLong
    );
    
    let habit_counter = &mut ctx.accounts.habit_counter;
    let clock = Clock::get()?;
    
    habit_counter.owner = ctx.accounts.owner.key();
    habit_counter.count = 0;
    habit_counter.last_increment = 0;
    habit_counter.habit_name = habit_name;
    habit_counter.created_at = clock.unix_timestamp;
    habit_counter.bump = ctx.bumps.habit_counter;
    
    msg!("Habit counter initialized: {}", habit_counter.habit_name);
    
    Ok(())
}