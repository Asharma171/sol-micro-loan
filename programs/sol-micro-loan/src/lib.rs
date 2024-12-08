// This file is auto-generated from the CIDL source.
// Editing this file directly is not recommended as it may be overwritten.

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("8Mcy3PYdYBAywjq7oweV1LQBaEQqi58hFZJgg3qA9DqN");

#[program]
pub mod sol_micro_loan {
    use super::*;

    /// Creates a new loan request.
    ///
    /// Accounts:
    /// 0. `[writable, signer]` borrower: [AccountInfo]
    /// 1. `[writable]` loan: [Loan]
    /// 2. `[]` system_program: [AccountInfo] Auto-generated, for account initialization
    ///
    /// Data:
    /// - amount: [u64] The amount requested for the loan.
    /// - interest_rate: [u8] Interest rate for the loan.
    /// - duration: [u64] Duration of the loan in seconds.
    /// - loan_seed_index: [u32] Auto-generated, from the input "loan" for the its seed definition "Loan", sets the seed named "index"
    pub fn create_loan(
        ctx: Context<CreateLoan>,
        amount: u64,
        interest_rate: u8,
        duration: i64,
        _loan_seed_index: u32,
    ) -> Result<()> {
        create_loan::handler(ctx, amount, interest_rate, duration)
    }

    /// Allows a lender to fund a loan request.
    ///
    /// Accounts:
    /// 0. `[writable, signer]` lender: [AccountInfo]
    /// 1. `[writable]` loan: [Loan]
    ///
    /// Data:
    /// - amount: [u64] Amount to fund the loan.
    /// - loan_seed_borrower: [Pubkey] Auto-generated, from the input "loan" for the its seed definition "Loan", sets the seed named "borrower"
    /// - loan_seed_index: [u32] Auto-generated, from the input "loan" for the its seed definition "Loan", sets the seed named "index"
    pub fn fund_loan(
        ctx: Context<FundLoan>,
        _loan_seed_borrower: Pubkey,
        _loan_seed_index: u32,
    ) -> Result<()> {
        fund_loan::handler(ctx)
    }

    /// Allows the borrower to make a repayment on a loan.
    ///
    /// Accounts:
    /// 0. `[writable, signer]` borrower: [AccountInfo]
    /// 1. `[writable]` loan: [Loan]
    ///
    /// Data:
    /// - amount: [u64] Repayment amount.
    /// - loan_seed_index: [u32] Auto-generated, from the input "loan" for the its seed definition "Loan", sets the seed named "index"
    pub fn repay_loan(ctx: Context<RepayLoan>, _loan_seed_index: u32) -> Result<()> {
        repay_loan::handler(ctx)
    }

    #[derive(Accounts)]
    #[instruction(
		amount: u64,
		interest_rate: u8,
		duration: i64,
		loan_seed_index: u32,
	)]
    pub struct CreateLoan<'info> {
        #[account(

        	init,
        	space = LoanAccount::LEN,
        	payer=borrower,
        	seeds = [
        		b"loan",
        		borrower.key().as_ref(),
        		loan_seed_index.to_le_bytes().as_ref(),
        	],
        	bump,
        )]
        pub loan_account: Account<'info, LoanAccount>,

        #[account(mut)]
        pub borrower: Signer<'info>,

        pub system_program: Program<'info, System>,
    }

    #[derive(Accounts)]
    #[instruction(
		loan_seed_borrower: Pubkey,
		loan_seed_index: u32,
	)]
    pub struct FundLoan<'info> {
        #[account(mut)]
        pub lender: Signer<'info>,

        /// CHECK: Safe because we check borrower key
        #[account(mut, address = loan_account.borrower)]
        pub borrower: AccountInfo<'info>,

        #[account(
			mut,
			seeds = [
				b"loan",
				loan_seed_borrower.as_ref(),
				loan_seed_index.to_le_bytes().as_ref(),
			],
			bump,
		)]
        pub loan_account: Account<'info, LoanAccount>,

        pub system_program: Program<'info, System>,
    }

    #[derive(Accounts)]
    #[instruction(
		loan_seed_index: u32,
	)]
    pub struct RepayLoan<'info> {
        #[account(mut)]
        pub borrower: Signer<'info>,

        /// CHECK: This account is verified in the handler

        #[account(mut)]
        pub lender: AccountInfo<'info>,

        #[account(
            mut,
            seeds = [
                b"loan",
                borrower.key().as_ref(),
                loan_seed_index.to_le_bytes().as_ref(),
            ],
            bump,
        )]
        pub loan_account: Account<'info, LoanAccount>,

        pub system_program: Program<'info, System>,
    }
}
