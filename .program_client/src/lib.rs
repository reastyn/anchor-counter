// DO NOT EDIT - automatically generated file (except `use` statements inside the `*_instruction` module
pub mod anchor_counter_instruction {
    use trdelnik_client::*;
    pub static PROGRAM_ID: Pubkey = Pubkey::new_from_array([
        101u8, 217u8, 78u8, 87u8, 14u8, 93u8, 172u8, 218u8, 32u8, 160u8, 245u8, 212u8, 8u8, 102u8,
        7u8, 237u8, 84u8, 168u8, 252u8, 199u8, 58u8, 21u8, 13u8, 162u8, 209u8, 234u8, 202u8, 224u8,
        101u8, 188u8, 22u8, 247u8,
    ]);
    pub async fn increment(
        client: &Client,
        parameters: anchor_counter::instruction::Increment,
        accounts: anchor_counter::accounts::Update,
        signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    ) -> Result<EncodedConfirmedTransactionWithStatusMeta, ClientError> {
        Ok(client
            .send_instruction(PROGRAM_ID, parameters, accounts, signers)
            .await?)
    }
    pub fn increment_ix(
        parameters: anchor_counter::instruction::Increment,
        accounts: anchor_counter::accounts::Update,
    ) -> Instruction {
        Instruction {
            program_id: PROGRAM_ID,
            data: parameters.data(),
            accounts: accounts.to_account_metas(None),
        }
    }
    pub async fn decrement(
        client: &Client,
        parameters: anchor_counter::instruction::Decrement,
        accounts: anchor_counter::accounts::Update,
        signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    ) -> Result<EncodedConfirmedTransactionWithStatusMeta, ClientError> {
        Ok(client
            .send_instruction(PROGRAM_ID, parameters, accounts, signers)
            .await?)
    }
    pub fn decrement_ix(
        parameters: anchor_counter::instruction::Decrement,
        accounts: anchor_counter::accounts::Update,
    ) -> Instruction {
        Instruction {
            program_id: PROGRAM_ID,
            data: parameters.data(),
            accounts: accounts.to_account_metas(None),
        }
    }
}
