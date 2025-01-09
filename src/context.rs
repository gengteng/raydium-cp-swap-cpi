use anchor_lang::prelude::*;

#[account]
pub struct AccountPlaceholder {}

#[derive(Accounts)]
#[instruction(index: u16)]
pub struct CreateAmmConfig<'info> {
    /// Address to be set as protocol owner.
    #[account(mut)]
    pub owner: Signer<'info>,

    /// Initialize config state account to store protocol owner address and fee rates.
    #[account(mut)]
    pub amm_config: Account<'info, AccountPlaceholder>,

    pub system_program: Account<'info, AccountPlaceholder>,
}

#[derive(Accounts)]
pub struct UpdateAmmConfig<'info> {
    /// The amm config owner or admin
    pub owner: Signer<'info>,

    /// Amm config account to be changed
    #[account(mut)]
    pub amm_config: Account<'info, AccountPlaceholder>,
}

#[derive(Accounts)]
pub struct UpdatePoolStatus<'info> {
    pub authority: Signer<'info>,

    #[account(mut)]
    pub pool_state: Account<'info, AccountPlaceholder>,
}

#[derive(Accounts)]
pub struct CollectProtocolFee<'info> {
    /// Only admin or owner can collect fee now
    pub owner: Signer<'info>,

    /// CHECK: pool vault and lp mint authority
    pub authority: Account<'info, AccountPlaceholder>,

    /// Pool state stores accumulated protocol fee amount
    #[account(mut)]
    pub pool_state: Account<'info, AccountPlaceholder>,

    /// Amm config account stores owner
    pub amm_config: Account<'info, AccountPlaceholder>,

    /// The address that holds pool tokens for token_0
    #[account(mut)]
    pub token_0_vault: Account<'info, AccountPlaceholder>,

    /// The address that holds pool tokens for token_1
    #[account(mut)]
    pub token_1_vault: Account<'info, AccountPlaceholder>,

    /// The mint of token_0 vault
    #[account(mut)]
    pub vault_0_mint: Account<'info, AccountPlaceholder>,

    /// The mint of token_1 vault
    #[account(mut)]
    pub vault_1_mint: Account<'info, AccountPlaceholder>,

    /// The address that receives the collected token_0 protocol fees
    #[account(mut)]
    pub recipient_token_0_account: Account<'info, AccountPlaceholder>,

    /// The address that receives the collected token_1 protocol fees
    #[account(mut)]
    pub recipient_token_1_account: Account<'info, AccountPlaceholder>,

    /// The SPL program to perform token transfers
    pub token_program: Account<'info, AccountPlaceholder>,

    /// The SPL program 2022 to perform token transfers
    pub token_program_2022: Account<'info, AccountPlaceholder>,
}

