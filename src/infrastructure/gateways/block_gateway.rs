use crate::application::gateways::block_gateway::BlockGateway;
use crate::domain::account::Account;
use crate::domain::block::Block;
use crate::domain::program::Program;
use crate::domain::transaction::Transaction;
use anyhow::Result;
use futures_util::{Future, StreamExt};
use solana_client::rpc_response::SlotUpdate;
use solana_client::{rpc_client::RpcClient, rpc_config::RpcBlockConfig};
use solana_pubsub_client::nonblocking::pubsub_client::PubsubClient;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_transaction_status::option_serializer::OptionSerializer;
use solana_transaction_status::{
    EncodedTransaction, TransactionDetails, UiInstruction, UiMessage, UiTransaction,
    UiTransactionEncoding, UiTransactionStatusMeta,
};
use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::mpsc::UnboundedSender;

const USDC_MINT: &str = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";

#[derive(Clone)]
pub struct BlockGatewayImpl {
    rpc_url: String,
    websocket_url: String,
}

impl BlockGatewayImpl {
    pub fn new<U: ToString>(cluster: U) -> Self {
        Self {
            rpc_url: format!("https://api.{}.solana.com", cluster.to_string()),
            websocket_url: format!("wss://api.{}.solana.com/", cluster.to_string()),
        }
    }

    fn get_accounts(&self, meta: &UiTransactionStatusMeta) -> HashMap<u8, Account> {
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
                        }
                    }
                }
            }
        }

        return accounts;
    }

    fn get_program(
        &self,
        meta: &UiTransactionStatusMeta,
        transaction: &UiTransaction,
    ) -> Option<Program> {
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
        &self,
        program: &Program,
        meta: &UiTransactionStatusMeta,
        transaction: &UiTransaction,
    ) -> Vec<(u8, u8)> {
        let mut account_pairs: Vec<(u8, u8)> = Vec::new();

        if let UiMessage::Raw(message) = &transaction.message {
            let _ = &message.instructions.iter().for_each(|instruction| {
                if instruction.program_id_index == program.index && instruction.accounts.len() == 3
                {
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
                        if compiled.program_id_index == program.index
                            && compiled.accounts.len() == 3
                        {
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
}

impl BlockGateway for BlockGatewayImpl {
    async fn subscribe(
        self: Arc<Self>,
        ready_sender: &UnboundedSender<()>,
        unsubscribe_sender: &UnboundedSender<
            Box<dyn FnOnce() -> Pin<Box<dyn Future<Output = ()> + Send>> + Send>,
        >,
        block_update_sender: &UnboundedSender<Block>,
    ) -> Result<()> {
        let pubsub_client = Arc::new(PubsubClient::new(&self.websocket_url.as_str()).await?);

        tokio::spawn({
            let ready_sender = ready_sender.clone();
            let unsubscribe_sender = unsubscribe_sender.clone();
            let block_update_sender = block_update_sender.clone();
            let pubsub_client = Arc::clone(&pubsub_client);

            async move {
                let (mut slot_updates_notifications, slot_updates_unsubscribe) =
                    pubsub_client.slot_updates_subscribe().await?;

                // With the subscription started,
                // send a signal back to the main task for synchronization.
                ready_sender.send(()).expect("channel");

                // Send the unsubscribe closure back to the main task.
                unsubscribe_sender
                    .send(slot_updates_unsubscribe)
                    .map_err(|e| format!("{}", e))
                    .expect("channel");

                drop((ready_sender, unsubscribe_sender));

                while let Some(slot_info) = slot_updates_notifications.next().await {
                    if let SlotUpdate::Completed { slot, timestamp: _ } = slot_info {
                        if let Ok(block) = self.get_block(slot) {
                            block_update_sender.send(block).expect("channel");
                        } else {
                            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                        }
                    }
                }

                Ok::<_, anyhow::Error>(())
            }
        });

        Ok(())
    }

    fn get_block(&self, block: u64) -> Result<Block, String> {
        let client = RpcClient::new(&self.rpc_url);
        let rpc_block_config = RpcBlockConfig {
            transaction_details: Some(TransactionDetails::Full),
            commitment: Some(CommitmentConfig::finalized()),
            max_supported_transaction_version: Some(0),
            encoding: Some(UiTransactionEncoding::Json),
            rewards: Some(false),
        };

        match client.get_block_with_config(block, rpc_block_config) {
            Ok(confirmed_block) => {
                let mut block = Block::new(block, confirmed_block.blockhash);
                let transactions = confirmed_block.transactions.unwrap();

                for transaction_with_meta in transactions {
                    let meta = transaction_with_meta.meta.unwrap();

                    if let Err(_) = &meta.status {
                        continue;
                    }

                    if let EncodedTransaction::Json(transaction) =
                        &transaction_with_meta.transaction
                    {
                        let accounts_by_index = self.get_accounts(&meta);

                        if accounts_by_index.len() == 0 {
                            continue;
                        }

                        if let Some(program) = self.get_program(&meta, &transaction) {
                            let account_pairs =
                                self.get_account_pairs(&program, &meta, &transaction);

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

                                let transaction = Transaction::new(
                                    transaction.signatures.get(0).unwrap().to_owned(),
                                    source_account,
                                    destination_account,
                                    program.clone(),
                                    USDC_MINT.to_owned(),
                                );

                                block.add_transaction(transaction);
                            });
                        }
                    }
                }

                return Ok(block);
            }
            Err(e) => {
                return Err(e.to_string());
            }
        }
    }
}
