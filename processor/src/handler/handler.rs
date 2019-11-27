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
    fn dgc_balance(&self, state: &mut DGCState, customer_pubkey: &str) -> Result<u32, ApplyError>;
    fn dgc_exchange(&self, state: &mut DGCState, currency: &str) -> Result<u32, ApplyError>;
    fn dgc_credit(&self, state: &mut DGCState, customer_pubkey: &str, currency: &str) -> Result<u32, ApplyError>;
    fn apply_credit(&self, state: &mut DGCState, customer_pubkey: &str, currency: &str, credit_amount: u32) -> Result<(), ApplyError>;
    fn transfer_dgc(&self, state: &mut DGCState, customer_pubkey: &str, beneficiary_pubkey: &str, transfer_amount: u32) -> Result<(), ApplyError>;
    fn sell_dgc(&self, state: &mut DGCState, customer_pubkey: &str, sell_dgc_amount: u32, currency: &str, expected_sell_currency__amount: u32) -> Result<(), ApplyError>;
    fn buy_dgc(&self, state: &mut DGCState, customer_pubkey: &str, buy_dgc_amount: u32, currency: &str, expected_buy_currency_amount: u32) -> Result<(), ApplyError>;
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

            Action::BalanceDGC => {
                let current_balance: u32 = self.dgc_balance(&mut state, customer_pubkey)?;                                
                info!("current balance: {} ", current_balance);
            }
            
            Action::ExchangeDGC => {
                //Get currency from payload
                let currency =  match payload.get_currency() {
                    Some(v) => v.as_str(),
                    None => {
                        return Err(ApplyError::InvalidTransaction(String::from(
                            "Action: ExchangeDGC. currency doesn't exist.",
                        )))
                    }                    
                };
                
                let current_exchange: u32 = self.dgc_exchange(&mut state, currency)?;                                
                info!("current exchange: {} ", current_exchange);
            }
            
            Action::CreditDGC => {
                //Get currency from payload
                let currency =  match payload.get_currency() {
                    Some(v) => v.as_str(),
                    None => {
                        return Err(ApplyError::InvalidTransaction(String::from(
                            "Action: CreditDGC. currency doesn't exist.",
                        )))
                    }                    
                };
                
                let current_credit: u32 = self.dgc_credit(&mut state, customer_pubkey, currency)?;                                
                info!("current credit: {} ", current_credit);
            }
            
            Action::ApplyCredit => {
            
                //Get currency from payload
                let currency =  match payload.get_currency() {
                    Some(v) => v.as_str(),
                    None => {
                        return Err(ApplyError::InvalidTransaction(String::from(
                            "Action: CreditDGC. currency doesn't exist.",
                        )))
                    }                    
                };
                
                //Get apply credit amount
                let credit_amount = payload.get_value();
        
                self.apply_credit(&mut state, customer_pubkey, currency, credit_amount)?;                
            }                        

            Action::TransferDGC => {
            
                //Get beneficiary details from payload
                let beneficiary_pubkey =  match payload.get_beneficiary() {
                    Some(v) => v.as_str(),
                    None => {
                        return Err(ApplyError::InvalidTransaction(String::from(
                            "Action: TransferDGC. beneficiary account doesn't exist.",
                        )))
                    }                    
                };
                
                //Get transfer amount
                let transfer_amount = payload.get_value();
        
                self.transfer_dgc(&mut state, customer_pubkey, beneficiary_pubkey, transfer_amount)?;                
            }                        

            Action::SellDGC => {
            
                //Get currency from payload
                let currency =  match payload.get_currency() {
                    Some(v) => v.as_str(),
                    None => {
                        return Err(ApplyError::InvalidTransaction(String::from(
                            "Action: SellDGC. currency doesn't exist.",
                        )))
                    }                    
                };
                
                //Get sell amount
                let sell_amount = payload.get_value();
        
                self.sell_dgc(&mut state, customer_pubkey, sell_dgc_amount, currency, expected_sell_currency_amount)?;                
            }                        

            Action::BuyDGC => {
            
                //Get currency from payload
                let currency =  match payload.get_currency() {
                    Some(v) => v.as_str(),
                    None => {
                        return Err(ApplyError::InvalidTransaction(String::from(
                            "Action: BuyDGC. currency doesn't exist.",
                        )))
                    }                    
                };
                
                //Get sell amount
                let buy_amount = payload.get_value();
        
                self.buy_dgc(&mut state, customer_pubkey, buy_dgc_amount, currency, expected_buy_currency_amount)?;                
            }                        
        }

        Ok(())
    }    
}

