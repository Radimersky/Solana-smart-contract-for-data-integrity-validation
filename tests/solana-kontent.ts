import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SolanaKontent } from "../target/types/solana_kontent";

describe("solana-kontent", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.SolanaKontent as Program<SolanaKontent>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
