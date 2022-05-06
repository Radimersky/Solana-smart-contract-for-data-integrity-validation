use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;
use anchor_lang::solana_program::instruction::Instruction;
use std::convert::Into;
use std::ops::Deref;

declare_id!("4C9v8ZmgtEbx8dDGb7YDXrsLbNswU3HAzhqBCKzUn21N");

#[program]
pub mod solana_kontent {
    use super::*;

	pub fn save_variant(
		ctx: Context<SaveVariant>, 
		variant_id: String,
		item_id: String,
		project_id: String,
		variant_hash: String,
		account_created: i64,
		last_modified: i64,
	) -> Result<()> {
		let variant: &mut Account<Variant> = &mut ctx.accounts.variant; 
		let author: &Signer = &ctx.accounts.author; 
		let clock: Clock = Clock::get().unwrap();

		if (variant_id.chars().count() != GUID_LENGTH  	
		|| item_id.chars().count() != GUID_LENGTH 
		|| project_id.chars().count() != GUID_LENGTH) {
            return Err(error!(ErrorCode::InvalidGuid));
        }

		if (variant_hash.chars().count() != HASH_LENGTH) {
            return Err(error!(ErrorCode::InvalidHash));
        }
	
		variant.variant_id = variant_id;
		variant.item_id = item_id;
		variant.project_id = project_id;
		variant.variant_hash = variant_hash;
		variant.author = *author.key;
		variant.account_created = account_created;
		variant.last_modified = last_modified;
	
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


#[account]
pub struct Variant {
<<<<<<< HEAD
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
=======
	pub variant_id: String, // "dd1439d5-4ee2-4895-a4e4-5b0d9d8c754e"
	pub item_id: String,
	pub project_id: String,
	pub variant_hash: String,
	pub author: Pubkey,
	pub account_created: i64,
	pub last_modified: i64,
>>>>>>> 3eba295 (Create smart contract basics)
}

const PUBLIC_KEY_LENGTH: usize = 32;
const TIMESTAMP_LENGTH: usize = 8;
<<<<<<< HEAD
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
=======
const GUID_LENGTH: usize = 36; // variant, item, project ids
const HASH_LENGTH: usize = 66; 
const HASH_SIGNATURE_LENGTH: usize = 36;
const DISCRIMINATOR_LENGTH: usize = 26; // Account info

impl Variant { 
	const LEN: usize = DISCRIMINATOR_LENGTH 
		+ GUID_LENGTH * 3
		+ HASH_LENGTH 
		+ PUBLIC_KEY_LENGTH
		+ TIMESTAMP_LENGTH * 2;
>>>>>>> 3eba295 (Create smart contract basics)
}

#[error_code]
pub enum ErrorCode {
	#[msg("The hash should be 66 chracters long.")]
    InvalidHash,
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
	#[msg("The GUID should be 36 characters long.")]
    InvalidGuid,
}