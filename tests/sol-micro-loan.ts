import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolMicroLoan } from "../target/types/sol_micro_loan";
import { assert } from "chai";

describe("solmicroloan", () => {
  console.log(process.argv);

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SolMicroLoan as Program<SolMicroLoan>;
  const wallet = provider.wallet as anchor.Wallet;
  const connection = provider.connection;

  const borrower = anchor.web3.Keypair.generate();
  const lender = anchor.web3.Keypair.generate();

  before(async () => {
    const latestBlockHash = await provider.connection.getLatestBlockhash();

    const tx = new anchor.web3.Transaction(latestBlockHash);

    const transfer = anchor.web3.SystemProgram.transfer({
      fromPubkey: wallet.publicKey,
      lamports: 10_000_000,
      toPubkey: borrower.publicKey,
    });

    tx.add(transfer);

    await anchor.web3.sendAndConfirmTransaction(connection, tx, [wallet.payer]);

    const txLender = new anchor.web3.Transaction(latestBlockHash);
    txLender.add(
      anchor.web3.SystemProgram.transfer({
        fromPubkey: wallet.publicKey,
        lamports: 10_000_000,
        toPubkey: lender.publicKey,
      })
    );

    await anchor.web3.sendAndConfirmTransaction(connection, txLender, [
      wallet.payer,
    ]);
  });

  it("Create a loan", async () => {
    const num = Buffer.alloc(4);
    num.writeUInt32LE(0, 0);
    const [loanAccountPda, loanAccountBump] =
      anchor.web3.PublicKey.findProgramAddressSync(
        [
          Buffer.from("loan"),
          borrower.publicKey.toBuffer(),
          num, // Assuming loan_count is 0
        ],
        program.programId
      );

    const txSig = await program.methods
      .createLoan(
        new anchor.BN(1000),
        5,
        new anchor.BN(60 * 60 * 24 * 7), // 7 days
        0
      )
      .accountsStrict({
        loanAccount: loanAccountPda,
        borrower: borrower.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([borrower])
      .rpc();

    const latestBlockHash = await provider.connection.getLatestBlockhash();
    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: txSig,
    });

    const loan = await program.account.loanAccount.fetch(loanAccountPda);
    console.log({ loan });
    assert.equal(loan.loanAmount.toNumber(), 1000);
    assert.equal(loan.interestRate, 5);
    assert.equal(loan.duration.toNumber(), 60 * 60 * 24 * 7);
    console.log("Loan created successfully");
  });

  it("Fund loan account", async () => {
    const loanIndex = 0;
    const num = Buffer.alloc(4);
    num.writeUInt32LE(loanIndex, 0);
    const [loanAccountPda, loanAccountBump] =
      anchor.web3.PublicKey.findProgramAddressSync(
        [
          Buffer.from("loan"),
          borrower.publicKey.toBuffer(),
          num, // Assuming loan_count is 0
        ],
        program.programId
      );

    const tx = await program.methods
      .fundLoan(borrower.publicKey, loanIndex)
      .accountsStrict({
        lender: lender.publicKey,
        borrower: borrower.publicKey,
        loanAccount: loanAccountPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([lender])
      .rpc();
    
    const borrowerBlance = await program.provider.connection.getBalance(borrower.publicKey);
    console.log({borrowerBlance});
  
    const loan = await program.account.loanAccount.fetch(loanAccountPda);
    console.log(loan);
    console.log("Loan funded successfully");
  });

  it("Replay Loan", async () => {
    const num = Buffer.alloc(4);
    num.writeUInt32LE(0, 0);
    const [loanAccountPda, loanAccountBump] =
      anchor.web3.PublicKey.findProgramAddressSync(
        [
          Buffer.from("loan"),
          borrower.publicKey.toBuffer(),
          num, // Assuming loan_count is 0
        ],
        program.programId
      );

    const txSig = await program.methods
      .repayLoan(0)
      .accountsStrict({
        loanAccount: loanAccountPda,
        borrower: borrower.publicKey,
        lender: lender.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([borrower])
      .rpc();

    const loan = await program.account.loanAccount.fetch(loanAccountPda);
    console.log(loan);
    console.log("Loan repaid successfully");

  });
});
