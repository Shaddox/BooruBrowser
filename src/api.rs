use serde::Deserialize;

pub const API_BASE: &str = "https://danbooru.donmai.us";
pub const PAGE_LIMIT: usize = 20;

#[derive(Debug, Clone, Deserialize)]
pub struct Post {
    pub id: u64,
    pub rating: String,
    pub score: i64,
    #[serde(default)]
    pub tag_string: String,
    #[serde(default)]
    pub tag_string_character: String,
    #[serde(default)]
    pub tag_string_copyright: String,
    #[serde(default)]
    pub tag_string_artist: String,
    #[serde(default)]
    pub file_url: Option<String>,
    #[serde(default)]
    pub large_file_url: Option<String>,
    #[serde(default)]
    pub preview_file_url: Option<String>,
    #[serde(default)]
    pub image_width: Option<u64>,
    #[serde(default)]
    pub image_height: Option<u64>,
    #[serde(default)]
    pub file_ext: Option<String>,
}

impl Post {
    pub fn post_url(&self) -> String {
        format!("{API_BASE}/posts/{}", self.id)
    }

    pub fn best_image_url(&self) -> Option<&str> {
        self.large_file_url
            .as_deref()
            .or(self.file_url.as_deref())
            .or(self.preview_file_url.as_deref())
    }

    pub fn short_tags(&self) -> String {
        let mut tags = self
            .tag_string
            .split_whitespace()
            .take(8)
            .collect::<Vec<_>>()
            .join(" ");
        if self.tag_string.split_whitespace().count() > 8 {
            tags.push_str(" ...");
        }
        tags
    }
}
