use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount};
use mpl_token_metadata;
use mpl_token_metadata::state::{Metadata, PREFIX};

declare_id!("2HV1ywovUQmKbVkadpBPpb9fSAE4sYfhpPxCUFg26FCp");

/// Transfers lamports from one account (must be program owned)
/// to another account. The recipient can by any account
fn transfer_lamports(
    from_account: &AccountInfo,
    to_account: &AccountInfo,
    amount_of_lamports: u64,
) -> Result<()> {
    // Does the from account have enough lamports to transfer?
    // if **from_account.try_borrow_lamports()? < amount_of_lamports {
    //     return Err(CustomError::InsufficientFundsForTransaction.into());
    // }
    // Debit from_account and credit to_account
    **from_account.try_borrow_mut_lamports()? -= amount_of_lamports;
    **to_account.try_borrow_mut_lamports()? += amount_of_lamports;
    Ok(())
}

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

fn check_metadata_validity(
    nft_token_account: &Account<TokenAccount>,
    nft_metadata_account: &AccountInfo,
    mpl_metadata_program: &AccountInfo,
) {
    let metadata_seed = &[
        PREFIX.as_bytes(),
        mpl_metadata_program.key.as_ref(),
        nft_token_account.mint.as_ref(),
    ];

    let (metadata_key, _metadata_seed) =
        Pubkey::find_program_address(metadata_seed, mpl_metadata_program.key);
    assert_eq!(metadata_key, nft_metadata_account.key());
}

#[program]
pub mod cz_story_commit {
    use super::*;
    pub fn initialize_bank(
        ctx: Context<InitializeBank>,
        bump: u8,
        creator_key: Pubkey,
    ) -> Result<()> {
        let bank_account = &mut ctx.accounts.bank_account;

        bank_account.bump = bump;
        bank_account.creator_key = creator_key;
        Ok(())
    }

    pub fn initialize_commit(
        ctx: Context<InitializeCommit>,
        bump: u8,

        background: u8,
        body: u8,
        clothes: u8,
        head: u8,
        insidehead: u8,
        eyes: u8,
        mouths: u8,
        hats: u8,
    ) -> Result<()> {
        let user = &ctx.accounts.user;
        let bank_account = &ctx.accounts.bank_account;
        let nft_mint_account = &ctx.accounts.nft_mint_account;
        let nft_token_account = &ctx.accounts.nft_token_account;
        let nft_metadata_account = &ctx.accounts.nft_metadata_account;
        let mpl_metadata_program = &ctx.accounts.mpl_metadata_program;

        check_context_validity(user, nft_mint_account, nft_token_account);

        let metadata = Metadata::from_account_info(nft_metadata_account)?;
        check_metadata_validity(
            nft_token_account,
            nft_metadata_account,
            mpl_metadata_program,
        );
        assert_eq!(
            metadata.data.creators.unwrap()[0].address,
            bank_account.creator_key
        );

        // Extract a service 'fee' for performing this instruction
        transfer_lamports(
            &bank_account.to_account_info(),
            &user.to_account_info(),
            100_000_000u64,
        )?;

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
        background: u8,
        body: u8,
        clothes: u8,
        head: u8,
        insidehead: u8,
        eyes: u8,
        mouths: u8,
        hats: u8,
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
pub struct InitializeBank<'info> {
    // The person at the origin of the transaction
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(init, seeds = [b"bank_v17".as_ref()], bump, payer = user, space=80)]
    pub bank_account: Account<'info, Bank>,

    // The system program is required to create the account
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct InitializeCommit<'info> {
    // The mint address of the NFT
    pub nft_mint_account: Account<'info, Mint>,
    //The token account ie. account that the user uses to hold the NFT
    pub nft_token_account: Account<'info, TokenAccount>,
    /// CHECK: ok
    pub nft_metadata_account: AccountInfo<'info>,

    // The person at the origin of the transaction
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub bank_account: Account<'info, Bank>,

    // The account that is going to be created as a PDA
    #[account(init, seeds = [b"commit_v01".as_ref(), nft_mint_account.key().as_ref()], bump, payer = user, space=8+40)]
    pub commit_account: Account<'info, CommitState>,

    // The system program is required to create the account
    pub system_program: Program<'info, System>,
    // The metaplex metadata program is required to resolve the master edition PDA
    /// CHECK: ok
    #[account(address = mpl_token_metadata::ID)]
    pub mpl_metadata_program: AccountInfo<'info>,
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
    pub background: u8,
    pub body: u8,
    pub clothes: u8,
    pub head: u8,
    pub insidehead: u8,
    pub eyes: u8,
    pub mouths: u8,
    pub hats: u8,
    pub bump: u8,
}

#[account]
#[derive(Default)]
pub struct Bank {
    pub bump: u8,
    pub creator_key: Pubkey,
}
