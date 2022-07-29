use std::collections::HashMap;
use std::num::NonZeroU32;
use std::time::Duration;
use nonzero_ext::nonzero;
use async_trait::async_trait;
use log::{debug, error, warn};
use serde_json::Value;
use tokio_tungstenite::tungstenite::Message;
use crate::common::command_translator::CommandTranslator;
use crate::common::message_handler::{MessageHandler, MiscMessage};
use crate::common::ws_client_internal::WSClientInternal;
use crate::{
    clients::common_traits::{
        Candlestick, Level3OrderBook, OrderBook, OrderBookTopK, Ticker, Trade, BBO,
    },
    WSClient,
};
pub(crate) const EXCHANGE_NAME: &str = "blockchain";

const WEBSOCKET_URL: &str = "wss://ws.blockchain.info/mercury-gateway/v1/ws";


const UPLINK_LIMIT: (NonZeroU32, std::time::Duration) =
    (nonzero!(10u32), std::time::Duration::from_secs(1));

// Internal unified client
pub struct BlockchainWSClient {
    client: WSClientInternal<BlockchainMessageHandler>,
    translator: BlockchainCommandTranslator,
}
impl_new_constructor!(
    BlockchainWSClient,
    EXCHANGE_NAME,
    WEBSOCKET_URL,
    BlockchainMessageHandler {
        // channel_id_meta: HashMap::new()
    },
    BlockchainCommandTranslator {}
);

panic_bbo!(BlockchainWSClient);
panic_l2_topk!(BlockchainWSClient);
// panic_ticker!(BlockchainWSClient);
panic_candlestick!(BlockchainWSClient);
// panic_l3_orderbook!(BlockchainWSClient);


#[async_trait]
impl OrderBook for BlockchainWSClient {
    async fn subscribe_orderbook(&self, symbols: &[String]) {
       // msg = '{
        //   "action": "subscribe",
        //   "channel": "l2",
        //   "symbol": "BTC-USD"
        // }
        let commands = symbols
            .iter()
            .map(|symbol| {
                format!(r#"{{"action": "subscribe","channel": "l2","symbol": "{}"}}"#,
                        symbol,
                )
            })
            .collect::<Vec<String>>();
          println!("{:?}",commands);
        self.send(&commands).await;

    }
}
#[async_trait]
impl Trade for BlockchainWSClient {
    async fn subscribe_trade(&self, symbols: &[String]) {
        let commands = symbols
            .iter()
            .map(|symbol| {
                format!(r#"{{"action": "subscribe","channel": "trades","symbol": "{}"}}"#,
                        symbol,
                )
            })
            .collect::<Vec<String>>();

        self.send(&commands).await;
    }
}
#[async_trait]
impl Level3OrderBook for BlockchainWSClient{
    async fn subscribe_l3_orderbook(&self, symbols: &[String]) {
        let commands = symbols
            .iter()
            .map(|symbol| {
                format!(r#"{{"action": "subscribe","channel": "l3","symbol": "{}"}}"#,
                        symbol,
                )
            })
            .collect::<Vec<String>>();

        self.send(&commands).await;
        todo!()
    }
}
#[async_trait]
impl Ticker for BlockchainWSClient{
    async fn subscribe_ticker(&self, symbols: &[String]) {
        let commands = symbols
            .iter()
            .map(|symbol| {
                format!(r#"{{"action": "subscribe","channel": "ticker","symbol": "{}"}}"#,
                        symbol,
                )
            })
            .collect::<Vec<String>>();

        self.send(&commands).await;
        todo!()
    }
}
// #[async_trait]
// impl l3_orderbook for BlockchainWSClient {
//     async fn subscribe_trade(&self, symbols: &[String]) {
//         let commands = symbols
//             .iter()
//             .map(|symbol| {
//                 format!(r#"{{"action": "subscribe","channel": "l3","symbol": "{}"}}"#,
//                         symbol,
//                 )
//             })
//             .collect::<Vec<String>>();
//
//         self.send(&commands).await;
//     }
//
// }

