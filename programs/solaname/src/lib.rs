use anchor_lang::prelude::*;

// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("8ZUmf4y3KiNbi8sb68BWxJH4nQXTJvSyfevwsVKm8iV2");

#[program]
mod solaname {
    use super::*;
    pub fn create_user(
        ctx: Context<Initialize>,
        first_name: String,
        last_name: String,
    ) -> Result<()> {
        ctx.accounts.new_account.first_name = first_name.clone();
        ctx.accounts.new_account.last_name = last_name.clone();

        msg!("Tx made by Hakan Bugra Erentug for BlockSprint Project!!");
        msg!("New Name Saved: {} {}!", first_name, last_name);
        // Message will show up in the tx logs
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    // We must specify the space in order to initialize an account.
    // First 8 bytes are default account discriminator,
    // next 8 bytes come from UserAccount.data being type u64.
    // (u64 = 64 bits unsigned integer = 8 bytes)
    #[account(init, payer = signer, space = 8 + 32 + 32)]
    pub new_account: Account<'info, UserAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct UserAccount {
    pub first_name: String,
    pub last_name: String,
}
