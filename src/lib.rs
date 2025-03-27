mod idl;
mod pb;

use anchor_lang::AnchorDeserialize;
use anchor_lang::Discriminator;
use base64::prelude::*;
use pb::substreams::v1::program::Data;
use pb::substreams::v1::program::ClaimEventEvent;
use pb::substreams::v1::program::DepositEventEvent;
use pb::substreams::v1::program::PoolInitializedEventEvent;
use pb::substreams::v1::program::RewardAddedEventEvent;
use pb::substreams::v1::program::UpdatedDistributorEventEvent;
use pb::substreams::v1::program::UpdatedDurationEventEvent;
use pb::substreams::v1::program::WithdrawEventEvent;
use pb::substreams::v1::program::AddRewardInstruction;
use pb::substreams::v1::program::ClaimInstruction;
use pb::substreams::v1::program::DepositInstruction;
use pb::substreams::v1::program::InitPoolInstruction;
use pb::substreams::v1::program::SetRewardsDistributorInstruction;
use pb::substreams::v1::program::SetRewardsDurationInstruction;
use pb::substreams::v1::program::WithdrawInstruction;












use sologger_log_context::programs_selector::ProgramsSelector;
use sologger_log_context::sologger_log_context::LogContext;
use substreams_solana::pb::sf::solana::r#type::v1::Block;

const PROGRAM_ID: &str = "BB9hUaLkTzWhzdVzi8BxjVD1CQuYMpqP3SiwQ5saAQ2W";

