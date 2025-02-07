use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

use crate::state::Bet;

#[derive(Accounts)]
#[instruction(seed:u128)]
pub struct PlaceBet<'info> {
    #[account(mut)]
    pub player: Signer<'info>,

    pub house: SystemAccount<'info>,

    #[account(
        mut,
        seeds = [b"vault", house.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,

    #[account(
        init,
        payer = player,
        space = 8 + Bet::INIT_SPACE,
        seeds = [b"bet", vault.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump
    )]
    pub bet: Account<'info, Bet>,

    pub system_program: Program<'info, System>,
}

impl PlaceBet<'_> {
    pub fn place_bet(
        &mut self,
        seed: u128,
        roll: u8,
        amount: u64,
        bumps: &PlaceBetBumps,
    ) -> Result<()> {
        self.create_bet(bumps, seed, roll, amount)?;
        self.deposit(amount)?;
        Ok(())
    }
    fn create_bet(
        &mut self,
        bumps: &PlaceBetBumps,
        seed: u128,
        roll: u8,
        amount: u64,
    ) -> Result<()> {
        self.bet.set_inner(Bet {
            slot: Clock::get()?.slot,
            player: self.player.key(),
            seed,
            roll,
            amount,
            bump: bumps.bet,
        });
        Ok(())
    }

    fn deposit(&mut self, amount: u64) -> Result<()> {
        let ctx = CpiContext::new(
            self.system_program.to_account_info(),
            Transfer {
                from: self.player.to_account_info(),
                to: self.vault.to_account_info(),
            },
        );
        transfer(ctx, amount)
    }
}
