use anchor_lang::prelude::*;

declare_id!("HdBggj1AtyVj5gnM3Ru2A2dVXijFgQZQQX6i5q8xU3JN");

const MAX_NAME_LENGTH: usize = 20;
const MAX_DESCRIPTION_LENGTH: usize = 50;
const DISCRIMINATOR: usize = 8;

#[program]
pub mod anchor_student_program {
    use super::*;

    pub fn add_student(ctx: Context<AddStudent>, name: String, description: String) -> Result<()> {
        // We require that the title is not longer than 20 characters
        require!(name.len() <= MAX_NAME_LENGTH, StudentError::NameTooLong);

        // We require that the description is not longer than 50 characters
        require!(
            description.len() <= MAX_DESCRIPTION_LENGTH,
            StudentError::DescriptionTooLong
        );

        msg!("Student Account Created");
        msg!("Name: {}", name);
        msg!("Description: {}", description);

        let student = &mut ctx.accounts.student;
        student.creator = ctx.accounts.initializer.key();
        student.name = name;
        student.description = description;
        Ok(())
    }

    pub fn update_student(
        ctx: Context<UpdateStudent>,
        name: String,
        description: String,
    ) -> Result<()> {
        msg!("Student account space reallocated");
        msg!("Name: {}", name);
        msg!("Description: {}", description);

        let student = &mut ctx.accounts.student;
        student.description = description;

        Ok(())
    }

    pub fn delete_student(_ctx: Context<DeleteStudent>, name: String) -> Result<()> {
        msg!("Student for {} deleted", name);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(name:String, description:String)]
pub struct AddStudent<'info> {
    #[account(
        init_if_needed,
        seeds = [name.as_bytes(), initializer.key().as_ref()],
        bump,
        payer = initializer,
        space = DISCRIMINATOR + Student::INIT_SPACE
    )]
    pub student: Account<'info, Student>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(name:String, description:String)]
pub struct UpdateStudent<'info> {
    #[account(
        mut,
        seeds = [name.as_bytes(), initializer.key().as_ref()],
        bump,
        realloc = DISCRIMINATOR + Student::INIT_SPACE,
        realloc::payer = initializer,
        realloc::zero = true,
    )]
    pub student: Account<'info, Student>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct DeleteStudent<'info> {
    #[account(
        mut,
        seeds=[name.as_bytes(), initializer.key().as_ref()],
        bump,
        close=initializer
    )]
    pub student: Account<'info, Student>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct Student {
    pub creator: Pubkey, // 32
    #[max_len(20)]
    pub name: String, // 4 + len()
    #[max_len(50)]
    pub description: String, // 4 + len()
}

#[error_code]
enum StudentError {
    #[msg("Student Name too long")]
    NameTooLong,
    #[msg("Student Description too long")]
    DescriptionTooLong,
}
