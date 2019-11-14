// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

use crypto::digest::Digest;
use crypto::sha2::Sha512;

use std::str;
use std::collections::HashMap;

use sawtooth_sdk::processor::handler::ApplyError;
use sawtooth_sdk::processor::handler::TransactionContext;

pub fn get_dgc_prefix() -> String {
    let mut sha = Sha512::new();
    sha.input_str("dgc-wallet");
    sha.result_str()[..6].to_string()
}

//dgc-wallet State
pub struct DGCState<'a> {
    context: &'a mut TransactionContext,
}

impl<'a> DGCState<'a> {
    pub fn new(context: &'a mut TransactionContext) -> DGCState {
        DGCState {
            context: context,
        }
    }

    fn calculate_address(name: &str) -> String {
        let mut sha = Sha512::new();
        sha.input_str(name);
        get_dgc_prefix() + &sha.result_str()[..64].to_string()
    }
    
    pub fn get(&mut self, name: &str) -> Result<Option<u32>, ApplyError> {
        let address = DGCState::calculate_address(name);
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

    pub fn set(&mut self, name: &str, value: u32) -> Result<(), ApplyError> {
       
        let mut sets = HashMap::new();
        sets.insert(DGCState::calculate_address(name), value.to_string().into_bytes());
        self.context
            .set_state(sets)
            .map_err(|err| ApplyError::InternalError(format!("{}", err)))?;

        Ok(())
    }
}    