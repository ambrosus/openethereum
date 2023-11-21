// TODO: implement the interface to call the fees contract

use ethabi::{self, ParamType};
use ethereum_types::{Address, U256};

use super::{SystemOrCodeCall, SystemOrCodeCallKind};
use error::Error;
use hash::keccak;
use std::sync::Arc;

use_contract!(fees_contract, "res/contracts/fees.json");
