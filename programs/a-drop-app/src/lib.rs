use anchor_lang::prelude::*;

pub mod error;

use anchor_lang::system_program;

use error::ErrorCode;

use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount}
};

use crate::system_program::{transfer, Transfer};


declare_id!("Fkth3sBewAfunPzzbUjGb5axoFCkPLJ3VDtgcDDuxF4H");

#[program]
pub mod airdrop_platform {
    use anchor_lang::solana_program::system_program;
    use anchor_spl::{associated_token::get_associated_token_address, token};

    use super::*;

    pub fn initialize_platform(ctx: Context<InitializePlatform>, admin: Pubkey) -> Result<()> {
        let admin_account = &mut ctx.accounts.admin_account;
        admin_account.admin = admin;
        Ok(())
    }

    // Initialize function to set up the contract
    pub fn create_airdrop(ctx: Context<CreateAirdrop>, amount: u64) -> Result<()> {
        msg!("Transferring tokens...");
        msg!(
            "Mint: {}",
            &ctx.accounts.mint_account.to_account_info().key()
        );

        msg!(
            "From Token Address: {}",
            &ctx.accounts.sender_token_account.key()
        );

        msg!(
            "To Token Address: {}",
            &ctx.accounts.recipient_token_account.key()
        );

        // Invoke the transfer instruction on the token program
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.sender_token_account.to_account_info(),
                    to: ctx.accounts.recipient_token_account.to_account_info(),
                    authority: ctx.accounts.sender_is_airdrop_creator.to_account_info(),
                },
            ),
            amount * 10u64.pow(ctx.accounts.mint_account.decimals as u32), // Transfer amount, adjust for decimals
        )?;

        msg!("The Mint Account used {}", ctx.accounts.recipient_token_account.mint);

        msg!("The balance of the sender's token account is now {}", ctx.accounts.sender_token_account.amount);

        msg!("The balance of the recipient's token account is now: {}", ctx.accounts.recipient_token_account.amount);


        // let r = ctx.accounts.recipient_token_account.to_account_info();

        msg!("The balance of the recipient's token account is now: {}", ctx.accounts.recipient_token_account.to_account_info().lamports.borrow().clone());

        msg!("Tokens transferred successfully.");

        Ok(())
    }

    pub fn claim_tokens(ctx : Context<ClaimTokens>, claim_fees: u64, number_of_tokens_to_claim : u64) -> Result<()>{

        msg!("Transferring tokens...");
        msg!(
            "Mint: {}",
            &ctx.accounts.mint_account.to_account_info().key()
        );

        // let token_account_address = get_associated_token_address(&ctx.accounts.system_account_or_system_account_signer.key(), token_mint_address, &ctx.accounts.mint_account.to_account_info().key());
        // let r = token_account_address.to_account_info();

        msg!(
            "From Token Address: {}",
            &ctx.accounts.sender_token_account.key()
        );

        msg!(
            "To Token Address: {}",
            &ctx.accounts.recipient_token_account.key()
        );


        // Claimer -> Sytem Account
        // claimer sends SOL = $1 to the contract
        transfer(

            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.claimer.to_account_info(),
                    to: ctx.accounts.system_account_or_system_account_signer.to_account_info(),
                },
            ),
            claim_fees,

        )?;

         // System Account -> Claimer
         // System Account sends the tokens to caller of the function
         // Invoke the transfer instruction on the token program
         token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.sender_token_account.to_account_info(),
                    to: ctx.accounts.recipient_token_account.to_account_info(),
                    authority: ctx.accounts.system_account_or_system_account_signer.to_account_info()
                },
            ),
            number_of_tokens_to_claim * 10u64.pow(ctx.accounts.mint_account.decimals as u32), // Transfer amount, adjust for decimals
        )?;

        msg!("The Mint Account used {}", ctx.accounts.recipient_token_account.mint);

        msg!("The balance of the sender's token account is now {}", ctx.accounts.sender_token_account.amount);

        msg!("The balance of the recipient's token account is now: {}", ctx.accounts.recipient_token_account.amount);


        // let r = ctx.accounts.recipient_token_account.to_account_info();

        msg!("The balance of the recipient's token account is now: {}", ctx.accounts.recipient_token_account.to_account_info().lamports.borrow().clone());

        msg!("Tokens transferred successfully.");

        Ok(())
    }

    //GET TOKEN ACCOUNT INFORMATION
    pub fn get_my_token_information(ctx: Context<GetMyAccountBalance>) -> Result<()> {
        // Convert the token_account to AccountInfo
        let token_account_info = ctx.accounts.token_account.to_account_info();

        // Print the information being fetched
        msg!("Token Account Key: {}", token_account_info.key);
        msg!("Token Account Owner: {}", token_account_info.owner);
        msg!(
            "Token Account Lamports: {}",
            token_account_info.lamports.borrow()
        );
        msg!(
            "Token Account Data Length: {}",
            token_account_info.data_len()
        );
        msg!(
            "Token Account Executable: {}",
            token_account_info.executable
        );
        msg!(
            "Token Account Rent Epoch: {}",
            token_account_info.rent_epoch
        );

        Ok(())
    }

    //GET TOKEN ACCOUNT ADDRESS OWNED BY CONTRACT
    //BUT WE NEED TO REMEMBER THE MINT ADDRESS
    pub fn get_token_account_address_of_contract(ctx: Context<GetTokenAccountOfContract>) -> Result<()> {
        let associated_token_address = get_associated_token_address(&ctx.accounts.contract_itself.key(), &ctx.accounts.token_mint_address.key());
        msg!("token's account_address_of_contract :{}", associated_token_address);
        Ok(())
    }

    // pub fn token_account_address(mint_address : Pubkey, contract_address : Pubkey) -> Pubkey {
    //     let associated_token_address = get_associated_token_address(&contract_address, &mint_address);
    //     msg!("token's account_address_of_contract :{}", associated_token_address);
    //     associated_token_address
    // }



    // pub fn read_price(ctx: Context<Pyth>) -> Result<()> {
    //     let price_feed = &ctx.accounts.price_feed;
    //     let clock = &ctx.accounts.clock;
    //     // Get the current timestamp
    //     let timestamp: i64 = clock.unix_timestamp;
    //     // Load the price from the price feed. Here, the price can be no older than 500 seconds.
    //     let price: pyth_sdk::Price = price_feed
    //         .get_price_no_older_than(timestamp, 30)
    //         .ok_or(ErrorCode::PythError)?;

    //     let confidence_interval: u64 = price.conf;

    //     let asset_price_full: i64 = price.price;

    //     let asset_exponent: i32 = price.expo;

    //     let asset_price = asset_price_full as f64 * 10f64.powi(asset_exponent);

    //     msg!("Price: {}", asset_price);
    //     msg!("Confidence interval: {}", confidence_interval);

    //     Ok(())
    // }

}

