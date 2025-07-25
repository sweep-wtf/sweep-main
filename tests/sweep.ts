import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Sweep } from "../target/types/sweep";
import { assert } from "chai";

describe("sweep-program", () => {
  const provider = anchor.AnchorProvider.local();
  anchor.setProvider(provider);
  const program = anchor.workspace.Sweep as Program<Sweep>;

  it("Initializes a sweep action", async () => {
    const tx = await program.methods
      .sweepDust({ minOutSol: new anchor.BN(1000) })
      .rpc();
    console.log("Transaction signature", tx);
    assert.isString(tx);
  });
});
