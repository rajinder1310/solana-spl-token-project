import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { StakingContract } from "../target/types/staking_contract";
import {
  createMint,
  getOrCreateAssociatedTokenAccount,
  mintTo,
  getAccount
} from "@solana/spl-token";
import { assert } from "chai";

describe("staking_contract", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.StakingContract as Program<StakingContract>;

  // Test variables
  let mint: anchor.web3.PublicKey;
  let vault: anchor.web3.PublicKey;
  let vaultBump: number;
  let stakeInfo: anchor.web3.PublicKey;

  // User (Staker) will be the provider for simplicity
  const staker = provider.wallet.publicKey;
  let stakerTokenAccount: anchor.web3.PublicKey;

  it("Is initialized!", async () => {
    // 1. Create a new Mint (Token)
    mint = await createMint(
      provider.connection,
      (provider.wallet as anchor.Wallet).payer, // Payer
      provider.wallet.publicKey, // Mint Authority
      null, // Freeze Authority
      6 // Decimals
    );
    console.log("Mint Created:", mint.toString());

    // 2. Derive Vault PDA
    [vault, vaultBump] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vault"), mint.toBuffer()],
      program.programId
    );
    console.log("Vault PDA:", vault.toString());

    // 3. Call Initialize
    const tx = await program.methods
      .initialize()
      .accounts({
        vault: vault,
        mint: mint,
        payer: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      })
      .rpc();

    console.log("Your transaction signature", tx);

    // Verify Vault Created
    const vaultAccount = await getAccount(provider.connection, vault);
    assert.ok(vaultAccount.owner.equals(vault), "Vault should be owned by PDA");
    assert.ok(vaultAccount.mint.equals(mint), "Vault should store correct mint");
  });

  it("Deposits Tokens!", async () => {
    // 1. Get/Create User's Token Account
    const stakerAta = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      (provider.wallet as anchor.Wallet).payer,
      mint,
      staker
    );
    stakerTokenAccount = stakerAta.address;

    // 2. Mint tokens to User
    await mintTo(
      provider.connection,
      (provider.wallet as anchor.Wallet).payer,
      mint,
      stakerTokenAccount,
      provider.wallet.publicKey,
      1000 // Amount to mint
    );

    // 3. Derive Stake Info PDA
    const [userStakeInfo] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("user"), staker.toBuffer()],
      program.programId
    );
    stakeInfo = userStakeInfo;

    // 4. Call Deposit
    const depositAmount = new anchor.BN(500);

    await program.methods
      .deposit(depositAmount) // Deposit 500
      .accounts({
        staker: staker,
        vault: vault,
        stakeInfo: stakeInfo,
        mint: mint,
        stakerTokenAccount: stakerTokenAccount,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log("Deposit Successful!");

    // 5. Verify Balances
    const vaultAccount = await getAccount(provider.connection, vault);
    assert.equal(Number(vaultAccount.amount), 500, "Vault should have 500 tokens");

    const userAccount = await getAccount(provider.connection, stakerTokenAccount);
    assert.equal(Number(userAccount.amount), 500, "User should have 500 tokens left");

    // 6. Verify On-Chain Data
    const stakeInfoAccount = await program.account.userStakeInfo.fetch(stakeInfo);
    assert.equal(stakeInfoAccount.amount.toNumber(), 500, "Stake Info should record 500");
    console.log("Stake Info Amount:", stakeInfoAccount.amount.toString());
  });
});