impl DGCTransactions for DGCTransactionHandler {

    fn dgc_balance(&self, state: &mut DGCState, customer_pubkey: &str) -> Result<u32, ApplyError> {
    
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

    fn dgc_exchange(&self, state: &mut DGCState, currency: &str) -> Result<u32, ApplyError> {
    
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

    fn dgc_credit(&self, state: &mut DGCState, customer_pubkey: &str, currency: &str) -> Result<u32, ApplyError> {
    
        let current_credit: u32 = match state.get_exchange(currency) {
            Ok(Some(v)) => v,
            Ok(None) => {
                info!("Creating new currency for user.");
                0              
            }
            Err(err) => return Err(err),
        };
        
        Ok(current_credit)
    }

    fn apply_credit(&self, state: &mut DGCState, customer_pubkey: &str, currency: &str, credit_amount: u32) -> Result<(), ApplyError> {
                   
        //Get credit of customer
        let current_credit: u32 = self.dgc_credit(state, customer_pubkey, currency)?;                                        
                                
        //Store new credit to state
        state.set_credit(customer_pubkey, currency, current_credit + credit_amount)?;
                                     
        Ok(())
    
    }

    fn transfer_dgc(&self, state: &mut DGCState, customer_pubkey: &str, beneficiary_pubkey: &str, transfer_amount: u32) -> Result<(), ApplyError> {
                   
        //Get balance of customer
        let current_balance: u32 = self.dgc_balance(state, customer_pubkey)?;                                        
                                
        //Get beneficiary balance
        let beneficiary_balance: u32 = self.dgc_balance(state, beneficiary_pubkey)?;        
        
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

    fn sell_dgc(&self, state: &mut DGCState, customer_pubkey: &str, sell_dgc_amount: u32, currency: &str, expected_sell_currency_amount: u32) -> Result<(), ApplyError> {
                   
        //Get dgc balance of customer
        let current_dgc_balance: u32 = self.dgc_balance(state, customer_pubkey)?;                                        

        //Get dgc credit of customer
        let current_dgc_credit: u32 = self.dgc_credit(state, customer_pubkey, "DGC")?;                                        

        //Get exchange rate of currency
        let current_exchange: u32 = self.dgc_exchange(state, currency)?;                                        

        //sell amount should not be greater than current account balance + credit
        if sell_dgc_amount > (current_dgc_balance + current_dgc_credit){
            return Err(ApplyError::InvalidTransaction(String::from(
                "Action: Sell amount is more than customer account balance.",
            )))
        }
        
        //Store new balance to state
        //state.set_balance(customer_pubkey, current_balance - sell_amount)?;
        //Store new exchange rate to state --> imcomplete
        //state.set_exchange(currency, current_exchange)?;
                                     
        Ok(())
    
    }

    fn buy_dgc(&self, state: &mut DGCState, customer_pubkey: &str, buy_dgc_amount: u32, currency: &str, expected_buy_currency_amount: u32) -> Result<(), ApplyError> {
                   
        //Get dgc balance of customer
        let current_balance: u32 = self.dgc_balance(state, customer_pubkey)?;                                        

        //Get currency credit of customer
        let currency_credit: u32 = self.dgc_credit(state, customer_pubkey, currency)?;                                        

        //Get exchange rate of currency
        let current_exchange: u32 = self.dgc_exchange(state, currency)?;                                        

        //buy amount should not be greater than current account balance        
        if (buy_dgc_amount * current_exchange) > currency_credit {
            return Err(ApplyError::InvalidTransaction(String::from(
                "Action: Buy amount is more than customer account credit.",
            )))
        }
        
        //Store new balance to state
        //state.set_balance(customer_pubkey, current_balance + buy_amount)?;
        //Store new exchange rate to state --> imcomplete
        //state.set_exchange(currency, current_exchange)?;
                                     
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
    
