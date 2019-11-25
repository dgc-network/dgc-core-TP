// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

use sawtooth_sdk::messages::processor::TpProcessRequest;
use sawtooth_sdk::processor::handler::ApplyError;
use sawtooth_sdk::processor::handler::TransactionContext;
use sawtooth_sdk::processor::handler::TransactionHandler;

use handler::payload::DGCPayload;
use handler::payload::Action;
use handler::state::DGCState;
use handler::state::get_dgc_prefix;

pub struct DGCTransactionHandler {
    family_name: String,
    family_versions: Vec<String>,
    namespaces: Vec<String>,
}

//Transactions in dgc
trait DGCTransactions {
    fn deposit(&self, state: &mut DGCState, customer_pubkey: &str, deposit_amount: u32) -> Result<(), ApplyError>;
    fn withdraw(&self, state: &mut DGCState, customer_pubkey: &str, withdraw_amount: u32) -> Result<(), ApplyError>;
    fn transfer(&self, state: &mut DGCState, customer_pubkey: &str, beneficiary_pubkey: &str, transfer_amount: u32) -> Result<(), ApplyError>;
    fn balance(&self, state: &mut DGCState, customer_pubkey: &str) -> Result<u32, ApplyError>;
    fn dgcBalance(&self, state: &mut DGCState, customer_pubkey: &str) -> Result<u32, ApplyError>;
    fn transferDGC(&self, state: &mut DGCState, customer_pubkey: &str, beneficiary_pubkey: &str, transfer_amount: u32) -> Result<(), ApplyError>;
    fn dgcExchange(&self, state: &mut DGCState, currency: &str) -> Result<u32, ApplyError>;
    fn sellDGC(&self, state: &mut DGCState, customer_pubkey: &str, currency: &str, sell_amount: u32) -> Result<(), ApplyError>;
    fn buyDGC(&self, state: &mut DGCState, customer_pubkey: &str, currency: &str, buy_amount: u32) -> Result<(), ApplyError>;
}

impl DGCTransactionHandler {
    
    pub fn new() -> DGCTransactionHandler {
        DGCTransactionHandler {
            family_name: String::from("dgc-core"),
            family_versions: vec![String::from("1.0")],
            namespaces: vec![String::from(get_dgc_prefix().to_string())],
        }
    }         
        
}

impl TransactionHandler for DGCTransactionHandler {
    fn family_name(&self) -> String {
        self.family_name.clone()
    }

    fn family_versions(&self) -> Vec<String> {
        self.family_versions.clone()
    }

    fn namespaces(&self) -> Vec<String> {
        self.namespaces.clone()
    }

