use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;
use anchor_lang::solana_program::instruction::Instruction;
use std::convert::Into;
use std::ops::Deref;

declare_id!("8rDWSauWEjKVZ6bKM1s8DrTtuLsqj2Eo3xSqUx9RvtfY");

#[program]
pub mod solana_kontent {
    use super::*;

	pub fn save_variant(
		ctx: Context<SaveVariant>, 
		project_id: String,
		item_codename: String,
		variant_id: String,
		variant_hash: String,
		variant_hash_signature: String,
		last_modified: i64,
	) -> Result<()> {
		let variant: &mut Account<Variant> = &mut ctx.accounts.variant; 
		let author: &Signer = &ctx.accounts.author; 
		let clock: Clock = Clock::get().unwrap();

		if variant_id.chars().count() > 60 {
			return Err(ErrorCode::InvalidVariantId.into())
		}

		if project_id.chars().count() != 36 {
			return Err(ErrorCode::InvalidProjectId.into())
		}

		if item_codename.chars().count() > 60 {
			return Err(ErrorCode::InvalidItemCodename.into())
		}

		if variant_hash.chars().count() > 40 {
			return Err(ErrorCode::InvalidHash.into())
		}

		if variant_hash_signature.chars().count() > 96 {
			return Err(ErrorCode::InvalidHashSignature.into())
		}

		variant.variant_id = variant_id;
		variant.project_id = project_id;
		variant.item_codename = item_codename;
		variant.variant_hash = variant_hash;
		variant.variant_hash_signature = variant_hash_signature;
		variant.author = *author.key;
		variant.account_created = clock.unix_timestamp;
		variant.last_modified = last_modified;
	
		Ok(()) 
	}

	pub fn delete_variant(_ctx: Context<DeleteVariant>) -> Result<()> {
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
pub struct DeleteVariant<'info> {
    #[account(mut, has_one = author, close = author)]
    pub variant: Account<'info, Variant>,
    pub author: Signer<'info>,
}

#[account]
pub struct Variant {
	pub author: Pubkey,
	pub project_id: String,
	pub item_codename: String,
	pub variant_id: String,
	pub variant_hash: String,
	pub variant_hash_signature: String,
	pub last_modified: i64,
	pub account_created: i64,
}

const PUBLIC_KEY_LENGTH: usize = 32; 
const TIMESTAMP_LENGTH: usize = 8;
const GUID_LENGTH: usize = 36;
const HASH_LENGTH: usize = 40;
const HASH_SIGNATURE_LENGTH: usize = 96;
const DISCRIMINATOR_LENGTH: usize = 8;
const CODENAME_LENGTH: usize = 60 * 4; 
const STRING_LENGTH_PREFIX: usize = 4; // Stores the length of the string.

impl Variant { 
	const LEN: usize = DISCRIMINATOR_LENGTH // Account metadata
		+ (STRING_LENGTH_PREFIX + GUID_LENGTH) // Project_id
		+ (STRING_LENGTH_PREFIX + CODENAME_LENGTH) * 2 // Item_codename, variant_id
		+ STRING_LENGTH_PREFIX + HASH_LENGTH // Variant_hash
		+ STRING_LENGTH_PREFIX + HASH_SIGNATURE_LENGTH // Variant_hash_signature
		+ PUBLIC_KEY_LENGTH // Author
		+ TIMESTAMP_LENGTH * 2; // Account_created, last_modified
}

#[error_code]
pub enum ErrorCode {
	#[msg("The variant id should be max 60 chracters long.")]
    InvalidVariantId,
	#[msg("The project id should be exactly 36 chracters long.")]
    InvalidProjectId,
	#[msg("The item codename should be max 60 chracters long.")]
    InvalidItemCodename,
	#[msg("The variant hash should be exactly 40 characters long.")]
    InvalidHash,
	#[msg("The variant hash signature should be exactly 96 characters long.")]
    InvalidHashSignature,
}