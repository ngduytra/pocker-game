import * as anchor from "@project-serum/anchor";
import {
  AnchorProvider,
  BN,
  Program,
  setProvider,
  Spl,
  utils,
  web3,
} from "@project-serum/anchor";
import { expect } from "chai";
import { PockerGame } from "../target/types/pocker_game";
import { initializeAccount, initializeMint, mintTo } from "./pretest";

describe("pocker_game", () => {
  // Configure the client to use the local cluster.
  const provider = AnchorProvider.env();
  setProvider(provider);

  const spl = Spl.token();

  // Initialize
  const token = new web3.Keypair();
  let tokenAccount: web3.PublicKey;

  const battle = new web3.Keypair();
  let treasurer: web3.PublicKey;
  let treasury: web3.PublicKey;

  const program = anchor.workspace.PockerGame as Program<PockerGame>;

  before(async () => {
    // Init mints
    await initializeMint(6, token, spl);
    // Init accounts
    tokenAccount = await anchor.utils.token.associatedAddress({
      mint: token.publicKey,
      owner: provider.wallet.publicKey,
    });
    await initializeAccount(
      tokenAccount,
      token.publicKey,
      provider.wallet.publicKey,
      provider
    );

    // Mint tokens
    await mintTo(new BN("1000000000000"), token.publicKey, tokenAccount, spl);
    // await mintTo(new BN("1000000000000"), yToken.publicKey, yTokenAccount, spl);
    // Derive treasury & treasurer
    const [treasurerPublicKey] = await web3.PublicKey.findProgramAddress(
      [Buffer.from("treasurer"), battle.publicKey.toBuffer()],
      program.programId
    );
    treasurer = treasurerPublicKey;
    treasury = await anchor.utils.token.associatedAddress({
      mint: token.publicKey,
      owner: treasurer,
    });
  });

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });

  it("Create Battle", async () => {
    // Add your test here.
    const txId = await program.methods
      .createBattle(new BN("50000000000"))
      .accounts({
        owner: provider.wallet.publicKey,
        battle: battle.publicKey,
        betToken: token.publicKey,
        ataOwner: tokenAccount,
        treasurer,
        treasury,
      })
      .signers([battle])
      .rpc();
    expect(txId).to.be.an("string");
  });

  it("Join Battle", async () => {
    // Add your test here.
    const txId = await program.methods
      .joinBattle()
      .accounts({
        player: provider.wallet.publicKey,
        battle: battle.publicKey,
        betToken: token.publicKey,
        ataPlayer: tokenAccount,
        treasury,
      })
      .rpc();
    expect(txId).to.be.an("string");
  });

  it("Execute Battle", async () => {
    // Add your test here.
    const txId = await program.methods
      .executeBattle()
      .accounts({
        owner: provider.wallet.publicKey,
        battle: battle.publicKey,
      })
      .rpc();
    expect(txId).to.be.an("string");
  });
});
