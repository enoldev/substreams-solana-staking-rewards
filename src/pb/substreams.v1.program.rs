// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Data {
    #[prost(message, repeated, tag="1")]
    pub claim_event_event_list: ::prost::alloc::vec::Vec<ClaimEventEvent>,
    #[prost(message, repeated, tag="2")]
    pub deposit_event_event_list: ::prost::alloc::vec::Vec<DepositEventEvent>,
    #[prost(message, repeated, tag="3")]
    pub pool_initialized_event_event_list: ::prost::alloc::vec::Vec<PoolInitializedEventEvent>,
    #[prost(message, repeated, tag="4")]
    pub reward_added_event_event_list: ::prost::alloc::vec::Vec<RewardAddedEventEvent>,
    #[prost(message, repeated, tag="5")]
    pub updated_distributor_event_event_list: ::prost::alloc::vec::Vec<UpdatedDistributorEventEvent>,
    #[prost(message, repeated, tag="6")]
    pub updated_duration_event_event_list: ::prost::alloc::vec::Vec<UpdatedDurationEventEvent>,
    #[prost(message, repeated, tag="7")]
    pub withdraw_event_event_list: ::prost::alloc::vec::Vec<WithdrawEventEvent>,
    #[prost(message, repeated, tag="8")]
    pub add_reward_instruction_list: ::prost::alloc::vec::Vec<AddRewardInstruction>,
    #[prost(message, repeated, tag="9")]
    pub claim_instruction_list: ::prost::alloc::vec::Vec<ClaimInstruction>,
    #[prost(message, repeated, tag="10")]
    pub deposit_instruction_list: ::prost::alloc::vec::Vec<DepositInstruction>,
    #[prost(message, repeated, tag="11")]
    pub init_pool_instruction_list: ::prost::alloc::vec::Vec<InitPoolInstruction>,
    #[prost(message, repeated, tag="12")]
    pub set_rewards_distributor_instruction_list: ::prost::alloc::vec::Vec<SetRewardsDistributorInstruction>,
    #[prost(message, repeated, tag="13")]
    pub set_rewards_duration_instruction_list: ::prost::alloc::vec::Vec<SetRewardsDurationInstruction>,
    #[prost(message, repeated, tag="14")]
    pub withdraw_instruction_list: ::prost::alloc::vec::Vec<WithdrawInstruction>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClaimEventEvent {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub pool: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub recipient: ::prost::alloc::string::String,
    #[prost(uint64, tag="4")]
    pub amount: u64,
    #[prost(uint64, tag="5")]
    pub timestamp: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DepositEventEvent {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub pool: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub depositor: ::prost::alloc::string::String,
    #[prost(uint64, tag="4")]
    pub amount: u64,
    #[prost(uint64, tag="5")]
    pub timestamp: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolInitializedEventEvent {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub pool: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub mint: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub reward: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub distributor: ::prost::alloc::string::String,
    #[prost(uint64, tag="6")]
    pub duration: u64,
    #[prost(uint64, tag="7")]
    pub timestamp: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RewardAddedEventEvent {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub pool: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub contributor: ::prost::alloc::string::String,
    #[prost(uint64, tag="4")]
    pub amount: u64,
    #[prost(uint64, tag="7")]
    pub timestamp: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdatedDistributorEventEvent {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub pool: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub new_distributor: ::prost::alloc::string::String,
    #[prost(uint64, tag="4")]
    pub timestamp: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdatedDurationEventEvent {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub pool: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub duration: u64,
    #[prost(uint64, tag="4")]
    pub timestamp: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WithdrawEventEvent {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub pool: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub withdrawer: ::prost::alloc::string::String,
    #[prost(uint64, tag="4")]
    pub amount: u64,
    #[prost(uint64, tag="5")]
    pub timestamp: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddRewardInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub amount: u64,
    #[prost(string, tag="3")]
    pub acct_signer: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub acct_mint: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub acct_reward_mint: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_pool: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub acct_pool_reward_token_account: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub acct_user_reward_token_account: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub acct_reward_token_program: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClaimInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub acct_signer: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub acct_mint: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub acct_reward_mint: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub acct_pool: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_pool_reward_token_account: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub acct_user: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub acct_user_reward_token_account: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub acct_reward_token_program: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DepositInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub amount: u64,
    #[prost(string, tag="3")]
    pub acct_signer: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub acct_mint: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub acct_reward_mint: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_pool: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub acct_pool_token_account: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub acct_user: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub acct_user_token_account: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub acct_user_reward_token_account: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub acct_token_program: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub acct_reward_token_program: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitPoolInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub duration: u64,
    #[prost(string, tag="3")]
    pub acct_signer: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub acct_mint: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub acct_reward_mint: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_pool: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub acct_pool_token_account: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub acct_token_program: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRewardsDistributorInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub acct_distributor: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub acct_mint: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub acct_reward_mint: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub acct_pool: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_new_distributor: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRewardsDurationInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub duration: u64,
    #[prost(string, tag="3")]
    pub acct_distributor: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub acct_mint: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub acct_reward_mint: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_pool: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WithdrawInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub amount: u64,
    #[prost(string, tag="3")]
    pub acct_signer: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub acct_mint: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub acct_reward_mint: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_pool: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub acct_pool_token_account: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub acct_user: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub acct_user_token_account: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub acct_token_program: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
