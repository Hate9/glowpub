mod fixture_generation;

use serde::{Deserialize, Serialize};
use std::fs::read_to_string;

use crate::{api::BoardPosts, Board, Post};

use super::{GlowficError, GlowficResponse, Replies};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

const ALL_POSTS: &str = "./src/api/tests/fixtures/api-posts.json";
const OK_POSTS: &str = "./src/api/tests/fixtures/api-posts-success.json";
const ERR_POSTS: &str = "./src/api/tests/fixtures/api-posts-error.json";

const ALL_REPLIES: &str = "./src/api/tests/fixtures/api-replies.json";
const OK_REPLIES: &str = "./src/api/tests/fixtures/api-replies-success.json";
const ERR_REPLIES: &str = "./src/api/tests/fixtures/api-replies-error.json";

const ALL_BOARDS: &str = "./src/api/tests/fixtures/api-boards.json";
const OK_BOARDS: &str = "./src/api/tests/fixtures/api-boards-success.json";
const ERR_BOARDS: &str = "./src/api/tests/fixtures/api-boards-error.json";

const ALL_BOARD_POSTS: &str = "./src/api/tests/fixtures/api-board_posts.json";
const OK_BOARD_POSTS: &str = "./src/api/tests/fixtures/api-board_posts-success.json";
const ERR_BOARD_POSTS: &str = "./src/api/tests/fixtures/api-board_posts-error.json";

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
struct Error {
    errors: Vec<GlowficError>,
}

#[tokio::test]
async fn deserialisation() -> Result<()> {
    let _posts: Vec<GlowficResponse<Post>> = serde_json::from_str(&read_to_string(ALL_POSTS)?)?;
    let _posts: Vec<Post> = serde_json::from_str(&read_to_string(OK_POSTS)?)?;
    let _posts: Vec<Error> = serde_json::from_str(&read_to_string(ERR_POSTS)?)?;

    let _replies: Vec<GlowficResponse<Replies>> =
        serde_json::from_str(&read_to_string(ALL_REPLIES)?)?;
    let _replies: Vec<Replies> = serde_json::from_str(&read_to_string(OK_REPLIES)?)?;
    let _replies: Vec<Error> = serde_json::from_str(&read_to_string(ERR_REPLIES)?)?;

    let _boards: Vec<GlowficResponse<Board>> = serde_json::from_str(&read_to_string(ALL_BOARDS)?)?;
    let _boards: Vec<Board> = serde_json::from_str(&read_to_string(OK_BOARDS)?)?;
    let _boards: Vec<Error> = serde_json::from_str(&read_to_string(ERR_BOARDS)?)?;

    let _board_posts: Vec<GlowficResponse<BoardPosts>> =
        serde_json::from_str(&read_to_string(ALL_BOARD_POSTS)?)?;
    let _board_posts: Vec<BoardPosts> = serde_json::from_str(&read_to_string(OK_BOARD_POSTS)?)?;
    let _board_posts: Vec<Error> = serde_json::from_str(&read_to_string(ERR_BOARD_POSTS)?)?;

    Ok(())
}
