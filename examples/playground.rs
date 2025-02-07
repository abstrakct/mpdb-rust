#[allow(unused_imports)]
use loco_rs::{cli::playground, prelude::*};
use mpdb::{app::App, models::_entities::countries};
use sha1::{Digest, Sha1};

#[tokio::main]
async fn main() -> loco_rs::Result<()> {
    let ctx = playground::<App>().await?;

    println!("welcome to playground. edit me at `examples/playground.rs`");

    // let active_model: countries::ActiveModel = countries::ActiveModel {
    //     name: Set(Some("Algulia".to_string())),
    //     uuid: Set(Uuid::parse_str("1a2b3c4d1111222233331234567890ff").unwrap()),
    //     code: Set(Some("AL".to_string())),
    //     ..Default::default()
    // };

    // active_model.insert(&ctx.db).await.unwrap();

    let mut checksum = Sha1::new();
    checksum.update("Norway".as_bytes());
    let _result = checksum.finalize();
    //let uuid = Builder::from_sha1_bytes(result.bytes());

    // let hex_hash = hex::encode(result);
    //println!("result: {:?}", result[..8].to_string());

    //    let uuid = format!(
    //        "{}-{}-4{}-{}{}-{}",
    //        &hex_hash[0..8],   // First 8 chars
    //        &hex_hash[8..12],  // Next 4 chars
    //        &hex_hash[13..16], // Set version to '4'
    //        (char::from_digit((hex_hash[16..17].parse::<u32>().unwrap() & 0x3) + 8, 16).unwrap()), // Variant
    //        &hex_hash[17..20], // Next 3 chars
    //        &hex_hash[20..32]  // Last 12 chars
    //    );

    //println!("result: {}", uuid);

    let res = countries::Entity::find().all(&ctx.db).await.unwrap();
    println!("{:?}", res);

    Ok(())
}
