use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount};
declare_id!("2HV1ywovUQmKbVkadpBPpb9fSAE4sYfhpPxCUFg26FCp");

fn check_context_validity(
    user: &Signer,
    nft_mint_account: &Account<Mint>,
    nft_token_account: &Account<TokenAccount>,
) {
    //Check the owner of the token account
    assert_eq!(nft_token_account.owner, user.key());
    //Check the mint on the token account
    assert_eq!(nft_token_account.mint, nft_mint_account.key());
    //Check the amount on the token account
    assert_eq!(nft_token_account.amount, 1);
}

#[program]
pub mod cz_story_commit {
    use super::*;
    pub fn initialize(
        ctx: Context<Initialize>,
        bump: u8,

        background: u64,
        body: u64,
        clothes: u64,
        head: u64,
        insidehead: u64,
        eyes: u64,
        mouths: u64,
        hats: u64,
    ) -> Result<()> {
        let user = &ctx.accounts.user;
        let nft_mint_account = &ctx.accounts.nft_mint_account;
        let nft_token_account = &ctx.accounts.nft_token_account;
        check_context_validity(user, nft_mint_account, nft_token_account);

        let commit_account = &mut ctx.accounts.commit_account;
        commit_account.bump = bump;

        // TODO: Use a hash map instead
        commit_account.background = background;
        commit_account.body = body;
        commit_account.clothes = clothes;
        commit_account.head = head;
        commit_account.insidehead = insidehead;
        commit_account.eyes = eyes;
        commit_account.mouths = mouths;
        commit_account.hats = hats;
        Ok(())
    }

    pub fn commit_story(
        ctx: Context<Commit>,
        background: u64,
        body: u64,
        clothes: u64,
        head: u64,
        insidehead: u64,
        eyes: u64,
        mouths: u64,
        hats: u64,
    ) -> Result<()> {
        let user = &ctx.accounts.user;
        let nft_mint_account = &ctx.accounts.nft_mint_account;
        let nft_token_account = &ctx.accounts.nft_token_account;
        check_context_validity(user, nft_mint_account, nft_token_account);

        let commit_account = &mut ctx.accounts.commit_account;
        // TODO: Use a hash map instead
        commit_account.background = background;
        commit_account.body = body;
        commit_account.clothes = clothes;
        commit_account.head = head;
        commit_account.insidehead = insidehead;
        commit_account.eyes = eyes;
        commit_account.mouths = mouths;
        commit_account.hats = hats;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct Initialize<'info> {
    // The mint address of the NFT
    pub nft_mint_account: Account<'info, Mint>,
    //The token account ie. account that the user uses to hold the NFT
    pub nft_token_account: Account<'info, TokenAccount>,

    // The person at the origin of the transaction
    #[account(mut)]
    pub user: Signer<'info>,

    // The account that is going to be created as a PDA
    // TODO: Compute the right space
    #[account(init, seeds = [b"seed".as_ref(), nft_mint_account.key().as_ref()], bump, payer = user, space=8+80)]
    pub commit_account: Account<'info, CommitState>,

    // The system program is required to create the account
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Commit<'info> {
    //The owner of the NFT
    pub user: Signer<'info>,
    //The mint account of the NFT
    pub nft_mint_account: Account<'info, Mint>,
    //The token account ie. account that the user uses to hold the NFT
    pub nft_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub commit_account: Account<'info, CommitState>,
}

#[account]
#[derive(Default)]
pub struct CommitState {
    pub background: u64,
    pub body: u64,
    pub clothes: u64,
    pub head: u64,
    pub insidehead: u64,
    pub eyes: u64,
    pub mouths: u64,
    pub hats: u64,
    pub bump: u8,
}
