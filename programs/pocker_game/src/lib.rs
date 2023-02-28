use anchor_lang::prelude::*;

pub mod schemas;
pub use schemas::*;

pub mod errors;
pub use errors::*;

pub mod instructions;
pub use instructions::*;

pub mod constants;

pub mod utils;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod pocker_game {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn create_battle(ctx: Context<CreateBattle>, amount: u64) -> Result<()> {
        create_battle::exec(ctx, amount)
    }

    pub fn join_battle(ctx: Context<JoinBattle>) -> Result<()> {
        join_battle::exec(ctx)
    }

    pub fn execute_battle(ctx: Context<ExecuteBattle>) -> Result<()> {
        execute_battle::exec(ctx)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
