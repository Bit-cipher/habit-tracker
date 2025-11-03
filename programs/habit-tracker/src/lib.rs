#[allow(unexpected_cfgs, deprecated)]
use anchor_lang::prelude::*;

pub mod state;
pub mod errors;
pub mod instructions;

use instructions::*;

declare_id!("Cm6zdUtRQiFaoxhpcXFbfjNxsFjhQr5AkZS6bwr29QAb");

#[program]
pub mod habit_tracker {
    use super::*;

    pub fn initialize_counter(
        ctx: Context<InitializeCounter>,
        habit_name: String,
    ) -> Result<()> {
        instructions::initialize::handler(ctx, habit_name)
    }

    pub fn increment_counter(ctx: Context<IncrementCounter>) -> Result<()> {
        instructions::increment::handler(ctx)
    }
}