use anchor_lang::prelude::*;

declare_id!("DP2bB5qqaQDyQWYXKy4ixE8wWXLzNh5F2kp4RJqRNkFf");

#[program]
pub mod myepicproject {
  use super::*;
  pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> Result <()> {
    // Get a reference to the account.
    let base_account = &mut ctx.accounts.base_account;
    // Initialize total_gifs.
    base_account.total_gifs = 0;
    Ok(())
  }
  // increment gif counter and take link to gif
  pub fn add_gif(ctx: Context<AddGif>, gif_link: String) -> Result <()> {
    // grab ref to base account then increment counter
    let base_account = &mut ctx.accounts.base_account;
    let user = &mut ctx.accounts.user;

    // gif storing struct
    let item = ItemStruct {
        gif_link: gif_link.to_string(),
        user_address: *user.to_account_info().key,
    };
    // add to the gif_list vector
    base_account.gif_list.push(item);
    base_account.total_gifs += 1;
    Ok(())
  }

}

// Attach certain variables to the StartStuffOff context.
#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program <'info, System>,
}

// Data for addGif Context
#[derive(Accounts)]
pub struct AddGif<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

// Struct to hold gif link and submitter address
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub gif_link: String,
    pub user_address: Pubkey,
}

// Tell Solana what we want to store on this account.
#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
    // Vector of giflink structs
    pub gif_list: Vec<ItemStruct>,
}
