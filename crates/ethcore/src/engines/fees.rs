// TODO: implement the interface to call the fees contract

use ethabi;
use ethereum_types::{Address, U256};

use super::{SystemOrCodeCall, SystemOrCodeCallKind};
use hash::keccak;

use_contract!(fees_contract, "res/contracts/fees.json");

#[derive(PartialEq, Debug)]
pub struct FeesContract {
    kind: SystemOrCodeCallKind,
}

impl FeesContract {
    /// Create a new fees contract client targeting the system call kind.
    pub fn new(kind: SystemOrCodeCallKind) -> FeesContract {
        FeesContract { kind }
    }

    /// Create a new fees contract client targeting the contract address.
    pub fn new_from_addres(address: Address) -> FeesContract {
        Self::new(SystemOrCodeCallKind::Address(address))
    }

    /// Create a new fees contract client targeting the given code.
    pub fn new_from_code(code: Arc<Vec<u8>>) -> FeesContract {
        let code_hash = keccak(&code[..]);

        Self::new(SystemOrCodeCallKind::Code(code, code_hash))
    }

    /// Returns the address of the fees contract
    pub fn address(&self) -> Option<Address> {
        match self.kind {
            SystemOrCodeCallKind::Address(address) => Some(address),
            _ => None,
        }
    }

    pub fn get_gas_price(&self, caller: &mut SystemOrCodeCall) -> Result<U256> {
        let input = fees_contract::functions::get_gas_price::encode_input();

        let output = caller(self.kind.clone(), input)
            .map_err(Into::into)
            .map_err(::engines::EngineError::FailedSystemCall)?;

        let types = &[ParamType::Uint(256)];

        let tokens = ethabi::decode(types, &output)
            .map_err(|err| err.to_string())
            .map_err(::engines::EngineError::FailedSystemCall)?;

        assert!(tokens.len() == 1);

        let gas_price = tokens[0].into_uint()?;

        Ok(gas_price)
    }

    pub fn get_pay_address_and_fee_percent(
        &self,
        caller: &mut SystemOrCodeCall,
    ) -> Result<(Address, U256)> {
        let input = fees_contract::functions::get_pay_addres_and_fee_percent::encode_input();

        let output = caller(self.kind.clone(), input)
            .map_err(Into::into)
            .map_err(::engine::EngineError::FailedSystemCall)?;

        let types = &[ParamType::Address, ParamType::Uint(256)];

        let tokens = ethabi::decode(types, &output)
            .map_err(|err| err.to_string())
            .map_err(::engines::EngineError::FailedSystemCall)?;

        assert!(output.len() == 2);

        let address = tokens[0].into_address()?;
        let percent = tokens[1].into_uint()?;

        Ok((address, percent))
    }
}
