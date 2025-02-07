use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub house: Signer<'info>,
    #[account(
        mut,
        seeds = [b"vault", house.key().as_ref()],
        bump,
    )]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl Initialize<'_> {
    pub fn init(&mut self, amount: u64) -> Result<()> {
        let cpi_ctx = CpiContext::new(
            self.system_program.to_account_info(),
            Transfer {
                from: self.house.to_account_info(),
                to: self.vault.to_account_info(),
            },
        );

        transfer(cpi_ctx, amount)
    }
}