    fn apply(
        &self,
        request: &TpProcessRequest,
        context: &mut TransactionContext,
    ) -> Result<(), ApplyError> {
        let header = &request.header;
        let customer_pubkey = match &header.as_ref() {
            Some(s) => &s.signer_public_key,
            None => {
                return Err(ApplyError::InvalidTransaction(String::from(
                    "Invalid header",
                )))
            }
        };
        
        let payload = DGCPayload::new(request.get_payload());
        let payload = match payload {
            Err(e) => return Err(e),
            Ok(payload) => payload,
        };
        let payload = match payload {
            Some(x) => x,
            None => {
                return Err(ApplyError::InvalidTransaction(String::from(
                    "Request must contain a payload",
                )))
            }
        };
        
        let mut state = DGCState::new(context);
        
        info!(
            "payload: {} {}",
            payload.get_action(),           
            payload.get_value(),
        );

        match payload.get_action() {
           
            Action::Deposit => {            
                let deposit_amount = payload.get_value();
                self.deposit(&mut state, customer_pubkey, deposit_amount)?;                                             
            }
                
            Action::Withdraw => {            
                let withdraw_amount = payload.get_value();
                self.withdraw(&mut state, customer_pubkey, withdraw_amount)?;                                                 
            }
            
            Action::Balance => {             
                let current_balance: u32 = self.balance(&mut state, customer_pubkey)?;                                
                info!("current balance: {} ", current_balance);
            }
            
            Action::Transfer => {
            
                //Get beneficiary details from payload
                let beneficiary_pubkey =  match payload.get_beneficiary() {
                    Some(v) => v.as_str(),
                    None => {
                        return Err(ApplyError::InvalidTransaction(String::from(
                            "Action: Transfer. beneficiary account doesn't exist.",
                        )))
                    }                    
                };
                
                //Get transfer amount
                let transfer_amount = payload.get_value();
        
                self.transfer(&mut state, customer_pubkey, beneficiary_pubkey, transfer_amount)?;                
            }                        

            Action::dgcBalance => {
                let current_balance: u32 = self.dgcBalance(&mut state, customer_pubkey)?;                                
                info!("current balance: {} ", current_balance);
            }
            
            Action::transferDGC => {
            
                //Get beneficiary details from payload
                let beneficiary_pubkey =  match payload.get_beneficiary() {
                    Some(v) => v.as_str(),
                    None => {
                        return Err(ApplyError::InvalidTransaction(String::from(
                            "Action: transferDGC. beneficiary account doesn't exist.",
                        )))
                    }                    
                };
                
                //Get transfer amount
                let transfer_amount = payload.get_value();
        
                self.transferDGC(&mut state, customer_pubkey, beneficiary_pubkey, transfer_amount)?;                
            }                        

            Action::dgcExchange => {
                //Get currency from payload
                let currency =  match payload.get_currency() {
                    Some(v) => v.as_str(),
                    None => {
                        return Err(ApplyError::InvalidTransaction(String::from(
                            "Action: dgcExchange. currency doesn't exist.",
                        )))
                    }                    
                };
                
                let current_exchange: u32 = self.dgcExchange(&mut state, currency)?;                                
                info!("current exchange: {} ", current_exchange);
            }
            
            Action::sellDGC => {
            
                //Get currency from payload
                let currency =  match payload.get_currency() {
                    Some(v) => v.as_str(),
                    None => {
                        return Err(ApplyError::InvalidTransaction(String::from(
                            "Action: sellDGC. currency doesn't exist.",
                        )))
                    }                    
                };
                
                //Get sell amount
                let sell_amount = payload.get_value();
        
                self.sellDGC(&mut state, customer_pubkey, currency, sell_amount)?;                
            }                        

            Action::buyDGC => {
            
                //Get currency from payload
                let currency =  match payload.get_currency() {
                    Some(v) => v.as_str(),
                    None => {
                        return Err(ApplyError::InvalidTransaction(String::from(
                            "Action: buyDGC. currency doesn't exist.",
                        )))
                    }                    
                };
                
                //Get sell amount
                let buy_amount = payload.get_value();
        
                self.buyDGC(&mut state, customer_pubkey, currency, buy_amount)?;                
            }                        
        }

        Ok(())
    }    
}

impl DGCTransactions for DGCTransactionHandler {

    fn dgcBalance(&self, state: &mut DGCState, customer_pubkey: &str) -> Result<u32, ApplyError> {
    
        let current_balance: u32 = match state.get_balance(customer_pubkey) {
            Ok(Some(v)) => v,
            Ok(None) => {
                info!("Creating new account for user.");
                0              
            }
            Err(err) => return Err(err),
        };
        
        Ok(current_balance)
    }

    fn transferDGC(&self, state: &mut DGCState, customer_pubkey: &str, beneficiary_pubkey: &str, transfer_amount: u32) -> Result<(), ApplyError> {
                   
        //Get balance of customer
        let current_balance: u32 = self.dgcBalance(state, customer_pubkey)?;                                        
                                
        //Get beneficiary balance
        let beneficiary_balance: u32 = self.dgcBalance(state, beneficiary_pubkey)?;        
        
        //Transfer amount should not be greater than current account balance        
        if transfer_amount > current_balance {
            return Err(ApplyError::InvalidTransaction(String::from(
                "Action: Transfer amount is more than customer account balance.",
            )))
        }
        
        //Store new balance to state
        state.set_balance(customer_pubkey, current_balance - transfer_amount)?;
        state.set_balance(beneficiary_pubkey, beneficiary_balance + transfer_amount)?;
                                     
        Ok(())
    
    }

    fn dgcExchange(&self, state: &mut DGCState, currency: &str) -> Result<u32, ApplyError> {
    
        let current_exchange: u32 = match state.get_exchange(currency) {
            Ok(Some(v)) => v,
            Ok(None) => {
                info!("Creating new currency for user.");
                0              
            }
            Err(err) => return Err(err),
        };
        
        Ok(current_exchange)
    }

