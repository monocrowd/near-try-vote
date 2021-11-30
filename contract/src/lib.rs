use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::serde_json::from_slice;
use near_sdk::{env, ext_contract, near_bindgen, setup_alloc, Promise, PromiseResult};
use std::collections::HashMap;

setup_alloc!();

type CandidateId = String;
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Candidate {
    candidate_id: CandidateId,
    metadata: Option<HashMap<String, String>>,
    votes: u128,
}

#[ext_contract(ext_voting)]
pub trait ExtVotingContract {
    fn view_candidates(&mut self) -> Vec<Candidate>;
}

#[ext_contract(ext_self)]
pub trait ContractCallback {
    fn try_vote_callback(&self, candidate_id: CandidateId) -> String;
}

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct TryVoteContract {}

#[near_bindgen]
impl TryVoteContract {
    pub fn try_vote(&mut self, candidate_id: CandidateId) -> Promise {
        // Inside a contract function on ContractA, a cross contract call is started
        // From ContractA to ContractB
        ext_voting::view_candidates(
            &"voting.happybits.testnet", // contract account id
            0,                           // yocto NEAR to attach
            5_000_000_000_000,           // gas to attach
        )
        // When the cross contract call from A to B finishes the my_callback method is triggered.
        // Since my_callback is a callback, it will have access to the returned data from B
        .then(ext_self::try_vote_callback(
            candidate_id,
            &env::current_account_id(), // this contract's account id
            0,                          // yocto NEAR to attach to the callback
            5_000_000_000_000,          // gas to attach to the callback
        ))
    }

    pub fn try_vote_callback(&self, candidate_id: CandidateId) -> String {
        assert_eq!(env::promise_results_count(), 1, "This is a callback method");

        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Failed => "oops!".to_string(),
            PromiseResult::Successful(result) => {
                let mut candidates = from_slice::<Vec<Candidate>>(&result).unwrap();
                candidates.sort_by(|a, b| b.votes.cmp(&a.votes));
                if candidates.len() == 0 {
                    String::from("you can be an candidate to win")
                } else if candidates.len() == 1 {
                    String::from("You have no choice")
                } else {
                    if candidates[0].votes == candidates[1].votes
                        && (candidates[0].candidate_id == candidate_id
                            || candidates[1].candidate_id == candidate_id)
                    {
                        String::from("game changing vote!!")
                    } else {
                        String::from("thanks vote")
                    }
                }
            }
        }
    }
}
