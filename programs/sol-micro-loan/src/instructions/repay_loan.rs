use crate::errors::MicroLoanError;
use crate::state::LoanStatus;
use crate::*;
use anchor_lang::prelude::*;

/// Allows the borrower to make a repayment on a loan.
///
/// Accounts:
/// 0. `[writable, signer]` borrower: [AccountInfo]
/// 1. `[writable]` loan: [Loan]
///
/// Data:
/// - amount: [u64] Repayment amount.
/// - loan_seed_index: [u32] Auto-generated, from the input "loan" for the its seed definition "Loan", sets the seed named "index"
pub fn handler(ctx: Context<RepayLoan>) -> Result<()> {
    let loan_account = &mut ctx.accounts.loan_account;

    require!(
        loan_account.status == LoanStatus::Funded,
        MicroLoanError::LoanAlreadyRepaid
    );

    let current_time = Clock::get()?.unix_timestamp;
    let end_time = loan_account.start_time + loan_account.duration;

    require!(
        current_time >= loan_account.start_time,
        MicroLoanError::RepaymentNotStarted
    );

    if loan_account.lender.is_none() {
        return Err(MicroLoanError::LenderNotSet.into());
    }

    if current_time > end_time {
        loan_account.status = LoanStatus::Defaulted;
        return Err(MicroLoanError::LoanDefaulted.into());
    }

    // Calculate repayment amount with interest
    let interest = (loan_account.loan_amount * loan_account.interest_rate as u64) / 100;
    let total_repayment = loan_account.loan_amount + interest;

    // Transfer repayment from borrower to lender
    let ix = anchor_lang::solana_program::system_instruction::transfer(
        &ctx.accounts.borrower.key(),
        &ctx.accounts.lender.key(),
        total_repayment,
    );

    anchor_lang::solana_program::program::invoke(
        &ix,
        &[
            ctx.accounts.borrower.to_account_info(),
            ctx.accounts.lender.clone(),
        ],
    )?;

    loan_account.status = LoanStatus::Repaid;

    Ok(())
}
