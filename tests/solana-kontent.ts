import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SolanaKontent } from "../target/types/solana_kontent";
import * as assert from "assert";

interface Variant {
  accountCreated: number;
  lastModified: number;
  variantId: string;
  itemId: string;
  projectId: string;
  variantHash: string;
}

describe("solana-kontent", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.SolanaKontent as Program<SolanaKontent>;

  const variantData: Variant = {
    accountCreated: new anchor.BN(1651041404),
    lastModified: new anchor.BN(1551041404),
    variantId: 'bb1439d5-4ee2-4895-a4e4-5b0d9d8c754e',
    itemId: 'ad1439d5-4ee2-4895-a4e4-5b0d9d8c754e',
    projectId: 'bd1439d5-4ee2-4895-a4e4-5b0d9d8c754e',
    variantHash: '0x7368b03bea99c5525aa7a9ba0b121fc381a4134f90d0f1b4f436266ad0f2b43b'
  }

  it('can save a variant', async () => {
    // Before sending the transaction to the blockchain.
    const variant = anchor.web3.Keypair.generate();
    console.log(variant.publicKey);
    await program.rpc.saveVariant(
      variantData.variantId, 
      variantData.itemId, 
      variantData.projectId, 
      variantData.variantHash,
      variantData.accountCreated,
      variantData.lastModified,
      {
        accounts: {
          variant: variant.publicKey,
          author: program.provider.wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        },
        signers: [variant],
    });

    // After sending the transaction to the blockchain.
    const variantAccount = await program.account.variant.fetch(variant.publicKey);

     assert.equal(variantAccount.variantId, variantData.variantId);
     assert.equal(variantAccount.itemId, variantData.itemId);
     assert.equal(variantAccount.projectId, variantData.projectId);
     assert.equal(variantAccount.variantHash, variantData.variantHash);
     assert.equal(variantAccount.author.toBase58(), program.provider.wallet.publicKey.toBase58());
     assert.ok(variantData.accountCreated);
     assert.ok(variantData.lastModified);
  });
});
