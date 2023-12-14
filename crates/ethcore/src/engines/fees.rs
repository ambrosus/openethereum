//! Defining the fees contract module
//! The contract holding information about gas price and fees addressuse super::{SystemOrCodeCallKind};

use ethereum_types::{Address, U256};
use types::ids::BlockId;

use crate::client::BlockChainClient;
use ethabi::FunctionOutputDecoder;

use_contract!(fees_contract, "res/contracts/fees.json");

/// The interface arround fees contract
pub struct FeesContract {
    address: Address,
}

impl FeesContract {
    /// Create a new fees contract client targeting the system call kind.
    pub fn new(address: Address) -> FeesContract {
        FeesContract { address }
    }

    /// Returns the current gas price
    pub fn get_gas_price(&self, client: &dyn BlockChainClient, block_id: BlockId) -> Option<U256> {
        let (data, decoder) = fees_contract::functions::get_gas_price::call();
        let value = client
            .call_contract(block_id, self.address, data)
            .map_err(|err| {
                error!(target: "gas_price", "Failed to call the contract: {:?}", err);
            })
            .ok()?;
        if let Some(result) = decoder.decode(&value).ok() {
            Some(result)
        } else {
            error!(target: "gas_price", "Failed to decode the result");
            None
        }
    }

    /// Returns the params for the new transaction fee reward
    pub fn get_fees_params(
        &self,
        client: &dyn BlockChainClient,
        block_id: BlockId,
    ) -> Option<(Address, U256)> {
        let (data, decoder) = fees_contract::functions::get_fees_params::call();
        let value = client
            .call_contract(block_id, self.address, data)
            .map_err(|err| {
                error!(target: "fees", "Failed to call the contract: {:?}", err);
            })
            .ok()?;
        if let Some(result) = decoder.decode(&value).ok() {
            Some(result)
        } else {
            error!(target:  "fees", "Failed to decode the result");
            None
        }
    }
}
