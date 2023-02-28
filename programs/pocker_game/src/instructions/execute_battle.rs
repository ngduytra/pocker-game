use anchor_lang::prelude::*;
use anchor_spl::{associated_token, token};

use crate::errors::ErrorCode;
use crate::schemas::battle::*;
use crate::utils::current_timestamp;

#[event]
pub struct ExecuteBattleEvent {
    pub owner: Pubkey,
    pub player: Pubkey,
    pub bet_token: Pubkey,
    pub winner: Pubkey,
    pub amount: u64,
}

#[derive(Accounts)]
pub struct ExecuteBattle<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut, has_one=owner@ErrorCode::InvalidAuthorization)]
    pub battle: Account<'info, Battle>,
}

pub fn exec(ctx: Context<ExecuteBattle>) -> Result<()> {
    let battle = &mut ctx.accounts.battle;
    let mut winner: Pubkey = battle.owner;
    if 1 as i64 > current_timestamp().ok_or(ErrorCode::InvalidCurrentDate)? {
        winner = battle.player;
    }

    battle.winner = winner;

    emit!(ExecuteBattleEvent {
        owner: ctx.accounts.owner.key(),
        player: battle.player.key(),
        bet_token: battle.bet_token.key(),
        winner: winner,
        amount: battle.amount,
    });

    Ok(())
}
