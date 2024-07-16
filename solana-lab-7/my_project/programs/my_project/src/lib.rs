use anchor_lang::prelude::*;

declare_id!("H4q9mk6S216dL3QfBdDZreEieJRWxjHMNtzuoQWwJvEE");

#[program]
pub mod my_project {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
