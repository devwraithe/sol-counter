import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Keypair, SystemProgram } from "@solana/web3.js";
import { SolCounter } from "../target/types/sol_counter";
import { assert } from "chai";

describe("sol-counter", () => {
  let provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.solCounter as Program<SolCounter>;

  const user_wallet = provider.wallet.publicKey;
  const payer = provider.wallet.payer; // payer = user
  const counter = Keypair.generate();

  const incrementNum = 20;
  const decrementNum = 8;

  it("Initialize counter", async () => {
    const tx = await program.methods
      .initialize()
      .accounts({
        user: user_wallet,
        counter: counter.publicKey,
      })
      .signers([counter, payer])
      .rpc();

    console.log("Initialize counter: Txn signature", tx);
  });

  it("Increment counter by 1", async () => {
    const tx = await program.methods
      .incrementOne()
      .accounts({
        counter: counter.publicKey,
      })
      .signers([payer])
      .rpc();

    console.log("Increment 1: Txn signature", tx);

    // Fetch the counter account
    const counterAccount = await program.account.counter.fetch(counter.publicKey);
    const countNumber = counterAccount.count.toNumber();
    console.log("Count:", countNumber);

    assert(countNumber === 1, "Expected to increment by 1");
  });

  it("Decrement counter by 1", async () => {
    const tx = await program.methods
      .decrementOne()
      .accounts({
        counter: counter.publicKey,
      })
      .signers([payer])
      .rpc();

    console.log("Decrement 1: Txn signature", tx);

    const counterAccount = await program.account.counter.fetch(counter.publicKey);
    const countNumber = counterAccount.count.toNumber();
    console.log("Count:", countNumber);

    assert(countNumber === 0, "Expected to decrement by 1");
  });

  it(`Increment counter by ${incrementNum}`, async () => {
    const tx = await program.methods
      .incrementAny(new anchor.BN(incrementNum))
      .accounts({
        counter: counter.publicKey,
      })
      .signers([payer])
      .rpc();

    console.log(`Increment ${incrementNum}: Txn signature`, tx);

    const counterAccount = await program.account.counter.fetch(counter.publicKey);
    const countNumber = counterAccount.count.toNumber();
    console.log("Count:", countNumber);

    assert(countNumber === 20, `Expected to increment by ${incrementNum}`);
  });

  it(`Decrement counter by ${decrementNum}`, async () => {
    const tx = await program.methods
      .decrementAny(new anchor.BN(decrementNum))
      .accounts({
        counter: counter.publicKey,
      })
      .signers([payer])
      .rpc();

    console.log(`Decrement ${decrementNum}: Txn signature`, tx);

    const counterAccount = await program.account.counter.fetch(counter.publicKey);
    const countNumber = counterAccount.count.toNumber();
    console.log("Count:", countNumber);

    assert(countNumber === 12, `Expected to decrement by ${decrementNum}`);
  });
});
