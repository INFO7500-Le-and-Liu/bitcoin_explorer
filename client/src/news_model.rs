#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub struct NewsData {
    pub id: String,
    pub title: String,
    pub url: String,
    pub body: String,
    pub source: String,
    pub tags: String
}
#[derive(Deserialize, Debug)]
pub struct NewsResponse {
    #[serde(rename = "Data")]
    pub data: Vec<NewsData>,
}