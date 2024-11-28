// Copyright 2015-2019 Parity Technologies (UK) Ltd.
// This file is part of Parity Ethereum.

// Parity Ethereum is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity Ethereum is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity Ethereum.  If not, see <http://www.gnu.org/licenses/>.

//! A client interface for interacting with the block gas limit contract.

use client::{BlockChainClient, BlockId};
use types::header::Header;
use ethabi::{self, ParamType};
use ethabi_contract::use_contract;
use ethereum_types::{Address};
use log::{debug, error};
use crate::executive::FeesParams;

use_contract!(contract, "res/contracts/fees.json");

pub fn block_tx_fee(full_client: &dyn BlockChainClient, header: &Header, address: Address) -> Option<FeesParams> {
    let (data, _decoder) = contract::functions::get_fees_params::call();
    let output = full_client.call_contract(BlockId::Hash(*header.parent_hash()), address, data).map_err(|err| {
        error!(target: "fees", "Contract call failed. Not changing the tx fee params. {:?}", err);
    }).ok()?;
    if output.is_empty() {
        debug!(target: "fees", "Contract call returned nothing. Not changing the tx fee params.");
        None
    } else {
        let types = &[
            ParamType::Address,
            ParamType::Uint(256),
        ];

        let tokens = ethabi::decode(types, &output).ok()?;

        assert!(tokens.len() == 2);

        let params = FeesParams {
            address: tokens[0].clone().to_address().unwrap(),
            governance_part: tokens[1].clone().to_uint().unwrap(),
        };
        Some(params)
    }
}