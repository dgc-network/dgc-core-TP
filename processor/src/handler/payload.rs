// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

use protobuf;
use protobuf::Message;
use protobuf::RepeatedField;

use std::str;
use std::fmt;

use sawtooth_sdk::processor::handler::ApplyError;

#[derive(Copy, Clone)]
enum Action {
/*
    ApplyCredit(payload::ApplyCreditAction),
    TransferDGCoin(payload::TransferDGCoinAction),
    SellDGCoin(payload::SellDGCoinAction),
    BuyDGCoin(payload::BuyDGCoinAction),
*/    
    ApplyCredit,
    TransferDGCoin,
    SellDGCoin,
    BuyDGCoin,
}

pub struct DGCPayload {
    action: Action,
    timestamp: u64,
}
/*
pub enum Action {
    Deposit,
    Withdraw,
    Balance,
    Transfer,
    BalanceDGC,
    CreditDGC,
    ExchangeDGC,
    ApplyCredit,
    TransferDGCoin,
    SellDGCoin,
    BuyDGCoin,
}

pub struct DGCPayload {
    action: Action,    
    value: u32,
    beneficiary_pubkey: Option<String>,
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Action::Deposit => "Action::Deposit",
                Action::Withdraw => "Action::Withdraw",
                Action::Balance => "Action::Balance",
                Action::Transfer => "Action::Transfer",
                Action::BalanceDGC => "Action::BalanceDGC",
                Action::ExchangeDGC => "Action::ExchangeDGC",
                Action::CreditDGC => "Action::CreditDGC",
                Action::ApplyCredit => "Action::ApplyCredit",
                Action::TransferDGCoin => "Action::TransferDGCoin",
                Action::SellDGCoin => "Action::SellDGCoin",
                Action::BuyDGCoin => "Action::BuyDGCoin",
            }
        )
    }
}
*/
impl DGCPayload {

    pub fn new(payload: &[u8]) -> Result<Option<DGCPayload>, ApplyError> {
        //let payload: payload::DGCPayload = match protobuf::parse_from_bytes(payload) {
        let payload = match protobuf::parse_from_bytes(payload) {
            Ok(payload) => payload,
            Err(_) => {
                return Err(ApplyError::InvalidTransaction(String::from(
                    "Cannot deserialize payload",
                )))
            }
        };

        let dgc_core_action = payload.get_action();
        let action = match dgc_core_action {
            //payload::DGCPayload_Action::APPLY_CREDIT => {
            ApplyCredit => {
                Action::ApplyCredit(apply_credit.clone())
            }
            payload::DGCPayload_Action::TRANSFER_DG_COIN => {
                Action::TransferDGCoin(transfer_dg_coin.clone())
            }
            payload::DGCPayload_Action::SELL_DG_COIN => {
                Action::SellDGCoin(sell_dg_coin.clone())
            }
            payload::DGCPayload_Action::BUY_DG_COIN => {
                Action::BuyDGCoin(buy_dg_coin.clone())
            }
        };

        let timestamp = match payload.get_timestamp() {
            0 => {
                return Err(ApplyError::InvalidTransaction(String::from(
                    "Timestamp is not set",
                )))
            }
            x => x,
        };

        Ok(Some(DGCPayload {
            action: action,
            timestamp: timestamp,
        }))
    }

    pub fn get_action(&self) -> Action {
        self.action.clone()
    }

    pub fn get_timestamp(&self) -> u64 {
        self.timestamp
    }
/*
    pub fn new(payload_data: &[u8]) -> Result<Option<DGCPayload>, ApplyError> {
    
        let payload_string = match str::from_utf8(&payload_data) {
            Ok(s) => s,
            Err(_) => {
                return Err(ApplyError::InvalidTransaction(String::from(
                    "Invalid payload serialization",
                )))
            }
        };

        //dgc payload is constructed as comma separated items
        let items: Vec<&str> = payload_string.split(",").collect();

        if items.len() < 2 {
            return Err(ApplyError::InvalidTransaction(String::from(
                "Payload must have at least 1 comma",
            )));
        }
        
        if items.len() > 3 {
            return Err(ApplyError::InvalidTransaction(String::from(
                "Payload must have at most 2 commas",
            )));
        }
                
        let (action, value) = (items[0], items[1]);
        
        if action.is_empty() {
            return Err(ApplyError::InvalidTransaction(String::from(
                "Action is required",
            )));
        }
        
        let action = match action {
            "deposit" => Action::Deposit,
            "withdraw" => Action::Withdraw,
            "balance" => Action::Balance,
            "transfer" => Action::Transfer,
            "dgcBalance" => Action::BalanceDGC,
            "dgcCredit" => Action::CreditDGC,
            "dgcExchange" => Action::ExchangeDGC,
            "applyCredit" => Action::ApplyCredit,
            "transferDGC" => Action::TransferDGCoin,
            "sellDGC" => Action::SellDGCoin,
            "buyDGC" => Action::BuyDGCoin,
            _ => {
                return Err(ApplyError::InvalidTransaction(String::from(
                    "Invalid Action",
                )))
            }
        };
 
         
        let value: u32 = match value.parse() {
            Ok(num) => num,
            Err(_) => {
                return Err(ApplyError::InvalidTransaction(String::from(
                    "Missing integer value",
                )))
            }
        };        
        
        let mut beneficiary_pubkey = None;
         
        if items.len() == 3  {
                    
            if items[2].is_empty() {
                return Err(ApplyError::InvalidTransaction(String::from(
                    "Beneficiary cannot be empty ",
                )));
            }
            
            beneficiary_pubkey = Some(items[2].to_string());
            
        }        
        
        let payload = DGCPayload {
            action: action,
            value: value,
            beneficiary_pubkey: beneficiary_pubkey,
        };
        
        Ok(Some(payload))                       
    }
    
    pub fn get_action(&self) -> Action {
        self.action
    }
    
    pub fn get_value(&self) -> u32 {
        self.value
    }
    
    pub fn get_beneficiary(&self) -> &Option<String> {
        &self.beneficiary_pubkey      
    }

    pub fn get_currency(&self) -> &Option<String> {
        &self.beneficiary_pubkey      
    }
*/
}
