use std::collections::{BTreeMap};
use serde::{Serialize, Deserialize};

use crate::contracts;
use crate::types::TxRef;
use crate::TransactionStatus;
use crate::contracts::{AccountIdWrapper};

use crate::std::string::String;
use crate::std::vec::Vec;
use core::str;

pub type TunaId = u32;

// Idea! 
//  from  tuna-app/tuna-chaincode.go

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tuna {
    vessel: String,
    timestamp: String,
    location: String,
    holder: String,
    id: u32
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TunaLedger {
    next_id: u32,
    tuna_list: BTreeMap<u32, Tuna>
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    Record {
        vessel: String,
        timestamp: String,
        location: String,
        holder: String,
    },
    Erase {
        id: TunaId,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Error {
    NotAuthorized,
    Other(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Request {
    QueryAll,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    QueryAll {
        tunas: Vec<Tuna>
    },
    Error(Error)
}

const ALICE: &'static str = "d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d";

impl TunaLedger {

    /// Initializes the contract
    pub fn new() -> Self {
        let mut tuna_list = BTreeMap::<u32, Tuna>::new();

        // Tuna{Vessel: "923F", Location: "67.0006, -70.5476", Timestamp: "1504054225", Holder: "Miriam"},
        let firstTuna = (String::from("923F"), String::from("67.0006, -70.5476"), String::from("1504054225"), String::from("Miriam"));

        let owner = AccountIdWrapper::from_hex(ALICE);
        let vessel = String::from(firstTuna.clone().0);
        let location = String::from(firstTuna.clone().1);
        let timestamp = String::from(firstTuna.clone().2);
        let holder = String::from(firstTuna.clone().3);

        let tunatuna = Tuna {
            vessel: vessel.clone(),
            location: location.clone(),
            timestamp: timestamp.clone(),
            holder: holder.clone(),
            id: 0
        };

        tuna_list.insert(0, tunatuna);

        TunaLedger { next_id: 1, tuna_list }
    }


}

impl contracts::Contract<Command, Request, Response> for TunaLedger {

    fn id(&self) -> contracts::ContractId { contracts::TUNA_LEDGER }

    fn handle_command(&mut self, _origin: &chain::AccountId, _txref: &TxRef, cmd: Command) -> TransactionStatus {
        match cmd {

            Command::Record { vessel, timestamp, location, holder } => {
                
                let current_user = AccountIdWrapper(_origin.clone());
                if let None = self.tuna_list.iter().find(|(_, tuna)| tuna.vessel == vessel) {

                    let id = self.next_id;
                    
                    let tunatuna = Tuna {
                        vessel: vessel.clone(),
                        timestamp: timestamp.clone(),
                        location: location.clone(),
                        holder: holder.clone(),
                        id
                    };

                    self.tuna_list.insert(id, tunatuna);
                    self.next_id += 1;

                    TransactionStatus::Ok
                } else {
                    TransactionStatus::TunaExist
                }

            },
            Command::Erase {id} => {
                let o = AccountIdWrapper(_origin.clone());

                if let Some(tuna) = self.tuna_list.get(&id) {
                    self.tuna_list.remove(&id);
                    TransactionStatus::Ok
                } else {
                    TransactionStatus::TunaIdNotFound
                }
            },
        }
    }

    fn handle_query(&mut self, _origin: Option<&chain::AccountId>, req: Request) -> Response {
        let inner = || -> Result<Response, Error> {
            match req {
                Request::QueryAll => {
                    Ok(Response::QueryAll { tunas: self.tuna_list.values().cloned().collect() })
                },    
            }
        };
        match inner() {
            Err(error) => Response::Error(error),
            Ok(resp) => resp
        }
    }
} 