impl_ws_client_trait!(BlockchainWSClient);

struct BlockchainMessageHandler {
    // channel_id_meta: HashMap<i64, String>, // CHANNEL_ID information
}
struct BlockchainCommandTranslator {}

impl MessageHandler for BlockchainMessageHandler {
    fn handle_message(&mut self, msg: &str) -> MiscMessage {
        println!("111111");
        if msg == "pong" {
            return MiscMessage::Pong;
        }
        let resp = serde_json::from_str::<HashMap<String, Value>>(msg);

        if resp.is_err() {
            error!("{} is not a JSON string, {}", msg, EXCHANGE_NAME);
            return MiscMessage::Other;
        }
        let obj = resp.unwrap();

        let event = obj.get("event").unwrap().as_str().unwrap();
        match event {
            "subscribed"  => {
                debug!("Received {} from {}", msg, EXCHANGE_NAME);
                MiscMessage::Normal
            }
            "unsubscribed"  => {
                warn!("Received {} from {}", msg, EXCHANGE_NAME);
                MiscMessage::Other
            }
            "rejected"  => {
                // error!("Received {} from {}", msg, EXCHANGE_NAME);
                // std::thread::sleep(Duration::from_secs(3));
                warn!("Received {} from {}", msg, EXCHANGE_NAME);
                MiscMessage::Other
            }
            "snapshot"  => {
                debug!("Received {} from {}", msg, EXCHANGE_NAME);
                MiscMessage::Normal
            }
            "updated" => {
                // warn!(
                //     "Received {}, which means Bitstamp is under maintenance",
                //     msg
                // );
                // std::thread::sleep(std::time::Duration::from_secs(20));
                debug!("Received {} from {}", msg, EXCHANGE_NAME);
                MiscMessage::Normal
            }
            _ =>{
                error!("Received {} from {}", msg, EXCHANGE_NAME);
                MiscMessage::Other
            }


        }

        // MiscMessage::Other
    }

    fn get_ping_msg_and_interval(&self) -> Option<(Message, u64)> {
        // todo!()
        None
    }
}

impl CommandTranslator for BlockchainCommandTranslator {
    fn translate_to_commands(&self, subscribe: bool, topics: &[(String, String)]) -> Vec<String> {
        // topics
        //     .iter()
        //     .map(|(channel, symbol)| Self::topic_to_command(channel, symbol, subscribe))
        //     .collect()
        let mut commands: Vec<String> = Vec::new();
        let n = topics.len();
        todo!()
        // let raw_channels = topics
        //     .iter()
        //     .map(|(channel, symbol)| format!("{}:{}", channel, symbol))
        //     .collect::<Vec<String>>();
        // format!(
        //     r#"{{"op":"{}","args":{}}}"#,
        //     if subscribe {
        //         "subscribe"
        //     } else {
        //         "unsubscribe"
        //     },
        //     serde_json::to_string(&raw_channels).unwrap()
        // )
    }

    fn translate_to_candlestick_commands(
        &self,
        subscribe: bool,
        symbol_interval_list: &[(String, usize)],
    ) -> Vec<String> {
        // symbol_interval_list
        //     .iter()
        //     .map(|(symbol, interval)| Self::to_candlestick_command(symbol, *interval, subscribe))
        //     .collect::<Vec<String>>()
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc;
    use crate::clients::blockchain::BlockchainWSClient;
    use crate::common::command_translator::CommandTranslator;
    use crate::WSClient;

    #[tokio::test(flavor = "multi_thread")]
    async fn test_spot_command() {
        let (tx, rx) = mpsc::channel();
        let block  = BlockchainWSClient::new(tx,None).await;
        block.subscribe_orderbook(&["BTC-USD".to_string()]).await;
        block.run().await;
        println!("111");
        // for temp in rx.recv(){
        //        println!("111{temp}");
        // }
    }
}



