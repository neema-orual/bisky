use bisky::atproto::{ClientBuilder, UserSession};
use bisky::bluesky::Bluesky;
use bisky::storage::{File, Storage as _};
use clap::Parser;
use std::path::PathBuf;
use url::Url;
use std::sync::Arc;
use serde_json::json;


#[tokio::main]
async fn main() {
    let storage = Arc::new(File::<UserSession>::new(PathBuf::from("./storage.json")));
    let mut client = ClientBuilder::default().session(None).build().unwrap();

    client.login(&Url::parse("https://bsky.social").unwrap(), &"nsf.nonbinary.computer", &"brxm-ocdx-kuao-ccih")
    .await
    .unwrap();

    let mut bsky = Bluesky::new(client);
    let mut user = bsky.user(&"nonbinary.computer").unwrap();
    let profile = user.get_profile().await.unwrap();
    println!("Profile: {:#?}", json!(profile).to_string());
    /*
    let likes = user.get_likes(100, None).await.unwrap();
    println!("Likes: {:#?}", likes);
    let follows = user.get_follows(100, None).await.unwrap();
    println!("Follows: {:#?}", follows);
    let followers = user.get_followers(100, None).await.unwrap();
    println!("Followers: {:#?}", followers);
    let mut stream = user.stream_posts().await.unwrap();

    while let Ok(record) = stream.next().await {
        println!("{:#?}", record);
    }*/
}
