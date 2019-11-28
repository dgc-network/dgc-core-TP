// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

use crypto::digest::Digest;
use crypto::sha2::Sha512;

use std::str;
use std::collections::HashMap;

use sawtooth_sdk::processor::handler::ApplyError;
use sawtooth_sdk::processor::handler::TransactionContext;

const FAMILY_NAME: &str = "dgc-core";
const FAMILY_VER: &str = "1.0";
const DGC_BALANCE: &str = "balance";
const DGC_CREDIT: &str  = "credit";
const DGC_EXCHANGE: &str= "exchange";
const APPLY_CREDIT: &str = "apply";
const TRANSFER_DGC: &str = "transfer";
const SELL_DGC: &str = "sell";
const BUY_DGC: &str = "buy";

pub fn get_dgc_prefix() -> String {
    let mut sha = Sha512::new();
    sha.input_str(FAMILY_NAME);
    sha.result_str()[..6].to_string()
}

pub fn hash(to_hash: &str, num: usize) -> String {
    let mut sha = Sha512::new();
    sha.input_str(to_hash);
    let temp = sha.result_str().to_string();
    let hash = match temp.get(..num) {
        Some(x) => x,
        None => "",
    };
    hash.to_string()
}

pub fn make_balance_state_address(identifier: &str) -> String {
    hash(FAMILY_NAME, 6) + &hash(DGC_BALANCE, 2) + &hash(identifier, 62)
}

pub fn make_exchange_state_address(currency: &str) -> String {
    hash(FAMILY_NAME, 6) + &hash(DGC_EXCHANGE, 2) + &hash(currency, 62)
}

pub fn make_credit_state_address(identifier: &str, currency: &str) -> String {
    hash(FAMILY_NAME, 6) + &hash(DGC_CREDIT, 2) + &hash(currency, 2) + &hash(identifier, 60)
}

pub fn make_sell_state_address(currency: &str, timestamp: &str) -> String {
    hash(FAMILY_NAME, 6) + &hash(SELL_DGC, 2) + &hash(currency, 2) + &hash(timestamp, 60)
}

pub fn make_buy_state_address(currency: &str, timestamp: &str) -> String {
    hash(FAMILY_NAME, 6) + &hash(BUY_DGC, 2) + &hash(currency, 2) + &hash(timestamp, 60)
}

//dgc-core State
pub struct DGCState<'a> {
    context: &'a mut TransactionContext,
}

impl<'a> DGCState<'a> {
    pub fn new(context: &'a mut TransactionContext) -> DGCState {
        DGCState {
            context: context,
        }
    }

    pub fn get_balance(&mut self, identifier: &str) -> Result<Option<u32>, ApplyError> {
        //let address = DGCState::calculate_address(name);
        let address = make_balance_state_address(identifier);
        let d = self.context.get_state(vec![address.clone()])?;
        match d {
            Some(packed) => {                
                
                let value_string = match String::from_utf8(packed) {
                    Ok(v) => v,
                    Err(_) => return Err(ApplyError::InvalidTransaction(String::from("Invalid UTF-8 sequence")))
                };                
                
                let value: u32 = match value_string.parse() {
                    Ok(v) => v,
                    Err(_) => return Err(ApplyError::InvalidTransaction(String::from("Unable to parse UTF-8 String as u32")))
                };
                
                Ok(Some(value))
                               
            }
            None => Ok(None),
        }
    }

    pub fn set_balance(&mut self, identifier: &str, value: u32) -> Result<(), ApplyError> {       
        let mut sets = HashMap::new();
        sets.insert(make_balance_state_address(identifier), value.to_string().into_bytes());
        self.context
            .set_state(sets)
            .map_err(|err| ApplyError::InternalError(format!("{}", err)))?;

        Ok(())
    }

    pub fn get_exchange(&mut self, currency: &str) -> Result<Option<u32>, ApplyError> {
        let address = make_exchange_state_address(currency);
        let d = self.context.get_state(vec![address.clone()])?;
        match d {
            Some(packed) => {                
                
                let value_string = match String::from_utf8(packed) {
                    Ok(v) => v,
                    Err(_) => return Err(ApplyError::InvalidTransaction(String::from("Invalid UTF-8 sequence")))
                };                
                
                let value: u32 = match value_string.parse() {
                    Ok(v) => v,
                    Err(_) => return Err(ApplyError::InvalidTransaction(String::from("Unable to parse UTF-8 String as u32")))
                };
                
                Ok(Some(value))
                               
            }
            None => Ok(None),
        }
    }

