pub use crate::constants::*;
use anchor_lang::prelude::*;
use num_traits::ToPrimitive;

#[account]
pub struct Battle {
    pub owner: Pubkey,
    pub player: Pubkey,
    pub winner: Pubkey,
    pub bet_token: Pubkey,
    pub amount: u64,
}

impl Battle {
    pub const LEN: usize = DISCRIMINATOR_SIZE + PUBKEY_SIZE * 4 + U64_SIZE;

    // pub fn swap(&self, a: u64) -> Option<(u64, u64, u64)> {
    //     let x_ = self.x.checked_add(a)?;
    //     let y_ = self
    //         .x
    //         .to_u128()?
    //         .checked_mul(self.y.to_u128()?)?
    //         .checked_div(x_.to_u128()?)?
    //         .to_u64()?;
    //     let b = self.y.checked_sub(y_)?;
    //     Some((b, x_, y_))
    // }
}
