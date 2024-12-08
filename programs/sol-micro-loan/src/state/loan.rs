// This file is auto-generated from the CIDL source.
// Editing this file directly is not recommended as it may be overwritten.

use anchor_lang::prelude::*;

#[account]
pub struct LoanAccount {
    pub borrower: Pubkey,
    pub lender: Option<Pubkey>,
    pub loan_amount: u64,
    pub interest_rate: u8,
    pub duration: i64,
    pub start_time: i64,
    pub funded_amount: u64,
    pub status: LoanStatus,
}

impl LoanAccount {
    pub const LEN: usize = 8 + 32 + 33 + 8 + 1 + 8 + 8 + 8 + 1;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum LoanStatus {
    Pending = 0,
    Funded = 1,
    Repaid = 2,
    Defaulted = 3,
}
