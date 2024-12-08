use crate::errors::MicroLoanError;
use crate::state::LoanStatus;
use crate::*;
use anchor_lang::prelude::*;

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
pub fn handler(
    ctx: Context<CreateLoan>,
    loan_amount: u64,
    interest_rate: u8,
    duration: i64,
) -> Result<()> {
    require!(loan_amount > 0, MicroLoanError::InvalidLoanAmount);
    require!(interest_rate > 0, MicroLoanError::InvalidInterestRate);
    require!(duration > 0, MicroLoanError::InvalidDuration);

    let loan_account = &mut ctx.accounts.loan_account;
    loan_account.borrower = ctx.accounts.borrower.key();
    loan_account.loan_amount = loan_amount;
    loan_account.interest_rate = interest_rate;
    loan_account.duration = duration;
    loan_account.funded_amount = 0;
    loan_account.start_time = 0;
    loan_account.status = LoanStatus::Pending;
    loan_account.lender = None;

    Ok(())
}