    pub fn set_exchange(&mut self, currency: &str, value: u32) -> Result<(), ApplyError> {       
        let mut sets = HashMap::new();
        sets.insert(make_exchange_state_address(currency), value.to_string().into_bytes());
        self.context
            .set_state(sets)
            .map_err(|err| ApplyError::InternalError(format!("{}", err)))?;

        Ok(())
    }

    pub fn get_credit(&mut self, identifier: &str, currency: &str) -> Result<Option<u32>, ApplyError> {
        let address = make_credit_state_address(identifier, currency);
        let d = self.context.get_state(vec![address.clone()])?;
        match d {
            Some(packed) => {                
                
                let value_string = match String::from_utf8(packed) {
                    Ok(v) => v,
                    Err(_) => return Err(ApplyError::InvalidTransaction(String::from("Invalid UTF-8 sequence")))
                };                
                
                let value: u32 = match value_string.parse() {
                    Ok(v) => v,
                    Err(_) => return Err(ApplyError::InvalidTransaction(String::from("Unable to parse UTF-8 String as u32")))
                };
                
                Ok(Some(value))
                               
            }
            None => Ok(None),
        }
    }

    pub fn set_credit(&mut self, identifier: &str, currency: &str, value: u32) -> Result<(), ApplyError> {       
        let mut sets = HashMap::new();
        sets.insert(make_credit_state_address(identifier, currency), value.to_string().into_bytes());
        self.context
            .set_state(sets)
            .map_err(|err| ApplyError::InternalError(format!("{}", err)))?;

        Ok(())
    }

    pub fn get_sell(&mut self, currency: &str, timestamp: &str) -> Result<Option<u32>, ApplyError> {
        let address = make_sell_state_address(currency, timestamp);
        let d = self.context.get_state(vec![address.clone()])?;
        match d {
            Some(packed) => {                
                
                let value_string = match String::from_utf8(packed) {
                    Ok(v) => v,
                    Err(_) => return Err(ApplyError::InvalidTransaction(String::from("Invalid UTF-8 sequence")))
                };                
                
                let value: u32 = match value_string.parse() {
                    Ok(v) => v,
                    Err(_) => return Err(ApplyError::InvalidTransaction(String::from("Unable to parse UTF-8 String as u32")))
                };
                
                Ok(Some(value))
                               
            }
            None => Ok(None),
        }
    }

    pub fn set_sell(&mut self, currency: &str, timestamp: &str, value: u32) -> Result<(), ApplyError> {       
        let mut sets = HashMap::new();
        sets.insert(make_sell_state_address(currency, timestamp), value.to_string().into_bytes());
        self.context
            .set_state(sets)
            .map_err(|err| ApplyError::InternalError(format!("{}", err)))?;

        Ok(())
    }

    pub fn get_buy(&mut self, currency: &str, timestamp: &str) -> Result<Option<u32>, ApplyError> {
        let address = make_buy_state_address(currency, timestamp);
        let d = self.context.get_state(vec![address.clone()])?;
        match d {
            Some(packed) => {                
                
                let value_string = match String::from_utf8(packed) {
                    Ok(v) => v,
                    Err(_) => return Err(ApplyError::InvalidTransaction(String::from("Invalid UTF-8 sequence")))
                };                
                
                let value: u32 = match value_string.parse() {
                    Ok(v) => v,
                    Err(_) => return Err(ApplyError::InvalidTransaction(String::from("Unable to parse UTF-8 String as u32")))
                };
                
                Ok(Some(value))
                               
            }
            None => Ok(None),
        }
    }

    pub fn set_buy(&mut self, currency: &str, timestamp: &str, value: u32) -> Result<(), ApplyError> {       
        let mut sets = HashMap::new();
        sets.insert(make_buy_state_address(currency, timestamp), value.to_string().into_bytes());
        self.context
            .set_state(sets)
            .map_err(|err| ApplyError::InternalError(format!("{}", err)))?;

        Ok(())
    }

}    