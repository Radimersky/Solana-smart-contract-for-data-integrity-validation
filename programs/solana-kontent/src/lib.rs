use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;
use anchor_lang::solana_program::instruction::Instruction;
use std::convert::Into;
use std::ops::Deref;

declare_id!("4C9v8ZmgtEbx8dDGb7YDXrsLbNswU3HAzhqBCKzUn21N");

#[program]
pub mod solana_kontent {
    use super::*;

	pub fn save_variant(ctx: Context<SaveVariant>, item_id: String, variant_id: String) -> Result<()> {
		let variant: &mut Account<Variant> = &mut ctx.accounts.variant; 
		let author: &Signer = &ctx.accounts.author; 
		let clock: Clock = Clock::get().unwrap();

		if variant_id.chars().count() > 36 {
            return Err(error!(ErrorCode::TopicTooLong));
        }
	
		//variant.author = *author.key; 
		//variant.account_created = clock.unix_timestamp;
		//variant.item_id = item_id;
		variant.variant_id = variant_id;
	
		Ok(()) 
	}

	 // Initializes a new multisig account with a set of owners and a threshold.
	 pub fn create_multisig(
        ctx: Context<CreateMultisig>,
        owners: Vec<Pubkey>,
        threshold: u64,
        nonce: u8,
    ) -> Result<()> {
        assert_unique_owners(&owners)?;
        require!(
            threshold > 0 && threshold <= owners.len() as u64,
            InvalidThreshold
        );
        require!(!owners.is_empty(), InvalidOwnersLen);

        let multisig = &mut ctx.accounts.multisig;
        multisig.owners = owners;
        multisig.threshold = threshold;
        multisig.nonce = nonce;
        multisig.owner_set_seqno = 0;
        Ok(())
    }

    // Creates a new transaction account, automatically signed by the creator,
    // which must be one of the owners of the multisig.
    pub fn create_transaction(
        ctx: Context<CreateTransaction>,
        pid: Pubkey,
        accs: Vec<TransactionAccount>,
        data: Vec<u8>,
    ) -> Result<()> {
        let owner_index = ctx
            .accounts
            .multisig
            .owners
            .iter()
            .position(|a| a == ctx.accounts.proposer.key)
            .ok_or(ErrorCode::InvalidOwner)?;

        let mut signers = Vec::new();
        signers.resize(ctx.accounts.multisig.owners.len(), false);
        signers[owner_index] = true;

        let tx = &mut ctx.accounts.transaction;
        tx.program_id = pid;
        tx.accounts = accs;
        tx.data = data;
        tx.signers = signers;
        tx.multisig = ctx.accounts.multisig.key();
        tx.did_execute = false;
        tx.owner_set_seqno = ctx.accounts.multisig.owner_set_seqno;

        Ok(())
    }

    // Approves a transaction on behalf of an owner of the multisig.
    pub fn approve(ctx: Context<Approve>) -> Result<()> {
        let owner_index = ctx
            .accounts
            .multisig
            .owners
            .iter()
            .position(|a| a == ctx.accounts.owner.key)
            .ok_or(ErrorCode::InvalidOwner)?;

        ctx.accounts.transaction.signers[owner_index] = true;

        Ok(())
    }