#[derive(Accounts)]
pub struct GetTokenAccountOfContract<'info> {
    pub contract_itself: SystemAccount<'info>,

    #[account(mut)]
    pub token_mint_address: Account<'info, Mint>,
}

#[derive(Accounts)]
pub struct GetMyAccountBalance<'info> {
    pub user: Signer<'info>,
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
}

#[derive(Accounts)]
pub struct CreateAirdrop<'info> {
    #[account(mut)]
    pub sender_is_airdrop_creator: Signer<'info>,

    pub recipient: SystemAccount<'info>,

    #[account(mut)]
    pub mint_account: Account<'info, Mint>,
    // O apna mint account kyo de rea?

    #[account(
        mut,
        associated_token::mint = mint_account,
        associated_token::authority = sender_is_airdrop_creator,
    )]
    pub sender_token_account: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = sender_is_airdrop_creator,
        associated_token::mint = mint_account,  // apa same mint account kyo de rhe han?
        associated_token::authority = recipient,
    )]
    pub recipient_token_account: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = sender_is_airdrop_creator,
        space = 8 + AirdropInfo::INIT_SPACE,
    )]
    airdrop_info: Account<'info, AirdropInfo>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ClaimTokens<'info> {

    //will pay the SOL = 1$
    #[account(mut)]
    pub claimer : Signer<'info>,

    #[account(mut)]
    pub system_account_or_system_account_signer : SystemAccount<'info>,
    #[account(mut)]
    pub mint_account: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint_account,
        associated_token::authority = system_account_or_system_account_signer,
    )]
    pub sender_token_account: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = system_account_or_system_account_signer,
        associated_token::mint = mint_account,  // apa same mint account kyo de rhe han?
        associated_token::authority = claimer,
    )]
    pub recipient_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)] // automatically calculate the space required for the struct
pub struct AirdropInfo {
    // pub token : Pubkey,      //32 bytes
    pub drop_amount: u64,
    pub claimable_amount: u64,
    pub claim_fee: u64,
}

#[derive(Accounts)]
pub struct InitializePlatform<'info> {
    #[account(init, payer = admin, space = 8 + 32)]
    pub admin_account: Account<'info, AdminAccount>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct AdminAccount {
    pub admin: Pubkey,
}

// #[derive(Accounts)]
// pub struct Pyth<'info> {
//     pub price_feed: Account<'info, PriceFeed>,
//     pub system_program: Program<'info, System>,
//     pub clock: Sysvar<'info, Clock>,
// }
