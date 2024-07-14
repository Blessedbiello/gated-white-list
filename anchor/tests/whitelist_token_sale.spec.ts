import * as anchor from "@project-serum/anchor";
import { Program, Idl } from "@project-serum/anchor";
import { PublicKey, Keypair, SystemProgram } from "@solana/web3.js";
import { expect } from "chai";
// Import the IDL JSON
import idl from "../target/idl/whitelist_token_sale.json";

describe("whitelist_token_sale", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.local();
  anchor.setProvider(provider);

  // Load the IDL
  const programId = new PublicKey("A1QHrhtGGSMDGaAtXvHwCGLFcABbjVnfuZikLkQipGDn");
  const program = new anchor.Program(idl as unknown as Idl, programId, provider);

  // Accounts
  const authority = Keypair.generate();
  const payer = Keypair.generate();

  let whitelistPda: PublicKey;
  let salePda: PublicKey;

  it("Initializes a whitelist", async () => {
    // Airdrop SOL to payer
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(payer.publicKey, 1000000000),
      "confirmed"
    );

    // Generate the PDA for the whitelist
    const [pda, whitelistBump] = await PublicKey.findProgramAddress(
      [authority.publicKey.toBuffer(), Buffer.from("TestWhitelist")],
      program.programId
    );
    whitelistPda = pda;

    // Initialize the whitelist
    await program.rpc.initWhitelist("TestWhitelist", {
      accounts: {
        whitelist: whitelistPda,
        authority: authority.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [authority],
    });

    // Fetch the whitelist account
    const whitelistAccount = await program.account.whitelistTokenSale.fetch(whitelistPda);
    expect(whitelistAccount.authority.toBase58()).to.equal(authority.publicKey.toBase58());
    expect(whitelistAccount.name).to.equal("TestWhitelist");
  });

  it("Adds an account to the whitelist", async () => {
    const accountToAdd = Keypair.generate();

    // Generate the PDA for the whitelist entry
    const [entryPda, entryBump] = await PublicKey.findProgramAddress(
      [accountToAdd.publicKey.toBuffer(), whitelistPda.toBuffer()],
      program.programId
    );

    // Add account to whitelist
    await program.rpc.addToWhitelist(accountToAdd.publicKey, {
      accounts: {
        entry: entryPda,
        whitelist: whitelistPda,
        authority: authority.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [authority],
    });

    // Fetch the whitelist entry account
    const entryAccount = await program.account.whitelistEntry.fetch(entryPda);
    expect(entryAccount.whitelisted.toBase58()).to.equal(accountToAdd.publicKey.toBase58());
    expect(entryAccount.parent.toBase58()).to.equal(whitelistPda.toBase58());
  });

  it("Initializes a token sale", async () => {
    const tokenPrice = new anchor.BN(1000000);
    const maxPerWallet = new anchor.BN(100);

    // Generate the PDA for the token sale
    const [pda, saleBump] = await PublicKey.findProgramAddress(
      [Buffer.from("sale"), authority.publicKey.toBuffer(), whitelistPda.toBuffer()],
      program.programId
    );
    salePda = pda;

    // Initialize the token sale
    await program.rpc.initSale(tokenPrice, maxPerWallet, {
      accounts: {
        sale: salePda,
        authority: authority.publicKey,
        whitelist: whitelistPda,
        systemProgram: SystemProgram.programId,
      },
      signers: [authority],
    });

    // Fetch the token sale account
    const saleAccount = await program.account.sale.fetch(salePda);
    expect(saleAccount.tokenPrice.toString()).to.equal(tokenPrice.toString());
    expect(saleAccount.maxPerWallet.toString()).to.equal(maxPerWallet.toString());
    expect(saleAccount.whitelist.toBase58()).to.equal(whitelistPda.toBase58());
  });

  it("Buys tokens", async () => {
    const accountToAdd = Keypair.generate();

    // Generate the PDA for the whitelist entry
    const [entryPda, entryBump] = await PublicKey.findProgramAddress(
      [accountToAdd.publicKey.toBuffer(), whitelistPda.toBuffer()],
      program.programId
    );

    // Add account to whitelist
    await program.rpc.addToWhitelist(accountToAdd.publicKey, {
      accounts: {
        entry: entryPda,
        whitelist: whitelistPda,
        authority: authority.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [authority],
    });

    // Buy tokens
    const buyAmount = new anchor.BN(10);

    // Generate the PDA for the user account
    const [userPda, userBump] = await PublicKey.findProgramAddress(
      [Buffer.from("user"), accountToAdd.publicKey.toBuffer(), salePda.toBuffer()],
      program.programId
    );

    await program.rpc.buyTokens(buyAmount, {
      accounts: {
        buyer: accountToAdd.publicKey,
        sale: salePda,
        whitelistEntry: entryPda,
        user: userPda,
        systemProgram: SystemProgram.programId,
      },
      signers: [accountToAdd],
    });

    // Fetch the user account
    const userAccount = await program.account.user.fetch(userPda);
    expect(userAccount.purchasedAmount.toString()).to.equal(buyAmount.toString());
  });
});
