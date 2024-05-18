#![allow(dead_code)]
use std::fmt;
use reqwest::{Client, Error};
use std::env;
use dotenv::dotenv;
use std::collections::HashMap;
use serde::Deserialize;

const APP_ID: &str = "32002";
const _CLIENT_USERNAME: &str = "Priax";
const _CLIENT_ID: &str = "27370985";
const BASE_URL: &str = "https://osu.ppy.sh/api/v2/";

#[derive(Debug, Deserialize)]
struct User {
    username: String,
    id: u64,
    country_code: String,
    statistics: Statistics,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Username: {}\nID: {}\nCountry Code: {}\nStatistics:\n{}",
            self.username, self.id, self.country_code, self.statistics
        )
    }
}

impl fmt::Display for Statistics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "    Level: {} (Progress: {}%), Global Rank: {}, Country Rank: {}, PP: {}, Hit Accuracy: {}%",
                self.level.current,
                self.level.progress,
                self.global_rank,
                self.country_rank,
                self.pp,
                self.hit_accuracy
        )
    }
}

#[derive(Debug, Deserialize)]
struct Statistics {
    level: Level,
    global_rank: u64,
    country_rank: u64,
    pp: f64,
    hit_accuracy: f64,
}

#[derive(Debug, Deserialize)]
struct Level {
    current: u64,
    progress: u64,
}

#[derive(Debug, Deserialize)]
struct Beatmapset {
    beatmap_id: u64,
    count: u64,
    beatmap: Beatmap,
    beatmapset: BeatmapsetData,
}

#[derive(Debug, Deserialize)]
struct Beatmap {
    beatmapset_id: u64,
    difficulty_rating: f64,
    id: u64,
    mode: String,
    status: String,
    total_length: u64,
    user_id: u64,
    version: String,
}

#[derive(Debug, Deserialize)]
struct BeatmapsetData {
    artist: String,
    artist_unicode: String,
    creator: String,
    favourite_count: Option<u64>,
    hype: Option<String>,
    id: u64,
    nsfw: bool,
    offset: u64,
    play_count: u64,
    preview_url: String,
    source: String,
    spotlight: bool,
    status: String,
    title: String,
    title_unicode: String,
    track_id: Option<u64>,
    user_id: u64,
    video: bool,
}

// User scores
#[derive(Debug, Deserialize)]
struct Stats {
    count_100: u32,
    count_300: u32,
    count_50: u32,
    count_geki: Option<u32>,
    count_katu: Option<u32>,
    count_miss: u32,
}

#[derive(Debug, Deserialize)]
struct Covers {
    cover: String,
    #[serde(rename = "cover@2x")]
    cover2x: String,
    card: String,
    #[serde(rename = "card@2x")]
    card2x: String,
    list: String,
    #[serde(rename = "list@2x")]
    list2x: String,
    slimcover: String,
    #[serde(rename = "slimcover@2x")]
    slimcover2x: String,
}

#[derive(Debug, Deserialize)]
struct BeatmapInfo {
    beatmapset_id: u64,
    difficulty_rating: f64,
    id: u64,
    mode: String,
    status: String,
    total_length: u32,
    user_id: u64,
    version: String,
    accuracy: f64,
    ar: f64,
    bpm: f64,
    convert: bool,
    count_circles: u32,
    count_sliders: u32,
    count_spinners: u32,
    cs: f64,
    deleted_at: Option<String>,
    drain: f64,
    hit_length: u32,
    is_scoreable: bool,
    last_updated: String,
    mode_int: u32,
    passcount: u32,
    playcount: u32,
    ranked: u32,
    url: String,
    checksum: String,
}

#[derive(Debug, Deserialize)]
struct BeatmapSetInfo {
    artist: String,
    artist_unicode: String,
    covers: Covers,
    creator: String,
    favourite_count: u32,
    hype: Option<u32>,
    id: u64,
    nsfw: bool,
    offset: u32,
    play_count: u32,
    preview_url: String,
    source: String,
    spotlight: bool,
    status: String,
    title: String,
    title_unicode: String,
    track_id: Option<u32>,
    user_id: u64,
    video: bool,
}

#[derive(Debug, Deserialize)]
struct UserInfo {
    avatar_url: String,
    country_code: String,
    default_group: String,
    id: u64,
    is_active: bool,
    is_bot: bool,
    is_deleted: bool,
    is_online: bool,
    is_supporter: bool,
    last_visit: String,
    pm_friends_only: bool,
    profile_colour: Option<String>,
    username: String,
}

#[derive(Debug, Deserialize)]
struct CurrentUserAttributes {
    pin: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Weight {
    percentage: f64,
    pp: f64,
}

#[derive(Debug, Deserialize)]
struct Score {
    accuracy: Option<f64>,
    best_id: Option<u64>,
    created_at: String,
    id: u64,
    max_combo: u32,
    mode: String,
    mode_int: u32,
    mods: Vec<String>,
    passed: bool,
    perfect: bool,
    pp: Option<f64>,
    rank: String,
    replay: bool,
    score: u64,
    statistics: Stats,
    #[serde(rename = "type")]
    score_type: String,
    user_id: u64,
    current_user_attributes: CurrentUserAttributes,
    beatmap: BeatmapInfo,
    beatmapset: BeatmapSetInfo,
    user: UserInfo,
    weight: Option<Weight>,
}

#[derive(Debug, Deserialize)]
struct Scores {
    scores: Vec<Score>,
}
// !user_scores

struct AccessToken {
    access_token: String,
    user_id: String,
    client: Client,
}

// (๑・ω・๑)
impl AccessToken {
    async fn new(client_secret: &str, username_or_id: &str, cli: Client) -> Result<Self, Error> {
        match get_access_token(client_secret).await {
            Ok(token) => Ok(Self { access_token: token, user_id: username_or_id.to_string(), client: cli }),
            Err(err) => Err(err),
        }
    }

