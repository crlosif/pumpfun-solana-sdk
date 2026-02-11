#![forbid(unsafe_code)]

use pumpfun_sdk_idl::generated::pump_amm_min;
use solana_client::rpc_client::RpcClient;
use solana_commitment_config::CommitmentConfig;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signature, Signer},
    transaction::Transaction,
};

#[derive(Debug, thiserror::Error)]
pub enum PumpswapClientError {
    #[error("rpc error: {0}")]
    Rpc(#[from] solana_client::client_error::ClientError),
}

pub struct PumpswapClient {
    rpc: RpcClient,
    commitment: CommitmentConfig,
}

impl PumpswapClient {
    pub fn new(rpc_url: impl Into<String>) -> Self {
        Self {
            rpc: RpcClient::new(rpc_url.into()),
            commitment: CommitmentConfig::confirmed(),
        }
    }

    pub fn with_commitment(mut self, c: CommitmentConfig) -> Self {
        self.commitment = c;
        self
    }

    pub fn rpc(&self) -> &RpcClient {
        &self.rpc
    }

    pub fn simulate_buy(
        &self,
        payer: &Keypair,
        pumpswap_program_id: Pubkey,
        accounts: pump_amm_min::buy::Accounts,
        args: pump_amm_min::buy::Args,
    ) -> Result<solana_client::rpc_response::RpcSimulateTransactionResult, PumpswapClientError> {
        let ix = pump_amm_min::buy::build_ix(pumpswap_program_id, accounts, args);
        let bh = self.rpc.get_latest_blockhash()?;
        let tx = Transaction::new_signed_with_payer(&[ix], Some(&payer.pubkey()), &[payer], bh);
        Ok(self.rpc.simulate_transaction(&tx)?.value)
    }

    pub fn send_buy(
        &self,
        payer: &Keypair,
        pumpswap_program_id: Pubkey,
        accounts: pump_amm_min::buy::Accounts,
        args: pump_amm_min::buy::Args,
    ) -> Result<Signature, PumpswapClientError> {
        let ix = pump_amm_min::buy::build_ix(pumpswap_program_id, accounts, args);
        let bh = self.rpc.get_latest_blockhash()?;
        let tx = Transaction::new_signed_with_payer(&[ix], Some(&payer.pubkey()), &[payer], bh);
        Ok(self
            .rpc
            .send_and_confirm_transaction_with_spinner_and_commitment(&tx, self.commitment)?)
    }

    pub fn simulate_sell(
        &self,
        payer: &Keypair,
        pumpswap_program_id: Pubkey,
        accounts: pump_amm_min::sell::Accounts,
        args: pump_amm_min::sell::Args,
    ) -> Result<solana_client::rpc_response::RpcSimulateTransactionResult, PumpswapClientError> {
        let ix = pump_amm_min::sell::build_ix(pumpswap_program_id, accounts, args);
        let bh = self.rpc.get_latest_blockhash()?;
        let tx = Transaction::new_signed_with_payer(&[ix], Some(&payer.pubkey()), &[payer], bh);
        Ok(self.rpc.simulate_transaction(&tx)?.value)
    }

    pub fn send_sell(
        &self,
        payer: &Keypair,
        pumpswap_program_id: Pubkey,
        accounts: pump_amm_min::sell::Accounts,
        args: pump_amm_min::sell::Args,
    ) -> Result<Signature, PumpswapClientError> {
        let ix = pump_amm_min::sell::build_ix(pumpswap_program_id, accounts, args);
        let bh = self.rpc.get_latest_blockhash()?;
        let tx = Transaction::new_signed_with_payer(&[ix], Some(&payer.pubkey()), &[payer], bh);
        Ok(self
            .rpc
            .send_and_confirm_transaction_with_spinner_and_commitment(&tx, self.commitment)?)
    }
}
