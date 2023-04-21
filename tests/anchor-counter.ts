import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { expect } from "chai";
import {
  Keypair,
  SystemProgram,
  PublicKey,
  Transaction,
  sendAndConfirmTransaction,
} from "@solana/web3.js";
import { AnchorCounter } from "../target/types/anchor_counter";

const COUNTER_ACCOUNT_SIZE = 8 + 8;

describe("anchor-counter", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.AnchorCounter as Program<AnchorCounter>;
  const payer = provider.wallet;
  const counter = anchor.web3.Keypair.generate();

  it("Initialize the account for later use", async () => {
    const counter = Keypair.generate();
    const lamports = await provider.connection.getMinimumBalanceForRentExemption(
      COUNTER_ACCOUNT_SIZE,
    );
    const tx = new Transaction().add(
      SystemProgram.createAccount({
        fromPubkey: provider.wallet.publicKey,
        newAccountPubkey: counter.publicKey,
        lamports,
        space: COUNTER_ACCOUNT_SIZE,
        programId: program.programId,
      })
    );

    await provider.sendAndConfirm(tx, [counter]);
  });

  it("Increment state of that account", async () => {
    const tx = await program.methods
      .increment()
      .accounts({ counter: counter.publicKey, user: payer.publicKey })
      .signers([counter])
      .rpc();

    const account = await program.account.counter.fetch(counter.publicKey);
    expect(account.count.toNumber() === 1);
  });
});