#[substreams::handlers::map]
fn map_program_data(blk: Block) -> Data {
    let mut claim_event_event_list: Vec<ClaimEventEvent> = Vec::new();
    let mut deposit_event_event_list: Vec<DepositEventEvent> = Vec::new();
    let mut pool_initialized_event_event_list: Vec<PoolInitializedEventEvent> = Vec::new();
    let mut reward_added_event_event_list: Vec<RewardAddedEventEvent> = Vec::new();
    let mut updated_distributor_event_event_list: Vec<UpdatedDistributorEventEvent> = Vec::new();
    let mut updated_duration_event_event_list: Vec<UpdatedDurationEventEvent> = Vec::new();
    let mut withdraw_event_event_list: Vec<WithdrawEventEvent> = Vec::new();
    let mut add_reward_instruction_list: Vec<AddRewardInstruction> = Vec::new();
    let mut claim_instruction_list: Vec<ClaimInstruction> = Vec::new();
    let mut deposit_instruction_list: Vec<DepositInstruction> = Vec::new();
    let mut init_pool_instruction_list: Vec<InitPoolInstruction> = Vec::new();
    let mut set_rewards_distributor_instruction_list: Vec<SetRewardsDistributorInstruction> = Vec::new();
    let mut set_rewards_duration_instruction_list: Vec<SetRewardsDurationInstruction> = Vec::new();
    let mut withdraw_instruction_list: Vec<WithdrawInstruction> = Vec::new();

    blk.transactions().for_each(|transaction| {

        // ------------- EVENTS -------------
        let meta_wrapped = &transaction.meta;
        let meta = meta_wrapped.as_ref().unwrap();
        let programs_selector: ProgramsSelector = ProgramsSelector::new(&["*".to_string()]);
        let log_contexts = LogContext::parse_logs_basic(&meta.log_messages, &programs_selector);

        log_contexts
            .iter()
            .filter(|context| context.program_id == PROGRAM_ID)
            .for_each(|context| {
                context.data_logs.iter().for_each(|data| {
                    if let Ok(decoded) = BASE64_STANDARD.decode(data) {
                        let slice_u8: &mut &[u8] = &mut &decoded[..];
                        let slice_discriminator: [u8; 8] =
                            slice_u8[0..8].try_into().expect("error");
                        let static_discriminator_slice: &'static [u8] = Box::leak(Box::new(slice_discriminator));

                        match static_discriminator_slice {
                            idl::idl::program::events::ClaimEvent::DISCRIMINATOR => {
                                if let Ok(event) =
                                    idl::idl::program::events::ClaimEvent::deserialize(
                                        &mut &slice_u8[8..],
                                    )
                                {
                                    claim_event_event_list.push(ClaimEventEvent {
                                        trx_hash: transaction.id(),
                                        pool: event.pool.to_string(),
                                        recipient: event.recipient.to_string(),
                                        amount: event.amount,
                                        timestamp: event.timestamp
                                    });
                                }
                            }
                            idl::idl::program::events::DepositEvent::DISCRIMINATOR => {
                                if let Ok(event) =
                                    idl::idl::program::events::DepositEvent::deserialize(
                                        &mut &slice_u8[8..],
                                    )
                                {
                                    deposit_event_event_list.push(DepositEventEvent {
                                        trx_hash: transaction.id(),
                                        pool: event.pool.to_string(),
                                        depositor: event.depositor.to_string(),
                                        amount: event.amount,
                                        timestamp: event.timestamp
                                    });
                                }
                            }
                            idl::idl::program::events::PoolInitializedEvent::DISCRIMINATOR => {
                                if let Ok(event) =
                                    idl::idl::program::events::PoolInitializedEvent::deserialize(
                                        &mut &slice_u8[8..],
                                    )
                                {
                                    pool_initialized_event_event_list.push(PoolInitializedEventEvent {
                                        trx_hash: transaction.id(),
                                        pool: event.pool.to_string(),
                                        mint: event.mint.to_string(),
                                        reward: event.reward.to_string(),
                                        distributor: event.distributor.to_string(),
                                        duration: event.duration,
                                        timestamp: event.timestamp
                                    });
                                }
                            }
                            idl::idl::program::events::RewardAddedEvent::DISCRIMINATOR => {
                                if let Ok(event) =
                                    idl::idl::program::events::RewardAddedEvent::deserialize(
                                        &mut &slice_u8[8..],
                                    )
                                {
                                    reward_added_event_event_list.push(RewardAddedEventEvent {
                                        trx_hash: transaction.id(),
                                        pool: event.pool.to_string(),
                                        contributor: event.contributor.to_string(),
                                        amount: event.amount,
                                        timestamp: event.timestamp
                                    });
                                }
                            }
                            idl::idl::program::events::UpdatedDistributorEvent::DISCRIMINATOR => {
                                if let Ok(event) =
                                    idl::idl::program::events::UpdatedDistributorEvent::deserialize(
                                        &mut &slice_u8[8..],
                                    )
                                {
                                    updated_distributor_event_event_list.push(UpdatedDistributorEventEvent {
                                        trx_hash: transaction.id(),
                                        pool: event.pool.to_string(),
                                        new_distributor: event.new_distributor.to_string(),
                                        timestamp: event.timestamp
                                    });
                                }
                            }
                            idl::idl::program::events::UpdatedDurationEvent::DISCRIMINATOR => {
                                if let Ok(event) =
                                    idl::idl::program::events::UpdatedDurationEvent::deserialize(
                                        &mut &slice_u8[8..],
                                    )
                                {
                                    updated_duration_event_event_list.push(UpdatedDurationEventEvent {
                                        trx_hash: transaction.id(),
                                        pool: event.pool.to_string(),
                                        duration: event.duration,
                                        timestamp: event.timestamp
                                    });
                                }
                            }
                            idl::idl::program::events::WithdrawEvent::DISCRIMINATOR => {
                                if let Ok(event) =
                                    idl::idl::program::events::WithdrawEvent::deserialize(
                                        &mut &slice_u8[8..],
                                    )
                                {
                                    withdraw_event_event_list.push(WithdrawEventEvent {
                                        trx_hash: transaction.id(),
                                        pool: event.pool.to_string(),
                                        withdrawer: event.withdrawer.to_string(),
                                        amount: event.amount,
                                        timestamp: event.timestamp
                                    });
                                }
                            }
                            _ => {}
                        }
                    }
                });
            });// ------------- INSTRUCTIONS -------------
        transaction
        .walk_instructions()
        .into_iter()
        .filter(|inst| inst.program_id().to_string() == PROGRAM_ID)
        .for_each(|inst| {
            let slice_u8: &[u8] = &inst.data()[..];
            if &slice_u8[0..8] == idl::idl::program::client::args::AddReward::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::AddReward::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    add_reward_instruction_list.push(AddRewardInstruction {
                        trx_hash: transaction.id(),
                        amount: instruction.amount,
                        acct_signer: accts[0].to_string(),
                        acct_mint: accts[1].to_string(),
                        acct_reward_mint: accts[2].to_string(),
                        acct_pool: accts[3].to_string(),
                        acct_pool_reward_token_account: accts[4].to_string(),
                        acct_user_reward_token_account: accts[5].to_string(),
                        acct_reward_token_program: accts[6].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::Claim::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::Claim::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    claim_instruction_list.push(ClaimInstruction {
                        trx_hash: transaction.id(),
                        acct_signer: accts[0].to_string(),
                        acct_mint: accts[1].to_string(),
                        acct_reward_mint: accts[2].to_string(),
                        acct_pool: accts[3].to_string(),
                        acct_pool_reward_token_account: accts[4].to_string(),
                        acct_user: accts[5].to_string(),
                        acct_user_reward_token_account: accts[6].to_string(),
                        acct_reward_token_program: accts[7].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::Deposit::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::Deposit::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    deposit_instruction_list.push(DepositInstruction {
                        trx_hash: transaction.id(),
                        amount: instruction.amount,
                        acct_signer: accts[0].to_string(),
                        acct_mint: accts[1].to_string(),
                        acct_reward_mint: accts[2].to_string(),
                        acct_pool: accts[3].to_string(),
                        acct_pool_token_account: accts[4].to_string(),
                        acct_user: accts[5].to_string(),
                        acct_user_token_account: accts[6].to_string(),
                        acct_user_reward_token_account: accts[7].to_string(),
                        acct_token_program: accts[8].to_string(),
                        acct_reward_token_program: accts[9].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::InitPool::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::InitPool::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    init_pool_instruction_list.push(InitPoolInstruction {
                        trx_hash: transaction.id(),
                        duration: instruction.duration,
                        acct_signer: accts[0].to_string(),
                        acct_mint: accts[1].to_string(),
                        acct_reward_mint: accts[2].to_string(),
                        acct_pool: accts[3].to_string(),
                        acct_pool_token_account: accts[4].to_string(),
                        acct_token_program: accts[5].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::SetRewardsDistributor::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::SetRewardsDistributor::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    set_rewards_distributor_instruction_list.push(SetRewardsDistributorInstruction {
                        trx_hash: transaction.id(),
                        acct_distributor: accts[0].to_string(),
                        acct_mint: accts[1].to_string(),
                        acct_reward_mint: accts[2].to_string(),
                        acct_pool: accts[3].to_string(),
                        acct_new_distributor: accts[4].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::SetRewardsDuration::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::SetRewardsDuration::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    set_rewards_duration_instruction_list.push(SetRewardsDurationInstruction {
                        trx_hash: transaction.id(),
                        duration: instruction.duration,
                        acct_distributor: accts[0].to_string(),
                        acct_mint: accts[1].to_string(),
                        acct_reward_mint: accts[2].to_string(),
                        acct_pool: accts[3].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::Withdraw::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::Withdraw::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    withdraw_instruction_list.push(WithdrawInstruction {
                        trx_hash: transaction.id(),
                        amount: instruction.amount,
                        acct_signer: accts[0].to_string(),
                        acct_mint: accts[1].to_string(),
                        acct_reward_mint: accts[2].to_string(),
                        acct_pool: accts[3].to_string(),
                        acct_pool_token_account: accts[4].to_string(),
                        acct_user: accts[5].to_string(),
                        acct_user_token_account: accts[6].to_string(),
                        acct_token_program: accts[7].to_string(),
                    });
                }
            }
        });
    });


    Data {
        claim_event_event_list,
        deposit_event_event_list,
        pool_initialized_event_event_list,
        reward_added_event_event_list,
        updated_distributor_event_event_list,
        updated_duration_event_event_list,
        withdraw_event_event_list,
        add_reward_instruction_list,
        claim_instruction_list,
        deposit_instruction_list,
        init_pool_instruction_list,
        set_rewards_distributor_instruction_list,
        set_rewards_duration_instruction_list,
        withdraw_instruction_list,
    }
}

























