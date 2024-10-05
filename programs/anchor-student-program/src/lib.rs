use anchor_lang::prelude::*;

declare_id!("HdBggj1AtyVj5gnM3Ru2A2dVXijFgQZQQX6i5q8xU3JN");

#[program]
pub mod anchor_student_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
