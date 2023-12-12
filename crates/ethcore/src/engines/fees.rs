//! Defining the fees contract module
//! The contract holding information about gas price and fees addressuse super::{SystemOrCodeCallKind};

use ethereum_types::{Address, U256};
use types::ids::BlockId;

use crate::client::BlockChainClient;
use ethabi::FunctionOutputDecoder;

use super::{SystemOrCodeCall, SystemOrCodeCallKind};
use error::Error;

use_contract!(fees_contract, "res/contracts/fees.json");

/// The interface arround fees contract
pub struct FeesContract {
    kind: SystemOrCodeCallKind,
}

impl FeesContract {
    /// Create a new fees contract client targeting the system call kind.
    pub fn new(kind: SystemOrCodeCallKind) -> FeesContract {
        FeesContract { kind }
    }

    /// Create a new fees contract client targeting the contract address.
    pub fn new_from_address(address: Address) -> FeesContract {
        Self::new(SystemOrCodeCallKind::Address(address))
    }

    /// Returns the addres of the fess contract.
    pub fn address(&self) -> Option<Address> {
        match self.kind {
            SystemOrCodeCallKind::Address(address) => Some(address),
            _ => None,
        }
    }

    /// Returns the current gas price
    pub fn get_gas_price(&self, caller: &mut SystemOrCodeCall) -> Result<U256, Error> {
        let input = fees_contract::functions::get_gas_price::encode_input();

        let output = caller(self.kind.clone(), input)
            .map_err(Into::into)
            .map_err(::engines::EngineError::FailedSystemCall)?;

        let price = fees_contract::functions::get_gas_price::decode_output(&output).unwrap();
        Ok(price)
    }

    /// Returns the params for the new transaction fee reward
    pub fn get_fees_params(
        &self,
        client: &dyn BlockChainClient,
        block_id: BlockId,
    ) -> Option<(Address, U256)> {
        match self.kind {
            SystemOrCodeCallKind::Address(address) => {
                let (data, decoder) = fees_contract::functions::get_fees_params::call();
                let value = client
                    .call_contract(block_id, address, data)
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
            _ => {
                error!(target: "fees", "Incompedible kind");
                None
            }
        }
    }
}
