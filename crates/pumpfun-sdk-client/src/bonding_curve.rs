#![forbid(unsafe_code)]

use pumpfun_sdk_core::{build_buy_ix, build_sell_ix, ids, Config};
use solana_client::rpc_client::RpcClient;
use solana_commitment_config::CommitmentConfig;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signature, Signer},
    transaction::Transaction,
};

#[derive(Debug, thiserror::Error)]
pub enum BondingCurveClientError {
    #[error("rpc error: {0}")]
    Rpc(#[from] solana_client::client_error::ClientError),
    #[error("account not found: {0}")]
    AccountNotFound(Pubkey),
    #[error("invalid account data: {0}")]
    InvalidData(String),
}

/// Fetched bonding curve state.
#[derive(Debug, Clone)]
pub struct BondingCurveState {
    pub virtual_token_reserves: u64,
    pub virtual_sol_reserves: u64,
    pub real_token_reserves: u64,
    pub real_sol_reserves: u64,
    pub token_total_supply: u64,
    pub complete: bool,
}

/// Client for Pump bonding curve: fetch accounts, simulate, send.
pub struct BondingCurveClient {
    rpc: RpcClient,
    commitment: CommitmentConfig,
}

impl BondingCurveClient {
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

    /// Fetch Global account and return fee_recipient.
    /// Layout: initialized(1) + authority(32) + fee_recipient(32) + ...
    pub fn fetch_fee_recipient(
        &self,
        pump_program_id: &Pubkey,
    ) -> Result<Pubkey, BondingCurveClientError> {
        let (global, _) = pumpfun_sdk_core::pda::pump_global(pump_program_id);
        let acc = self.rpc.get_account(&global).map_err(|_| {
            BondingCurveClientError::AccountNotFound(global)
        })?;
        let data = acc.data.as_slice();
        if data.len() < 65 {
            return Err(BondingCurveClientError::InvalidData(
                "Global account too short".into(),
            ));
        }
        // fee_recipient at offset 33 (1 + 32)
        let bytes: [u8; 32] = data[33..65]
            .try_into()
            .map_err(|_| BondingCurveClientError::InvalidData("fee_recipient slice".into()))?;
        Ok(Pubkey::new_from_array(bytes))
    }

    /// Fetch bonding curve account data.
    pub fn fetch_bonding_curve(
        &self,
        pump_program_id: &Pubkey,
        mint: &Pubkey,
    ) -> Result<BondingCurveState, BondingCurveClientError> {
        let (bonding_curve, _) = pumpfun_sdk_core::pda::pump_bonding_curve(pump_program_id, mint);
        let acc = self.rpc.get_account(&bonding_curve).map_err(|_| {
            BondingCurveClientError::AccountNotFound(bonding_curve)
        })?;
        let data = acc.data.as_slice();
        if data.len() < 41 {
            return Err(BondingCurveClientError::InvalidData(
                "BondingCurve account too short".into(),
            ));
        }
        Ok(BondingCurveState {
            virtual_token_reserves: u64::from_le_bytes(data[0..8].try_into().unwrap()),
            virtual_sol_reserves: u64::from_le_bytes(data[8..16].try_into().unwrap()),
            real_token_reserves: u64::from_le_bytes(data[16..24].try_into().unwrap()),
            real_sol_reserves: u64::from_le_bytes(data[24..32].try_into().unwrap()),
            token_total_supply: u64::from_le_bytes(data[32..40].try_into().unwrap()),
            complete: data[40] != 0,
        })
    }

    /// Simulate bonding curve buy.
    pub fn simulate_buy(
        &self,
        cfg: &Config,
        payer: &Keypair,
        mint: Pubkey,
        amount: u64,
        max_sol_cost: u64,
    ) -> Result<
        solana_client::rpc_response::RpcSimulateTransactionResult,
        BondingCurveClientError,
    > {
        let fee_recipient = self.fetch_fee_recipient(&cfg.pump_program_id)?;
        let (ix, _) = build_buy_ix(
            cfg,
            mint,
            payer.pubkey(),
            fee_recipient,
            amount,
            max_sol_cost,
            &ids::TOKEN_2022_PROGRAM_ID,
        )
        .map_err(|e| BondingCurveClientError::InvalidData(e.to_string()))?;
        let bh = self.rpc.get_latest_blockhash()?;
        let tx = Transaction::new_signed_with_payer(&[ix], Some(&payer.pubkey()), &[payer], bh);
        Ok(self.rpc.simulate_transaction(&tx)?.value)
    }

    /// Send and confirm bonding curve buy.
    pub fn send_buy(
        &self,
        cfg: &Config,
        payer: &Keypair,
        mint: Pubkey,
        amount: u64,
        max_sol_cost: u64,
    ) -> Result<Signature, BondingCurveClientError> {
        let fee_recipient = self.fetch_fee_recipient(&cfg.pump_program_id)?;
        let (ix, _) = build_buy_ix(
            cfg,
            mint,
            payer.pubkey(),
            fee_recipient,
            amount,
            max_sol_cost,
            &ids::TOKEN_2022_PROGRAM_ID,
        )
        .map_err(|e| BondingCurveClientError::InvalidData(e.to_string()))?;
        let bh = self.rpc.get_latest_blockhash()?;
        let tx = Transaction::new_signed_with_payer(&[ix], Some(&payer.pubkey()), &[payer], bh);
        Ok(self
            .rpc
            .send_and_confirm_transaction_with_spinner_and_commitment(&tx, self.commitment)?)
    }

    /// Simulate bonding curve sell.
    pub fn simulate_sell(
        &self,
        cfg: &Config,
        payer: &Keypair,
        mint: Pubkey,
        amount: u64,
        min_sol_output: u64,
    ) -> Result<
        solana_client::rpc_response::RpcSimulateTransactionResult,
        BondingCurveClientError,
    > {
        let fee_recipient = self.fetch_fee_recipient(&cfg.pump_program_id)?;
        let (ix, _) = build_sell_ix(
            cfg,
            mint,
            payer.pubkey(),
            fee_recipient,
            amount,
            min_sol_output,
            &ids::TOKEN_2022_PROGRAM_ID,
        )
        .map_err(|e| BondingCurveClientError::InvalidData(e.to_string()))?;
        let bh = self.rpc.get_latest_blockhash()?;
        let tx = Transaction::new_signed_with_payer(&[ix], Some(&payer.pubkey()), &[payer], bh);
        Ok(self.rpc.simulate_transaction(&tx)?.value)
    }

    /// Send and confirm bonding curve sell.
    pub fn send_sell(
        &self,
        cfg: &Config,
        payer: &Keypair,
        mint: Pubkey,
        amount: u64,
        min_sol_output: u64,
    ) -> Result<Signature, BondingCurveClientError> {
        let fee_recipient = self.fetch_fee_recipient(&cfg.pump_program_id)?;
        let (ix, _) = build_sell_ix(
            cfg,
            mint,
            payer.pubkey(),
            fee_recipient,
            amount,
            min_sol_output,
            &ids::TOKEN_2022_PROGRAM_ID,
        )
        .map_err(|e| BondingCurveClientError::InvalidData(e.to_string()))?;
        let bh = self.rpc.get_latest_blockhash()?;
        let tx = Transaction::new_signed_with_payer(&[ix], Some(&payer.pubkey()), &[payer], bh);
        Ok(self
            .rpc
            .send_and_confirm_transaction_with_spinner_and_commitment(&tx, self.commitment)?)
    }
}
