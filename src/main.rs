use std::env;
use web3::types::{BlockId, U64, BlockNumber};
use std::io::Write;
use std::fs::File;
use sqlx::prelude::*;
use sqlx::postgres::{PgConnection,PgCursor};
use sqlx::Pool;

#[tokio::main]
async fn main() -> web3::Result<()> {


    //单个连接
    let mut pool = Pool::<PgConnection>::new(&env::var("DATABASE_URL").unwrap()).await.unwrap();

    // 可以对连接池配置一些参数
    /*
    let pool = Pool::<PgConnection>::builder()
    .max_size(5)
    .build(&env::var("DATABASE_URL").unwrap()).await.unwrap();
    */
    let transport = web3::transports::Http::new("http://localhost:8540")?;
    let web3 = web3::Web3::new(transport);

    let mut block_num = web3.eth().block_number().await?;
    let block_str = block_num.to_string();
    let block_number = block_str.parse::<i64>().unwrap();

    let mut n: i64 = 1;

    println!("The number of the most recent block is : {:?}", block_num);
    //let mut block_data = web3.eth().block_with_txs(BlockId::Number(BlockNumber::Number(U64::from(block_number)))).await?;
    //println!("The {:?} block data is： {:?}", block_number, block_data);
    //let mut text = File::create("./test.txt").expect("create failed");
    //write!(text, "{:?}", block_data).expect("write failed");//Write into file.
    loop {
        let mut block_data = web3.eth().block_with_txs(BlockId::Number(BlockNumber::Number(U64::from(n)))).await?;
        if let Some(data) = &mut block_data {
            let mut eth_block = serde_json::to_value(data).unwrap();
            //let mut json = serde_json::to_string_pretty(&data).unwrap();
            //println!("{}", json);

            //block_head
            let hash = eth_block["hash"].as_str().unwrap();
            let parent_hash = eth_block["parentHash"].as_str().unwrap();
            let sha3_uncles = eth_block["sha3Uncles"].as_str().unwrap();
            let miner = eth_block["miner"].as_str().unwrap();
            let state_root = eth_block["stateRoot"].as_str().unwrap();
            let transactions_root = eth_block["transactionsRoot"].as_str().unwrap();
            let receipts_root = eth_block["receiptsRoot"].as_str().unwrap();
            let number = eth_block["number"].as_str().unwrap();
            let gas_used = eth_block["gasUsed"].as_str().unwrap();
            let gas_limit = eth_block["gasLimit"].as_str().unwrap();
            let base_fee_per_gas = "null";
            let extra_data = eth_block["extraData"].as_str().unwrap();
            let logs_bloom = eth_block["logsBloom"].as_str().unwrap();
            let timestamp = eth_block["timestamp"].as_str().unwrap();
            let difficulty = eth_block["difficulty"].as_str().unwrap();
            let total_difficulty = eth_block["totalDifficulty"].as_str().unwrap();
            // let seal_fields = "[]";TODO
            // let uncles = "[]";TODO
            let tx = eth_block["transactions"].as_array().unwrap();
            let transactions = tx.len().to_string();
            let size = eth_block["size"].as_str().unwrap();
            let mix_hash = "null";
            let nonce = "null";

            //block_tx
            for i in tx {
                // println!("{}", i);//Get single tx data.

                let hash_tx = i["hash"].as_str().unwrap();
                let nonce = i["nonce"].as_str().unwrap();
                let block_hash = i["blockHash"].as_str().unwrap();
                let block_number = i["blockNumber"].as_str().unwrap();
                let transaction_index = i["transactionIndex"].as_str().unwrap();
                let from_addr = i["from"].as_str().unwrap();
                let to_addr = i["to"].as_str().unwrap();
                let value = i["value"].as_str().unwrap();
                let gas_price = i["gasPrice"].as_str().unwrap();
                let gas = i["gas"].as_str().unwrap();
                let input = i["input"].as_str().unwrap();
                let v = i["v"].as_str().unwrap();
                let r = i["r"].as_str().unwrap();
                let s = i["s"].as_str().unwrap();
                let raw = i["raw"].as_str().unwrap();

                let sql = "INSERT INTO tx VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15)";
                let mut count = sqlx::query(sql).bind(hash_tx).bind(nonce).bind(block_hash).bind(block_number).bind(transaction_index).bind(from_addr)
                    .bind(to_addr).bind(value).bind(gas_price).bind(gas).bind(input).bind(v).bind(r).bind(s)
                    .bind(raw).execute(&pool).await.unwrap();
                //println!("Add {} records to the Tx table successfully! ", count);
            }

            let sql2 = "INSERT INTO header VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15,$16,$17,$18,$19,$20)";
            let count2 = sqlx::query(sql2).bind(hash).bind(parent_hash).bind(sha3_uncles).bind(miner).bind(state_root).bind(transactions_root)
                .bind(receipts_root).bind(number).bind(gas_used).bind(gas_limit).bind(base_fee_per_gas).bind(extra_data).bind(logs_bloom).bind(timestamp)
                .bind(difficulty).bind(total_difficulty).bind(transactions).bind(size).bind(mix_hash).bind(nonce).execute(&pool).await.unwrap();
            //println!("Add {} records to the Head table successfully! ", count2);
        }

        if n == block_number {
            break;
        }
        n+=1;
    }
    println!("{} blocks data has been imported",n);
    Ok(())
}