#[derive(Accounts)]
pub struct CollectFundFee<'info> {
    /// Only admin or fund_owner can collect fee now
    pub owner: Signer<'info>,

    /// CHECK: pool vault and lp mint authority
    pub authority: Account<'info, AccountPlaceholder>,

    /// Pool state stores accumulated protocol fee amount
    #[account(mut)]
    pub pool_state: Account<'info, AccountPlaceholder>,

    /// Amm config account stores fund_owner
    pub amm_config: Account<'info, AccountPlaceholder>,

    /// The address that holds pool tokens for token_0
    #[account(mut)]
    pub token_0_vault: Account<'info, AccountPlaceholder>,

    /// The address that holds pool tokens for token_1
    #[account(mut)]
    pub token_1_vault: Account<'info, AccountPlaceholder>,

    /// The mint of token_0 vault
    #[account(mut)]
    pub vault_0_mint: Account<'info, AccountPlaceholder>,

    /// The mint of token_1 vault
    #[account(mut)]
    pub vault_1_mint: Account<'info, AccountPlaceholder>,

    /// The address that receives the collected token_0 fund fees
    #[account(mut)]
    pub recipient_token_0_account: Account<'info, AccountPlaceholder>,

    /// The address that receives the collected token_1 fund fees
    #[account(mut)]
    pub recipient_token_1_account: Account<'info, AccountPlaceholder>,

    /// The SPL program to perform token transfers
    pub token_program: Account<'info, AccountPlaceholder>,

    /// The SPL program 2022 to perform token transfers
    pub token_program_2022: Account<'info, AccountPlaceholder>,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    /// Address paying to create the pool. Can be anyone
    #[account(mut)]
    pub creator: Signer<'info>,

    /// Which config the pool belongs to.
    pub amm_config: Account<'info, AccountPlaceholder>,

    /// CHECK: pool vault and lp mint authority
    pub swap_authority: Account<'info, AccountPlaceholder>,

    /// CHECK: Initialize an account to store the pool state
    #[account(mut)]
    pub pool_state: Account<'info, AccountPlaceholder>,

    /// Token_0 mint, the key must smaller then token_1 mint.
    pub token_0_mint: Account<'info, AccountPlaceholder>,

    /// Token_1 mint, the key must greater than token_0 mint.
    pub token_1_mint: Account<'info, AccountPlaceholder>,

    /// pool lp mint
    #[account(mut)]
    pub lp_mint: Account<'info, AccountPlaceholder>,

    /// payer token0 account
    #[account(mut)]
    pub creator_token_0: Account<'info, AccountPlaceholder>,

    /// creator token1 account
    #[account(mut)]
    pub creator_token_1: Account<'info, AccountPlaceholder>,

    /// creator lp token account
    #[account(mut)]
    pub creator_lp_token: Account<'info, AccountPlaceholder>,

    /// CHECK: Token_0 vault for the pool, created by contract
    #[account(mut)]
    pub token_0_vault: Account<'info, AccountPlaceholder>,

    /// CHECK: Token_1 vault for the pool, created by contract
    #[account(mut)]
    pub token_1_vault: Account<'info, AccountPlaceholder>,

    /// create pool fee account
    #[account(mut)]
    pub create_pool_fee: Account<'info, AccountPlaceholder>,

    /// an account to store oracle observations
    #[account(mut)]
    pub observation_state: Account<'info, AccountPlaceholder>,

    /// Program to create mint account and mint tokens
    pub token_program: Account<'info, AccountPlaceholder>,

    /// Spl token program or token program 2022
    pub token_0_program: Account<'info, AccountPlaceholder>,

    /// Spl token program or token program 2022
    pub token_1_program: Account<'info, AccountPlaceholder>,

    /// Program to create an ATA for receiving position NFT
    pub associated_token_program: Account<'info, AccountPlaceholder>,

    /// To create a new program account
    pub system_program: Account<'info, AccountPlaceholder>,

    /// Sysvar for program account
    pub rent: Account<'info, AccountPlaceholder>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    /// Pays to mint the position
    pub owner: Signer<'info>,

    /// CHECK: pool vault and lp mint authority
    pub authority: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub pool_state: Account<'info, AccountPlaceholder>,

    /// Owner lp token account
    #[account(mut)]
    pub owner_lp_token: Account<'info, AccountPlaceholder>,

    /// The payer's token account for token_0
    #[account(mut)]
    pub token_0_account: Account<'info, AccountPlaceholder>,

    /// The payer's token account for token_1
    #[account(mut)]
    pub token_1_account: Account<'info, AccountPlaceholder>,

    /// The address that holds pool tokens for token_0
    #[account(mut)]
    pub token_0_vault: Account<'info, AccountPlaceholder>,

    /// The address that holds pool tokens for token_1
    #[account(mut)]
    pub token_1_vault: Account<'info, AccountPlaceholder>,

    /// Token Program
    pub token_program: Account<'info, AccountPlaceholder>,

    /// Token program 2022
    pub token_program_2022: Account<'info, AccountPlaceholder>,

    /// The mint of token_0 vault
    #[account(mut)]
    pub vault_0_mint: Account<'info, AccountPlaceholder>,

    /// The mint of token_1 vault
    #[account(mut)]
    pub vault_1_mint: Account<'info, AccountPlaceholder>,

    /// Lp token mint
    #[account(mut)]
    pub lp_mint: Account<'info, AccountPlaceholder>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    /// Pays to mint the position
    pub owner: Signer<'info>,

    /// CHECK: pool vault and lp mint authority
    pub authority: Account<'info, AccountPlaceholder>,

    /// Pool state account
    #[account(mut)]
    pub pool_state: Account<'info, AccountPlaceholder>,

    /// Owner lp token account
    #[account(mut)]
    pub owner_lp_token: Account<'info, AccountPlaceholder>,

    /// The token account for receive token_0
    #[account(mut)]
    pub token_0_account: Account<'info, AccountPlaceholder>,

    /// The token account for receive token_1
    #[account(mut)]
    pub token_1_account: Account<'info, AccountPlaceholder>,

    /// The address that holds pool tokens for token_0
    #[account(mut)]
    pub token_0_vault: Account<'info, AccountPlaceholder>,

    /// The address that holds pool tokens for token_1
    #[account(mut)]
    pub token_1_vault: Account<'info, AccountPlaceholder>,

    /// Token Program
    pub token_program: Account<'info, AccountPlaceholder>,

    /// Token program 2022
    pub token_program_2022: Account<'info, AccountPlaceholder>,

    /// The mint of token_0 vault
    #[account(mut)]
    pub vault_0_mint: Account<'info, AccountPlaceholder>,

    /// The mint of token_1 vault
    #[account(mut)]
    pub vault_1_mint: Account<'info, AccountPlaceholder>,

    /// Pool lp token mint
    #[account(mut)]
    pub lp_mint: Account<'info, AccountPlaceholder>,

    /// memo program
    /// CHECK:
    pub memo_program: Account<'info, AccountPlaceholder>,
}

#[derive(Accounts)]
pub struct Swap<'info> {
    /// The user performing the swap
    pub payer: Signer<'info>,

    /// CHECK: pool vault and lp mint authority
    pub authority: Account<'info, AccountPlaceholder>,

    /// The factory state to read protocol fees
    pub amm_config: Account<'info, AccountPlaceholder>,

    /// The program account of the pool in which the swap will be performed
    #[account(mut)]
    pub pool_state: Account<'info, AccountPlaceholder>,

    /// The user token account for input token
    #[account(mut)]
    pub input_token_account: Account<'info, AccountPlaceholder>,

    /// The user token account for output token
    #[account(mut)]
    pub output_token_account: Account<'info, AccountPlaceholder>,

    /// The vault token account for input token
    #[account(mut)]
    pub input_vault: Account<'info, AccountPlaceholder>,

    /// The vault token account for output token
    #[account(mut)]
    pub output_vault: Account<'info, AccountPlaceholder>,

    /// SPL program for input token transfers
    pub input_token_program: Account<'info, AccountPlaceholder>,

    /// SPL program for output token transfers
    pub output_token_program: Account<'info, AccountPlaceholder>,

    /// The mint of input token
    #[account(mut)]
    pub input_token_mint: Account<'info, AccountPlaceholder>,

    /// The mint of output token
    #[account(mut)]
    pub output_token_mint: Account<'info, AccountPlaceholder>,

    /// The program account for the most recent oracle observation
    #[account(mut)]
    pub observation_state: Account<'info, AccountPlaceholder>,
}
