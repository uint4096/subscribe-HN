use reqwest;
use serde::Deserialize;

pub struct HN;

#[derive(Deserialize)]
pub struct News {
    pub title: String,
    pub url: Option<String>,
    pub dead: Option<bool>,
    pub id: u32,
    pub text: Option<String>,
}

impl<'a> HN {
    const BASE_URL: &'a str = "https://hacker-news.firebaseio.com/v0/";
    const NEW_STORIES: &'a str = "newstories.json";
    const ITEM: &'a str = "item";

    pub async fn get_story_ids() -> Vec<u32> {
        match reqwest::get(format!("{}/{}", HN::BASE_URL, HN::NEW_STORIES)).await {
            Ok(response) => match response.json::<Vec<u32>>().await {
                Ok(ids) => return ids,
                Err(e) => panic!("Error while converting ids to JSON! Error: {e}"),
            },
            Err(e) => panic!("Unable to fetch story ids! Error: {e}"),
        }
    }

    pub async fn get_story(id: u32) -> News {
        match reqwest::get(format!(
            "{}/{}/{}.json",
            HN::BASE_URL,
            HN::ITEM,
            id.to_string()
        ))
        .await
        {
            Ok(response) => match response.json::<News>().await {
                Ok(news) => return news,
                Err(e) => panic!("Error while converting story to JSON! Error: {e}"),
            },
            Err(e) => panic!("Unable to fetch story! Error: {e}"),
        }
    }
}
