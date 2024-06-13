#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub struct NewsData {
    pub title: String,
    pub url: String,
    pub body: String,
    pub source: String,
    pub tags: String
}
#[derive(Deserialize, Debug)]
pub struct NewsResponse {
    pub Data: Vec<NewsData>,
}