use crate::*;
use anchor_lang::prelude::*;
use crate::state::LoanStatus;
use crate::errors::MicroLoanError;

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
pub fn handler(ctx: Context<FundLoan>) -> Result<()> {
    let loan_account = &mut ctx.accounts.loan_account;

    require!(
        loan_account.status == LoanStatus::Pending,
        MicroLoanError::LoanAlreadyFunded
    );

    // Transfer funds from lender to borrower
    let ix = anchor_lang::solana_program::system_instruction::transfer(
        &ctx.accounts.lender.key(),
        &ctx.accounts.borrower.key(),
        loan_account.loan_amount,
    );

    anchor_lang::solana_program::program::invoke(
        &ix,
        &[
            ctx.accounts.lender.to_account_info(),
            ctx.accounts.borrower.clone(),
        ],
    )?;

    loan_account.funded_amount = loan_account.loan_amount;
    loan_account.lender = Some(ctx.accounts.lender.key());
    loan_account.status = LoanStatus::Funded;
    loan_account.start_time = Clock::get()?.unix_timestamp;

    Ok(())
}
