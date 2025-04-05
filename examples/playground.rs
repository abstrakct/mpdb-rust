#[allow(unused_imports)]
use loco_rs::{cli::playground, prelude::*};

use mpdb::models::{_entities::sets, concerts};
#[allow(unused_imports)]
use mpdb::{
    app::App,
    models::_entities::{cities, countries, venues},
};
// use sha1::{Digest, Sha1};

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

    // let mut checksum = Sha1::new();
    // checksum.update("Norway".as_bytes());
    // let _result = checksum.finalize();
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

    // let res = venues::Entity::find()
    //     .find_also_related(cities::Entity)
    //     .and_also_related(countries::Entity)
    //     .all(&ctx.db)
    //     .await
    //     .unwrap();

    // let res = cities::Entity::find_by_country_id(&ctx.db, 1).await;
    // let res = cities::Entity::find_with_countries(&ctx.db).await;
    // let res = venues::Entity::find_by_city_slug(&ctx.db, "trondheim-norway").await;
    // let res = venues::Entity::find_all_with_concerts_in_date_range(
    //     &ctx.db,
    //     Date::from_str("2025-01-01").expect("wrong start date"),
    //     Date::from_str("2025-03-30").expect("wrong end date"),
    // )
    // .await;

    // let res = countries::Entity::find_all_venues(&ctx.db, 12).await;

    let res = venues::Model::find_by_name(&ctx.db, "UFFA").await;
    match res.as_ref() {
        Ok(r) => println!("{}", serde_json::to_string_pretty(&r).unwrap()),
        Err(e) => println!("ERROR: {}", e),
    }

    let res2 = res
        .as_ref()
        .unwrap()
        .city(&ctx.db)
        .await
        .unwrap()
        .venues(&ctx.db)
        .await;
    match res2 {
        Ok(r) => println!("{}", serde_json::to_string_pretty(&r).unwrap()),
        Err(e) => println!("ERROR: {}", e),
    }

    // let res3 = res.unwrap().concerts(&ctx.db).await;
    // match res3 {
    //     Ok(r) => println!("{}", serde_json::to_string_pretty(&r).unwrap()),
    //     Err(e) => println!("ERROR: {}", e),
    // }

    let set = sets::Entity::find_by_id(3152).one(&ctx.db).await?;
    let res = set.as_ref().unwrap().songs(&ctx.db).await;
    match res {
        Ok(r) => println!("{}", serde_json::to_string_pretty(&r).unwrap()),
        Err(e) => println!("ERROR: {}", e),
    }

    let concerts = concerts::Model::find_all_with_venue_and_artist(&ctx.db).await?;
    println!("{}", serde_json::to_string_pretty(&concerts[0]).unwrap());

    Ok(())
}
