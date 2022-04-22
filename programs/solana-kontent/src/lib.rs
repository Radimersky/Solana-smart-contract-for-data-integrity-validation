use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;

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
	
		variant.author = *author.key; 
		variant.account_created = clock.unix_timestamp;
		variant.item_id = item_id;
		variant.variant_id = variant_id;
	
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
	pub account_created: i64,
	pub last_modified: i64,
	pub variant_id: String, // "dd1439d5-4ee2-4895-a4e4-5b0d9d8c754e"
	pub item_id: String,
	pub project_id: String,
	pub author: Pubkey,
	pub variant_hash: String,
	pub variant_hash_signature: String,
}

const PUBLIC_KEY_LENGTH: usize = 32;
const TIMESTAMP_LENGTH: usize = 8;
const IDENTIFIER_LENGTH: usize = 36;
const HASH_LENGTH: usize = 36;
const HASH_SIGNATURE_LENGTH: usize = 36;
const DISCRIMINATOR_LENGTH: usize = 8;

impl Variant { 
	const LEN: usize = DISCRIMINATOR_LENGTH 
		+ TIMESTAMP_LENGTH * 2
		+ IDENTIFIER_LENGTH * 3 // variant, item, project ids
		+ PUBLIC_KEY_LENGTH // Author. 
		+ HASH_LENGTH 
		+ HASH_SIGNATURE_LENGTH;
}

#[error_code]
pub enum ErrorCode {
    #[msg("The ID should be 36 characters long.")]
    TopicTooLong,
}