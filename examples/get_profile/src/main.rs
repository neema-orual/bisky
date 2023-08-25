use bisky::atproto::{Client, ClientBuilder, UserSession};
use bisky::bluesky::Bluesky;
use bisky::lexicon::app::bsky::feed::Post;
use bisky::storage::{File, Storage as _};
use clap::Parser;
use std::path::PathBuf;
use url::Url;
use std::sync::Arc;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Arguments {
    #[clap(index = 1)]
    storage: PathBuf,
    #[clap(index = 2)]
    service: Url,
    #[clap(index = 3)]
    username: String,
    #[clap(index = 4)]
    password: String,
    #[clap(index = 5)]
    query: String,
}

#[tokio::main]
async fn main() {
    let args = Arguments::parse();

    //let storage = Arc::new(File::<UserSession>::new(args.storage));

    let mut client= ClientBuilder::default().session(None).build().unwrap();
    client.login(&"https://bsky.social", &"nsf.nonbinary.computer", &"brxm-ocdx-kuao-ccih").await;
    let mut bsky = Bluesky::new(client);
    let mut me = bsky.me().unwrap();
    let post = me.get_post_thread(&"https://bsky.app/profile/shreyan.bsky.social/post/3k5is5uplem2w").await.unwrap();
    println!("Post\n{:#?}", post);

}
