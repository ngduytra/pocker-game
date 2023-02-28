use anchor_lang::prelude::*;
use anchor_spl::{associated_token, token};

use crate::errors::ErrorCode;
use crate::schemas::battle::*;

#[event]
pub struct CreateBattleEvent {
    pub owner: Pubkey,
    pub bet_token: Pubkey,
    pub amount: u64,
}

#[derive(Accounts)]
pub struct CreateBattle<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(init, payer = owner, space = Battle::LEN)]
    pub battle: Account<'info, Battle>,
    pub bet_token: Box<Account<'info, token::Mint>>,
    #[account(mut)]
    pub ata_owner: Account<'info, token::TokenAccount>,
    #[account(seeds = [b"treasurer", &battle.key().to_bytes()], bump)]
    /// CHECK: Just a pure account
    pub treasurer: AccountInfo<'info>,
    #[account(
    init,
    payer = owner,
    associated_token::mint = bet_token,
    associated_token::authority = treasurer
  )]
    pub treasury: Box<Account<'info, token::TokenAccount>>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn exec(ctx: Context<CreateBattle>, amount: u64) -> Result<()> {
    let battle = &mut ctx.accounts.battle;
    if amount <= 0 {
        return err!(ErrorCode::InvalidAmount);
    }

    // let approve_ctx = CpiContext::new(
    //     ctx.accounts.token_program.to_account_info(),
    //     token::Approve {
    //         to: ctx.accounts.ata_planer.to_account_info(),
    //         delegate: ctx.accounts.treasury.to_account_info(),
    //         authority: ctx.accounts.planer.to_account_info(),
    //     },
    // );
    // token::approve(approve_ctx, fund)?;
    let transfer_x_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        token::Transfer {
            from: ctx.accounts.ata_owner.to_account_info(),
            to: ctx.accounts.treasury.to_account_info(),
            authority: ctx.accounts.owner.to_account_info(),
        },
    );
    token::transfer(transfer_x_ctx, amount)?;

    battle.owner = ctx.accounts.owner.key();
    battle.bet_token = ctx.accounts.bet_token.key();
    battle.amount = amount;

    emit!(CreateBattleEvent {
        owner: ctx.accounts.owner.key(),
        bet_token: ctx.accounts.bet_token.key(),
        amount,
    });

    Ok(())
}
