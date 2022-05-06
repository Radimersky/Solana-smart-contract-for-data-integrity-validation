import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SolanaKontent } from "../target/types/solana_kontent";
import * as assert from "assert";

describe("solana-kontent", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.SolanaKontent as Program<SolanaKontent>;

  it('can save a variant', async () => {
    // Before sending the transaction to the blockchain.
    const variant = anchor.web3.Keypair.generate();

    await program.rpc.saveVariant('ggg', 'ABCD1234', {
        accounts: {
          variant: variant.publicKey,
          author: program.provider.wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        },
        signers: [variant],
    });

    // After sending the transaction to the blockchain.
    const variantAccount = await program.account.variant.fetch(variant.publicKey);
  	console.log(variantAccount);

     // Ensure it has the right data.
     assert.equal(variantAccount.author.toBase58(), program.provider.wallet.publicKey.toBase58());
     assert.equal(variantAccount.itemId, 'ggg');
     assert.equal(variantAccount.variantId, 'ABCD1234');
  });
});