    async fn get_user_data(&self) -> Result<User, Error> {
        let endpoint = format!("{}users/{}", BASE_URL, self.user_id);

        let response = self.client
            .get(&endpoint)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .send()
            .await?;

        let body = response.text().await?;
        // println!("{}", body);

        let user: User = serde_json::from_str(&body).expect("Did not found the data");
        Ok(user)
    }

    async fn get_user_beatmaps(&self, user_id: u64, beatmap_type: &str, limit: Option<u64>, offset: Option<u64>) -> Result<Vec<Beatmapset>, Error> {
        let mut endpoint = format!("{}users/{}/beatmapsets/{}", BASE_URL, user_id, beatmap_type);

        if let Some(limit) = limit {
            endpoint.push_str(&format!("?limit={}", limit));
        }
        if let Some(offset) = offset {
            endpoint.push_str(&format!("&offset={}", offset));
        }

        let response = self.client
            .get(&endpoint)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .send()
            .await?;

        let body = response
            .text()
            .await?;
        // println!("{}\n\n", body);

        let beatmaps: Vec<Beatmapset> = serde_json::from_str(&body).expect("Failed to parse user beatmaps");
        Ok(beatmaps)
    }

    async fn get_user_scores(&self, user_id: u64, beatmap_type: &str, mode: &str, limit: Option<u64>, offset: Option<u64>, fails: Option<bool>) -> Result<Vec<Score>, Error> {
        let mut endpoint = format!("{}users/{}/scores/{}", BASE_URL, user_id, beatmap_type);

        let query_params = vec![
            ("limit", limit.map(|v| v.to_string())),
            ("offset", offset.map(|v| v.to_string())),
            ("include_fails", fails.map(|v| v.to_string())),
            ("mode", Some(mode.to_string())),
        ];

        let query_string: Vec<String> = query_params
            .into_iter()
            .filter_map(|(key, value)| value.map(|v| format!("{}={}", key, v)))
            .collect();

        if !query_string.is_empty() {
            endpoint.push_str("?");
            endpoint.push_str(&query_string.join("&"));
        }

        // println!("Requesting URL: {}", endpoint);

        let response = self.client
            .get(&endpoint)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .send()
            .await?;

        // println!("Response status: {}", response.status());

        let body = response.text().await?;

        // println!("Response body: {}\n\n", body);

        let scores: Vec<Score> = serde_json::from_str(&body).expect("Failed to parse user scores");
        Ok(scores)
    }
}

async fn get_access_token(client_secret: &str) -> Result<String, Error> {
    let client = Client::new();

    let mut params = HashMap::new();
    params.insert("client_id", APP_ID);
    params.insert("client_secret", client_secret);
    params.insert("grant_type", "client_credentials");
    params.insert("scope", "public");

    let response = client
        .post("https://osu.ppy.sh/oauth/token")
        .form(&params)
        .send()
        .await?;

    let body = response.json::<serde_json::Value>().await?;
    let access_token = body["access_token"].as_str().expect("Access token not found").to_owned();

    Ok(access_token)
}


#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();

    let client_secret = env::var("CLIENT_SECRET")
        .expect("Failed to get secret from environment");

    let access_token = AccessToken::new(&client_secret, "Priax", Client::new())
        .await
        .expect("Access token not found");

    let user_data = access_token.get_user_data()
        .await
        .expect("No user data found");

    println!("{}", user_data);
    let user_beatmaps = access_token.get_user_beatmaps(27370985, "most_played", Some(5), Some(1))
        .await
        .expect("Failed to get user beatmaps");

    let titles: Vec<String> = user_beatmaps.iter()
        .map(|beatmap| beatmap.beatmapset.title.clone())
        .collect();
    println!("{}", titles.join(", "));

    let user_scores = access_token.get_user_scores(27370985, "best", "osu", Some(5), None, Some(false))
        .await
        .expect("Failed to get user scores");

    for score in &user_scores {
        let accuracy = score.accuracy.map_or("N/A".to_string(), |acc| format!("{:.2}%", acc * 100.0));
        let pp = score.pp.map_or("N/A".to_string(), |pp| format!("{:.2}", pp));

        // println!("{:#?}\n\n", score);
        print!("Title: {}, ", score.beatmapset.title);
        println!(
            "Accuracy: {}, Max combo: {}, mode: {}, pp: {}, rank: {}, mods: {:?}",
            accuracy,
            score.max_combo,
            score.mode,
            pp,
            score.rank,
            score.mods,
        );
        // println!("Title_unicode: {}, Artist: {}, Artist_unicode: {}", score.beatmapset.title_unicode, score.beatmapset.artist, score.beatmapset.artist_unicode);
        // println!("{:?}\n", score.beatmap);
        // println!("{:?}\n", score.beatmapset);
    }

    Ok(())
}
