use anchor_lang::prelude::*;
use anchor_spl::{associated_token, token};

use crate::errors::ErrorCode;
use crate::schemas::battle::*;

#[event]
pub struct JoinBattleEvent {
    pub owner: Pubkey,
    pub player: Pubkey,
    pub bet_token: Pubkey,
    pub amount: u64,
}

#[derive(Accounts)]
pub struct JoinBattle<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    #[account(mut)]
    pub battle: Account<'info, Battle>,
    pub bet_token: Box<Account<'info, token::Mint>>,
    #[account(mut)]
    pub ata_player: Account<'info, token::TokenAccount>,
    #[account(mut)]
    pub treasury: Box<Account<'info, token::TokenAccount>>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn exec(ctx: Context<JoinBattle>) -> Result<()> {
    let battle = &mut ctx.accounts.battle;
    // if battle.player == (new Pubkey("111111111111111111111111111111111")) {
    //     return err!(ErrorCode::InvalidAmount);
    // }

    // let approve_ctx = CpiContext::new(
    //     ctx.accounts.token_program.to_account_info(),
    //     token::Approve {
    //         to: ctx.accounts.ata_planer.to_account_info(),
    //         delegate: ctx.accounts.treasury.to_account_info(),
    //         authority: ctx.accounts.planer.to_account_info(),
    //     },
    // );s
    // token::approve(approve_ctx, fund)?;
    let transfer_x_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        token::Transfer {
            from: ctx.accounts.ata_player.to_account_info(),
            to: ctx.accounts.treasury.to_account_info(),
            authority: ctx.accounts.player.to_account_info(),
        },
    );
    token::transfer(transfer_x_ctx, battle.amount)?;

    battle.player = ctx.accounts.player.key();

    emit!(JoinBattleEvent {
        owner: battle.owner.key(),
        player: ctx.accounts.player.key(),
        bet_token: battle.bet_token.key(),
        amount: battle.amount,
    });

    Ok(())
}
