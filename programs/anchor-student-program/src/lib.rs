use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{mint_to, Mint, MintTo, Token, TokenAccount};

declare_id!("3QoRrf23SGbLhR38hDipxXyMaPF836NJZNmNQ7JcUuMW");

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

        mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                MintTo {
                    authority: ctx.accounts.initializer.to_account_info(),
                    to: ctx.accounts.token_account.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                },
                &[&["mint".as_bytes(), &[ctx.bumps.mint]]],
            ),
            10 * 10 ^ 6,
        )?;

        msg!("Minted tokens");

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

    pub fn initialize_token_mint(_ctx: Context<InitializeMint>) -> Result<()> {
        msg!("Token mint initialized");
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
    pub token_program: Program<'info, Token>,
    #[account(
        seeds = ["mint".as_bytes()],
        bump,
        mut
    )]
    pub mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = initializer,
        associated_token::mint = mint,
        associated_token::authority = initializer
    )]
    pub token_account: Account<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
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

#[derive(Accounts)]
pub struct InitializeMint<'info> {
    #[account(
        init,
        seeds = ["mint".as_bytes()],
        bump,
        payer = initializer,
        mint::decimals = 6,
        mint::authority = initializer,
    )]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
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
