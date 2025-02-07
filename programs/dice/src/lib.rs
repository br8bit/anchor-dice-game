use anchor_lang::prelude::*;

declare_id!("8i4esFWjHMHWXhP3oH9WNzLTARtBmYXxuoWhpoy14sEU");

pub mod instructions;
pub mod state;

#[program]
pub mod dice {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}