    // Set owners and threshold at once.
    pub fn set_owners_and_change_threshold<'info>(
        ctx: Context<'_, '_, '_, 'info, Auth<'info>>,
        owners: Vec<Pubkey>,
        threshold: u64,
    ) -> Result<()> {
        set_owners(
            Context::new(
                ctx.program_id,
                ctx.accounts,
                ctx.remaining_accounts,
                ctx.bumps.clone(),
            ),
            owners,
        )?;
        change_threshold(ctx, threshold)
    }

    // Sets the owners field on the multisig. The only way this can be invoked
    // is via a recursive call from execute_transaction -> set_owners.
    pub fn set_owners(ctx: Context<Auth>, owners: Vec<Pubkey>) -> Result<()> {
        assert_unique_owners(&owners)?;
        require!(!owners.is_empty(), InvalidOwnersLen);

        let multisig = &mut ctx.accounts.multisig;

        if (owners.len() as u64) < multisig.threshold {
            multisig.threshold = owners.len() as u64;
        }

        multisig.owners = owners;
        multisig.owner_set_seqno += 1;

        Ok(())
    }

    // Changes the execution threshold of the multisig. The only way this can be
    // invoked is via a recursive call from execute_transaction ->
    // change_threshold.
    pub fn change_threshold(ctx: Context<Auth>, threshold: u64) -> Result<()> {
        require!(threshold > 0, InvalidThreshold);
        if threshold > ctx.accounts.multisig.owners.len() as u64 {
            return Err(ErrorCode::InvalidThreshold.into());
        }
        let multisig = &mut ctx.accounts.multisig;
        multisig.threshold = threshold;
        Ok(())
    }

    // Executes the given transaction if threshold owners have signed it.
    pub fn execute_transaction(ctx: Context<ExecuteTransaction>) -> Result<()> {
        // Has this been executed already?
        if ctx.accounts.transaction.did_execute {
            return Err(ErrorCode::AlreadyExecuted.into());
        }

        // Do we have enough signers.
        let sig_count = ctx
            .accounts
            .transaction
            .signers
            .iter()
            .filter(|&did_sign| *did_sign)
            .count() as u64;
        if sig_count < ctx.accounts.multisig.threshold {
            return Err(ErrorCode::NotEnoughSigners.into());
        }

        // Execute the transaction signed by the multisig.
        let mut ix: Instruction = (*ctx.accounts.transaction).deref().into();
        ix.accounts = ix
            .accounts
            .iter()
            .map(|acc| {
                let mut acc = acc.clone();
                if &acc.pubkey == ctx.accounts.multisig_signer.key {
                    acc.is_signer = true;
                }
                acc
            })
            .collect();
        let multisig_key = ctx.accounts.multisig.key();
        let seeds = &[multisig_key.as_ref(), &[ctx.accounts.multisig.nonce]];
        let signer = &[&seeds[..]];
        let accounts = ctx.remaining_accounts;
        anchor_lang::solana_program::program::invoke_signed(&ix, accounts, signer)?;

        // Burn the transaction to ensure one time use.
        ctx.accounts.transaction.did_execute = true;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct SaveVariant<'info> { 
	#[account(init, payer = author, space = Variant::LEN)]
	pub variant: Account<'info, Variant>, 
	#[account(mut)]
	pub author: Signer<'info>, 
	pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateMultisig<'info> {
    #[account(zero, signer)]
    multisig: Box<Account<'info, Multisig>>,
}

#[derive(Accounts)]
pub struct CreateTransaction<'info> {
    multisig: Box<Account<'info, Multisig>>,
    #[account(zero, signer)]
    transaction: Box<Account<'info, Transaction>>,
    // One of the owners. Checked in the handler.
    proposer: Signer<'info>,
}

#[derive(Accounts)]
pub struct Approve<'info> {
    #[account(constraint = multisig.owner_set_seqno == transaction.owner_set_seqno)]
    multisig: Box<Account<'info, Multisig>>,
    #[account(mut, has_one = multisig)]
    transaction: Box<Account<'info, Transaction>>,
    // One of the multisig owners. Checked in the handler.
    owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct Auth<'info> {
    #[account(mut)]
    multisig: Box<Account<'info, Multisig>>,
    #[account(
        seeds = [multisig.key().as_ref()],
        bump = multisig.nonce,
    )]
    multisig_signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct ExecuteTransaction<'info> {
    #[account(constraint = multisig.owner_set_seqno == transaction.owner_set_seqno)]
    multisig: Box<Account<'info, Multisig>>,
    #[account(
        seeds = [multisig.key().as_ref()],
        bump = multisig.nonce,
    )]
    /// CHECK: This is not dangerous because we don't read or write from this account
    multisig_signer: UncheckedAccount<'info>,
    #[account(mut, has_one = multisig)]
    transaction: Box<Account<'info, Transaction>>,
}