    fn sellDGC(&self, state: &mut DGCState, customer_pubkey: &str, currency: &str, sell_amount: u32) -> Result<(), ApplyError> {
                   
        //Get balance of customer
        let current_balance: u32 = self.dgcBalance(state, customer_pubkey)?;                                        

        //Get exchange rate of currency
        let current_exchange: u32 = self.dgcExchange(state, currency)?;                                        

        //sell amount should not be greater than current account balance        
        if sell_amount > current_balance {
            return Err(ApplyError::InvalidTransaction(String::from(
                "Action: Sell amount is more than customer account balance.",
            )))
        }
        
        //Store new balance to state
        state.set_balance(customer_pubkey, current_balance - sell_amount)?;
        //Store new exchange rate to state --> imcomplete
        state.set_exchange(currency, current_exchange)?;
                                     
        Ok(())
    
    }

    fn buyDGC(&self, state: &mut DGCState, customer_pubkey: &str, currency: &str, buy_amount: u32) -> Result<(), ApplyError> {
                   
        //Get balance of customer
        let current_balance: u32 = self.dgcBalance(state, customer_pubkey)?;                                        

        //Get exchange rate of currency
        let current_exchange: u32 = self.dgcExchange(state, currency)?;                                        

        //sell amount should not be greater than current account balance        
        //if sell_amount > current_balance {
        //    return Err(ApplyError::InvalidTransaction(String::from(
        //        "Action: Sell amount is more than customer account balance.",
        //    )))
        //}
        
        //Store new balance to state
        state.set_balance(customer_pubkey, current_balance + buy_amount)?;
        //Store new exchange rate to state --> imcomplete
        state.set_exchange(currency, current_exchange)?;
                                     
        Ok(())
    
    }

    fn balance(&self, state: &mut DGCState, customer_pubkey: &str) -> Result<u32, ApplyError> {
    
        let current_balance: u32 = match state.get_balance(customer_pubkey) {
            Ok(Some(v)) => v,
            Ok(None) => {
                info!("First time deposit. Creating new account for user.");
                0              
            }
            Err(err) => return Err(err),
        };
        
        Ok(current_balance)
    }

    fn deposit(&self, state: &mut DGCState, customer_pubkey: &str, deposit_amount: u32) -> Result<(), ApplyError> {
                   
        let current_balance: u32 = self.balance(state, customer_pubkey)?;
                      
        let new_balance = current_balance + deposit_amount;
        
        //Store new balance to state
        state.set_balance(customer_pubkey, new_balance)?;
        
        Ok(())
    
    }

    fn withdraw(&self, state: &mut DGCState, customer_pubkey: &str, withdraw_amount: u32) -> Result<(), ApplyError> {
                   
        let current_balance: u32 = self.balance(state, customer_pubkey)?;                    
        
        //Withdraw amount should not be greater than current account balance
        if withdraw_amount > current_balance {
            return Err(ApplyError::InvalidTransaction(String::from(
                "Action: Withdraw amount is more than account balance.",
            )))
        }
        
        //update balance
        let new_balance = current_balance - withdraw_amount;
        
        //Store new balance to state
        state.set_balance(customer_pubkey, new_balance)?;
        
        Ok(())
    
    }
    
    fn transfer(&self, state: &mut DGCState, customer_pubkey: &str, beneficiary_pubkey: &str, transfer_amount: u32) -> Result<(), ApplyError> {
                   
        //Get balance of customer
        let current_balance: u32 = self.balance(state, customer_pubkey)?;                                        
                                
        //Get beneficiary balance
        let beneficiary_balance: u32 = self.balance(state, beneficiary_pubkey)?;        
        
        //Transfer amount should not be greater than current account balance        
        if transfer_amount > current_balance {
            return Err(ApplyError::InvalidTransaction(String::from(
                "Action: Transfer amount is more than customer account balance.",
            )))
        }
        
        //Store new balance to state
        state.set_balance(customer_pubkey, current_balance - transfer_amount)?;
        state.set_balance(beneficiary_pubkey, beneficiary_balance + transfer_amount)?;
                                     
        Ok(())
    
    }

}
    
