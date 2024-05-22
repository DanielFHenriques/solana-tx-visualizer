use crate::domain::account::Account;
use crate::domain::block::Block;
use crate::domain::program::Program;
use crate::domain::transaction::Transaction;
use solana_client::{rpc_client::RpcClient, rpc_config::RpcBlockConfig};
use solana_sdk::commitment_config::CommitmentConfig;
use solana_transaction_status::option_serializer::OptionSerializer;
use solana_transaction_status::{
    EncodedTransaction, TransactionDetails, UiInstruction, UiMessage, UiTransaction,
    UiTransactionEncoding, UiTransactionStatusMeta,
};
use std::collections::HashMap;

const USDC_MINT: &str = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";

pub struct BlockGatewayImpl {
    base_url: String,
}

impl BlockGatewayImpl {
    pub fn new<U: ToString>(base_url: U) -> Self {
        Self {
            base_url: base_url.to_string(),
        }
    }
}

fn get_accounts(meta: &UiTransactionStatusMeta) -> HashMap<u8, Account> {
    let mut accounts: HashMap<u8, Account> = HashMap::new();

    if let OptionSerializer::Some(pre_token_balances) = &meta.pre_token_balances {
        for balance in pre_token_balances {
            if balance.mint == USDC_MINT {
                if let OptionSerializer::Some(owner) = &balance.owner {
                    if let Some(pre_balance) = balance.ui_token_amount.ui_amount {
                        let account = Account::new(owner, balance.account_index, pre_balance);
                        accounts.insert(account.index, account);
                    }
                }
            }
        }
    }

    if let OptionSerializer::Some(post_token_balances) = &meta.post_token_balances {
        for account in accounts.values_mut() {
            for balance in post_token_balances {
                if account.index == balance.account_index {
                    if let Some(post_balance) = balance.ui_token_amount.ui_amount {
                        account.update_post_balance(post_balance);
                    } else {
                        println!(
                            "No pre balance found for account index: {}",
                            balance.account_index
                        )
                    }
                }
            }
        }
    }

    return accounts;
}

fn get_program(meta: &UiTransactionStatusMeta, transaction: &UiTransaction) -> Option<Program> {
    let mut addresses: Vec<String> = Vec::new();

    if let UiMessage::Raw(message) = &transaction.message {
        addresses.append(&mut message.account_keys.clone());
    }

    if let OptionSerializer::Some(loaded_addresses) = &meta.loaded_addresses {
        let load_addresses_clone = loaded_addresses.clone();

        addresses.append(&mut load_addresses_clone.writable.clone());
        addresses.append(&mut load_addresses_clone.readonly.clone());
    }

    if let OptionSerializer::Some(pre_token_balances) = &meta.pre_token_balances {
        for balance in pre_token_balances {
            if balance.mint == USDC_MINT {
                if let OptionSerializer::Some(program_id) = &balance.program_id {
                    let mut account_keys = addresses.iter();
                    let index = account_keys
                        .position(|account| account == program_id)
                        .unwrap();

                    return Some(Program::new(program_id, index as u8));
                }
            }
        }
    }

    return None;
}

fn get_account_pairs(
    program: &Program,
    meta: &UiTransactionStatusMeta,
    transaction: &UiTransaction,
) -> Vec<(u8, u8)> {
    let mut account_pairs: Vec<(u8, u8)> = Vec::new();

    if let UiMessage::Raw(message) = &transaction.message {
        let _ = &message.instructions.iter().for_each(|instruction| {
            if instruction.program_id_index == program.index && instruction.accounts.len() == 3 {
                let source = instruction.accounts.get(0).unwrap();
                let destination = instruction.accounts.get(1).unwrap();

                account_pairs.push((source.to_owned(), destination.to_owned()));
            }
        });
    }

    if let OptionSerializer::Some(inner_instructions) = &meta.inner_instructions {
        for inner_instruction in inner_instructions {
            for instruction in &inner_instruction.instructions {
                if let UiInstruction::Compiled(compiled) = &instruction {
                    if compiled.program_id_index == program.index && compiled.accounts.len() == 3 {
                        let source = compiled.accounts.get(0).unwrap();
                        let destination = compiled.accounts.get(1).unwrap();

                        account_pairs.push((source.to_owned(), destination.to_owned()));
                    }
                }
            }
        }
    }

    return account_pairs;
}

impl BlockGatewayImpl {
    pub fn get_block(&self, block: u64) -> Result<Block, String> {
        let client = RpcClient::new(&self.base_url);
        let rpc_block_config = RpcBlockConfig {
            transaction_details: Some(TransactionDetails::Full),
            commitment: Some(CommitmentConfig::finalized()),
            max_supported_transaction_version: Some(0),
            encoding: Some(UiTransactionEncoding::Json),
            rewards: Some(false),
        };

        match client.get_block_with_config(block, rpc_block_config) {
            Ok(block) => {
                let mut block_transations: Vec<Transaction> = Vec::new();
                let transactions = block.transactions.unwrap();

                for transaction_with_meta in transactions {
                    let meta = transaction_with_meta.meta.unwrap();

                    if let Err(_) = &meta.status {
                        continue;
                    }

                    if let EncodedTransaction::Json(transaction) =
                        &transaction_with_meta.transaction
                    {
                        let accounts_by_index = get_accounts(&meta);

                        if accounts_by_index.len() == 0 {
                            continue;
                        }

                        if let Some(program) = get_program(&meta, &transaction) {
                            let account_pairs = get_account_pairs(&program, &meta, &transaction);

                            if account_pairs.len() == 0 {
                                continue;
                            }

                            account_pairs.iter().for_each(|(source, destination)| {
                                let source_account = match accounts_by_index.get(source) {
                                    Some(account) => account.clone(),
                                    None => return,
                                };
                                let destination_account = match accounts_by_index.get(destination) {
                                    Some(account) => account.clone(),
                                    None => return,
                                };

                                let destination_transfer_amount = destination_account.post_balance
                                    - destination_account.pre_balance;

                                if destination_transfer_amount <= 0.0 {
                                    return;
                                }

                                let transaction = Transaction::new(
                                    transaction.signatures.get(0).unwrap().to_owned(),
                                    source_account,
                                    destination_account,
                                    program.clone(),
                                    destination_transfer_amount,
                                    USDC_MINT.to_owned(),
                                );
                                block_transations.push(transaction);
                            });
                        }
                    }
                }

                let b = Block::new(block.blockhash, block_transations);
                return Ok(b);
            }
            Err(e) => {
                return Err(e.to_string());
            }
        }
    }
}
