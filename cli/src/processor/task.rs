use cronos_sdk::Client;
use solana_sdk::pubkey::Pubkey;

use crate::cli::CliError;

pub fn get(client: &Client, address: &Pubkey) -> Result<(), CliError> {
    let task = client
        .get::<cronos_sdk::scheduler::state::Task>(&address)
        .map_err(|_err| CliError::AccountDataNotParsable(address.to_string()))?;
    println!("{:#?}", task);
    Ok(())
}
