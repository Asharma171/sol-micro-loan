// This file is auto-generated from the CIDL source.
// Editing this file directly is not recommended as it may be overwritten.
//
// Docs: https://docs.codigo.ai/c%C3%B3digo-interface-description-language/specification#errors

use anchor_lang::prelude::*;

#[error_code]
pub enum MicroLoanError {
    #[msg("Invalid loan amount.")]
    InvalidLoanAmount,
    #[msg("Loan already funded.")]
    LoanAlreadyFunded,
    #[msg("Not authorized.")]
    NotAuthorized,
    #[msg("Loan not funded.")]
    LoanNotFunded,
    #[msg("Repayment period has not started.")]
    RepaymentNotStarted,
    #[msg("Loan already repaid.")]
    LoanAlreadyRepaid,
    #[msg("Loan has defaulted.")]
    LoanDefaulted,
    #[msg("Invalid interest rate.")]
    InvalidInterestRate,
    #[msg("Invalid duration.")]
    InvalidDuration,
    #[msg("Lender not set.")]
    LenderNotSet,
    #[msg("Invalid repayment amount.")]
    InvalidRepaymentAmount,
    #[msg("Invalid repayment period.")]
    InvalidRepaymentPeriod,
    #[msg("Invalid borrower.")]
    InvalidBorrower,
    #[msg("Invalid lender.")]
    InvalidLender,
    #[msg("Invalid loan.")]
    InvalidLoan,
    #[msg("Invalid loan status.")]
    InvalidLoanStatus,
    #[msg("Invalid loan type.")]
    InvalidLoanType,
    #[msg("Invalid loan term.")]
    InvalidLoanTerm,
    #[msg("Invalid loan purpose.")]
    InvalidLoanPurpose,
    #[msg("Invalid loan request.")]
    InvalidLoanRequest,
}

