import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Keypair, SystemProgram } from "@solana/web3.js";
import { SolCounter } from "../target/types/sol_counter";

describe("sol-counter", () => {
  let provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.solCounter as Program<SolCounter>;

  // Generate keypair for counter
  const user_wallet = provider.wallet.publicKey;
  const payer = provider.wallet.payer; // payer = user
  const counter = Keypair.generate();

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

  it("Increment counter", async () => {
    const tx = await program.methods
      .increment()
      .accounts({
        counter: counter.publicKey,
      })
      .signers([payer])
      .rpc();

    console.log("Increment: Txn signature", tx);

    // Fetch the counter account
    const counterAccont = await program.account.counter.fetch(counter.publicKey);
    console.log("Count:", counterAccont.count.toNumber());
  });


});
