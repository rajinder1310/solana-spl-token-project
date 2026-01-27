import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TOKEN_PROGRAM_ID, createAccount, getAccount } from "@solana/spl-token";
import { assert } from "chai";

describe("token-contract", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  // Read the program from the workspace
  const program = anchor.workspace.TokenContract;

  let mint: anchor.web3.PublicKey;
  // @ts-ignore
  let payer = provider.wallet as anchor.Wallet;
  let tokenAccountA: anchor.web3.PublicKey;
  let tokenAccountB: anchor.web3.PublicKey;

  it("Is initialized!", async () => {
    // Generate a new mint keypair
    const mintKeypair = anchor.web3.Keypair.generate();
    mint = mintKeypair.publicKey;

    // Execute the initialize_mint instruction
    await program.methods
      .initializeMint(9) // 9 decimals
      .accounts({
        mint: mint,
        payer: payer.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      })
      .signers([mintKeypair])
      .rpc();

    console.log("Mint initialized:", mint.toBase58());
  });

  it("Mints tokens", async () => {
    // Create a token account for the payer
    tokenAccountA = await createAccount(
      provider.connection,
      payer.payer,
      mint,
      payer.publicKey
    );

    const amount = new anchor.BN(1000);

    // Execute the mint_token instruction
    await program.methods
      .mintToken(amount)
      .accounts({
        mint: mint,
        tokenAccount: tokenAccountA,
        authority: payer.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();

    const account = await getAccount(provider.connection, tokenAccountA);
    assert.equal(account.amount.toString(), "1000");
    console.log("Minted 1000 tokens to", tokenAccountA.toBase58());
  });

  it("Transfers tokens", async () => {
    // Create another token account
    const otherUser = anchor.web3.Keypair.generate();
    tokenAccountB = await createAccount(
      provider.connection,
      payer.payer,
      mint,
      otherUser.publicKey
    );

    const amount = new anchor.BN(500);

    // Execute the transfer_token instruction
    await program.methods
      .transferToken(amount)
      .accounts({
        from: tokenAccountA,
        to: tokenAccountB,
        authority: payer.publicKey, // Payer is owner of tokenAccountA
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();

    const accountA = await getAccount(provider.connection, tokenAccountA);
    const accountB = await getAccount(provider.connection, tokenAccountB);

    assert.equal(accountA.amount.toString(), "500");
    assert.equal(accountB.amount.toString(), "500");
    console.log("Transferred 500 tokens to", tokenAccountB.toBase58());
  });
});