#[account]
pub struct Variant {
	//pub account_created: i64,
	//pub last_modified: i64,
	pub variant_id: String, // "dd1439d5-4ee2-4895-a4e4-5b0d9d8c754e"
	//pub item_id: String,
	//pub project_id: String,
	//pub author: Pubkey,
	//pub variant_hash: String,
}

#[account]
pub struct Multisig {
    pub owners: Vec<Pubkey>,
    pub threshold: u64,
    pub nonce: u8,
    pub owner_set_seqno: u32,
}

#[account]
pub struct Transaction {
    // The multisig account this transaction belongs to.
    pub multisig: Pubkey,
    // Target program to execute against.
    pub program_id: Pubkey,
    // Accounts requried for the transaction.
    pub accounts: Vec<TransactionAccount>,
    // Instruction data for the transaction.
    pub data: Vec<u8>,
    // signers[index] is true iff multisig.owners[index] signed the transaction.
    pub signers: Vec<bool>,
    // Boolean ensuring one time execution.
    pub did_execute: bool,
    // Owner set sequence number.
    pub owner_set_seqno: u32,
}

const PUBLIC_KEY_LENGTH: usize = 32;
const TIMESTAMP_LENGTH: usize = 8;
const IDENTIFIER_LENGTH: usize = 36;
const HASH_LENGTH: usize = 36;
const DISCRIMINATOR_LENGTH: usize = 8;

impl Variant { 
	const LEN: usize = DISCRIMINATOR_LENGTH 
		+ IDENTIFIER_LENGTH; // variant, item, project ids
}

impl From<&Transaction> for Instruction {
    fn from(tx: &Transaction) -> Instruction {
        Instruction {
            program_id: tx.program_id,
            accounts: tx.accounts.iter().map(Into::into).collect(),
            data: tx.data.clone(),
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct TransactionAccount {
    pub pubkey: Pubkey,
    pub is_signer: bool,
    pub is_writable: bool,
}

impl From<&TransactionAccount> for AccountMeta {
    fn from(account: &TransactionAccount) -> AccountMeta {
        match account.is_writable {
            false => AccountMeta::new_readonly(account.pubkey, account.is_signer),
            true => AccountMeta::new(account.pubkey, account.is_signer),
        }
    }
}

impl From<&AccountMeta> for TransactionAccount {
    fn from(account_meta: &AccountMeta) -> TransactionAccount {
        TransactionAccount {
            pubkey: account_meta.pubkey,
            is_signer: account_meta.is_signer,
            is_writable: account_meta.is_writable,
        }
    }
}

fn assert_unique_owners(owners: &[Pubkey]) -> Result<()> {
    for (i, owner) in owners.iter().enumerate() {
        require!(
            !owners.iter().skip(i + 1).any(|item| item == owner),
            UniqueOwners
        )
    }
    Ok(())
}

#[error_code]
pub enum ErrorCode {
	#[msg("The given owner is not part of this multisig.")]
    InvalidOwner,
    #[msg("Owners length must be non zero.")]
    InvalidOwnersLen,
    #[msg("Not enough owners signed this transaction.")]
    NotEnoughSigners,
    #[msg("Cannot delete a transaction that has been signed by an owner.")]
    TransactionAlreadySigned,
    #[msg("Overflow when adding.")]
    Overflow,
    #[msg("Cannot delete a transaction the owner did not create.")]
    UnableToDelete,
    #[msg("The given transaction has already been executed.")]
    AlreadyExecuted,
    #[msg("Threshold must be less than or equal to the number of owners.")]
    InvalidThreshold,
    #[msg("Owners must be unique")]
    UniqueOwners,
	#[msg("The ID should be 36 characters long.")]
    TopicTooLong,
}