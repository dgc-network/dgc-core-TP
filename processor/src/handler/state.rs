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
const DGC_BALANCE: &str = "ba";
const DGC_CREDIT: &str  = "ca";
const DGC_EXCHANGE: &str= "ec";
const APPLY_CREDIT: &str = "apply";
const TRANSFER_DGC: &str = "transfer";
const SELL_DGC: &str = "sell";
const BUY_DGC: &str = "buy";

pub fn get_dgc_prefix() -> String {
    let mut sha = Sha512::new();
    sha.input_str(&FAMILY_NAME);
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

pub fn make_balance_address(identifier: &str) -> String {
    get_dgc_prefix() + &DGC_BALANCE + &hash(identifier, 62)
}

pub fn make_credit_address(identifier: &str, currency: &str) -> String {
    get_dgc_prefix() + &DGC_CREDIT + &hash(currency, 2) + &hash(identifier, 60)
}

pub fn make_exchange_address(currency: &str) -> String {
    get_dgc_prefix() + &DGC_EXCHANGE + &hash(currency, 62)
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
/*
    fn calculate_address(name: &str) -> String {
        let mut sha = Sha512::new();
        sha.input_str(name);
        get_dgc_prefix() + &sha.result_str()[..64].to_string()
    }
*/    
    pub fn get_balance(&mut self, identifier: &str) -> Result<Option<u32>, ApplyError> {
        //let address = DGCState::calculate_address(name);
        let address = make_balance_address(identifier);
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
        //sets.insert(DGCState::calculate_address(name), value.to_string().into_bytes());
        sets.insert(make_balance_address(identifier), value.to_string().into_bytes());
        self.context
            .set_state(sets)
            .map_err(|err| ApplyError::InternalError(format!("{}", err)))?;

        Ok(())
    }

    pub fn get_exchange(&mut self, currency: &str) -> Result<Option<u32>, ApplyError> {
        //let address = DGCState::calculate_address(currency);
        let address = make_exchange_address(currency);
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
        //sets.insert(DGCState::calculate_address(currency), value.to_string().into_bytes());
        sets.insert(make_exchange_address(currency), value.to_string().into_bytes());
        self.context
            .set_state(sets)
            .map_err(|err| ApplyError::InternalError(format!("{}", err)))?;

        Ok(())
    }

    pub fn get_credit(&mut self, identifier: &str, currency: &str) -> Result<Option<u32>, ApplyError> {
        let address = make_credit_address(identifier, currency);
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
        sets.insert(make_credit_address(identifier, currency), value.to_string().into_bytes());
        self.context
            .set_state(sets)
            .map_err(|err| ApplyError::InternalError(format!("{}", err)))?;

        Ok(())
    }

}    