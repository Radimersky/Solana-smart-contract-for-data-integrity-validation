import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SolanaKontent } from "../target/types/solana_kontent";
import * as assert from "assert";
import * as bs58 from "bs58";

interface Variant {
  lastModified: number;
  variantId: string;
  itemId: string;
  itemCodename: string,
  projectId: string;
  variantHash: string;
  variantHashSignature: string;
}

const variantData: Variant = {
  lastModified: new anchor.BN(1551041404),
  variantId: 'default',
  itemId: 'ad1439d5-4ee2-4895-a4e4-5b0d9d8c754e',
  itemCodename: 'some_codename_that_is_sixty_characters_long_which_is_maxsize',
  projectId: 'bd1439d5-4ee2-4895-a4e4-5b0d9d8c754e',
  variantHash: '5f951f757209cd0a6d5c153fa3c2d83028476cfe',
  variantHashSignature: 'MEYCIQCdl6nwzByuBcXgXY+HNjwqqxspd34h2KAJZU+vBm45swIhALZAtmWSOEE4rpvZLQcJsVGjETbD0I9qQG/iJryNkWqA',
}

const differentAuthor = anchor.web3.Keypair.generate();

describe("solana-kontent", () => {

  const saveVariant = async (
    variant: anchor.web3.Keypair, 
    authorPubKey: anchor.web3.PublicKey,
    signers: anchor.web3.Keypair[]
    ) => {
    await program.rpc.saveVariant(
      variantData.projectId, 
      variantData.itemCodename,
      variantData.variantId, 
      variantData.itemId, 
      variantData.variantHash,
      variantData.variantHashSignature,
      variantData.lastModified,
      {
        accounts: {
          variant: variant.publicKey,
          author: authorPubKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        },
        signers: signers,
    });
  };

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.SolanaKontent as Program<SolanaKontent>;

  it('can save a variant', async () => {
    // Before sending the transaction to the blockchain.
    const variant = anchor.web3.Keypair.generate();

    await saveVariant(variant, program.provider.wallet.publicKey, [variant]);

    // After sending the transaction to the blockchain.
    const variantAccount = await program.account.variant.fetch(variant.publicKey);

     assert.equal(variantAccount.variantId, variantData.variantId);
     assert.equal(variantAccount.itemId, variantData.itemId);
     assert.equal(variantAccount.itemCodename, variantData.itemCodename);
     assert.equal(variantAccount.projectId, variantData.projectId);
     assert.equal(variantAccount.variantHash, variantData.variantHash);
     assert.equal(variantAccount.variantHashSignature, variantData.variantHashSignature);
     assert.equal(variantAccount.author.toBase58(), program.provider.wallet.publicKey.toBase58());
     assert.ok(variantAccount.accountCreated);
     assert.ok(variantAccount.lastModified);
  });

  it('can use different author', async () => {    
    // Airdrop SOL to different author
    const signature = await program.provider.connection.requestAirdrop(differentAuthor.publicKey, 10000000);
    // Await until we get the airdrop
    await program.provider.connection.confirmTransaction(signature);

    const variant = anchor.web3.Keypair.generate();
    await saveVariant(variant, differentAuthor.publicKey, [variant, differentAuthor]);

    const variantAccount = await program.account.variant.fetch(variant.publicKey);

    assert.equal(variantAccount.variantId, variantData.variantId);
    assert.equal(variantAccount.itemId, variantData.itemId);
    assert.equal(variantAccount.itemCodename, variantData.itemCodename);
    assert.equal(variantAccount.projectId, variantData.projectId);
    assert.equal(variantAccount.variantHash, variantData.variantHash);
    assert.equal(variantAccount.variantHashSignature, variantData.variantHashSignature);
    assert.equal(variantAccount.author.toBase58(), differentAuthor.publicKey.toBase58());
    assert.ok(variantAccount.accountCreated);
    assert.ok(variantAccount.lastModified);
  });

  it('can fetch all variants', async () => {
    const variants = await program.account.variant.all();
    // 2 accounts because they were created in tests above
    assert.equal(variants.length, 2);
  });

  it('can get variants of single author', async () => {
    const authorPubKey = differentAuthor.publicKey;
    console.log(authorPubKey.toBase58().length);
    const variants = await program.account.variant.all([
        {
            memcmp: {
                offset: 8, // Discriminator.
                bytes: authorPubKey.toBase58(),
            }
        }
    ]);
    console.log(authorPubKey);
    console.log(authorPubKey.toBuffer());
    assert.equal(variants.length, 1);
    assert.ok(variants.every(variant => {
      return variant.account.author.toBase58() === authorPubKey.toBase58()
    }))
  });
  
  it('can get variants by project ID', async () => {
    const variants = await program.account.variant.all([
        {
            memcmp: {
                offset: 8 + // Discriminator.
                     32 + // Author public key.
                     4, // String length prefix 
                bytes: bs58.encode(Buffer.from(variantData.projectId)),
            }
        }
    ]);

    assert.equal(variants.length, 2);
    assert.ok(variants.every(variant => {
        return variant.account.projectId === variantData.projectId;
    }))
  });

  it('can get variants by item codename', async () => {
    const variants = await program.account.variant.all([
        {
            memcmp: {
                offset: 8 + // Discriminator.
                    32 + // Author public key.
                    4 + 36 + // String length prefix + Project ID.
                    4, // String length prefix 
                bytes: bs58.encode(Buffer.from(variantData.itemCodename)),
            }
        }
    ]);

    assert.equal(variants.length, 2);
    assert.ok(variants.every(variant => {
        return variant.account.itemCodename === variantData.itemCodename;
    }))
  });

  it('can get variants by variant ID', async () => {
    const variants = await program.account.variant.all([
        {
            memcmp: {
                offset: 8 + // Discriminator.
                    32 + // Author public key.
                    4 + 36 + // String length prefix + Project ID.
                    4 + 60 + // String length prefix + Item codename.
                    4, // String length prefix 
                bytes: bs58.encode(Buffer.from(variantData.variantId)),
            }
        }
    ]);

    assert.equal(variants.length, 2);
    assert.ok(variants.every(variant => {
        return variant.account.variantId === variantData.variantId;
    }))
  });

  it('can delete a variant', async () => {
    // Create a new variant.
    const author = program.provider.wallet.publicKey;
    const variant = anchor.web3.Keypair.generate();

    await saveVariant(variant, author, [variant]);

    // Delete the Tweet.
    await program.rpc.deleteVariant({
        accounts: {
            variant: variant.publicKey,
            author,
        },
    });

    // Ensure fetching the tweet account returns null.
    const variantAccount = await program.account.variant.fetchNullable(variant.publicKey);
    assert.ok(variantAccount === null);
});

it('cannot delete variant of other author', async () => {
  // Create a new variant.
  const author = program.provider.wallet.publicKey;
  const variant = anchor.web3.Keypair.generate();

    await saveVariant(variant, author, [variant]);

  // Try to delete the Tweet from a different author.
  try {
      await program.rpc.deleteVariant({
          accounts: {
              variant: variant.publicKey,
              author: anchor.web3.Keypair.generate().publicKey,
          },
      });
      assert.fail('Someone else variant was deleted');
  } catch (error) {
      const variantAccount = await program.account.variant.fetch(variant.publicKey);
      assert.equal(variantAccount.projectId, variantData.projectId);
      assert.equal(variantAccount.variantHash, variantData.variantHash);
  }
});

  var guidTestData = [
    {guid: 'aa-bbb'},
    {guid: 'bb1439d5-4ee2-4895-a4e4-5b0d9d8c754e-longer'},
    {guid: ''},
  ];

  guidTestData.forEach((guidData) => {
    it('fails with guid ' + guidData.guid + 'that is not 36 chars long', async () => {
      const variant = anchor.web3.Keypair.generate();
      
      try {
        await program.rpc.saveVariant(
          guidData.guid, 
          variantData.itemId, 
          variantData.projectId, 
          variantData.variantHash,
          variantData.variantHashSignature,
          variantData.lastModified,
          {
            accounts: {
              variant: variant.publicKey,
              author: program.provider.wallet.publicKey,
              systemProgram: anchor.web3.SystemProgram.programId,
            },
            signers: [variant],
        });
        assert.fail();
      } catch (err) {
        assert.equal(err.code, 6001);
        assert.equal(err.msg, 'The GUID should be 36 characters long.');
      }
    });
  });
});
