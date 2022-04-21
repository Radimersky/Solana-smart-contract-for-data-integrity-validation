use anchor_lang::prelude::*;

declare_id!("4C9v8ZmgtEbx8dDGb7YDXrsLbNswU3HAzhqBCKzUn21N");

#[program]
pub mod solana_kontent {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[account]
pub struct Variant {
	pub account_created: i64,
	pub last_mod_timestamp: i64,
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