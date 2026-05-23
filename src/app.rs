use std::time::Duration;

use anyhow::{Context, Result};

use crate::api::{API_BASE, PAGE_LIMIT, Post};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Focus {
    Search,
    Results,
}

#[derive(Debug)]
pub struct App {
    pub query: String,
    pub page: usize,
    pub posts: Vec<Post>,
    pub selected: usize,
    pub focus: Focus,
    pub status: String,
    pub loading: bool,
    pub should_quit: bool,
    pub show_help: bool,
    client: reqwest::Client,
}

impl App {
    pub fn new() -> Result<Self> {
        let client = reqwest::Client::builder()
            .user_agent("booru-browser/0.1 (+https://danbooru.donmai.us)")
            .timeout(Duration::from_secs(20))
            .build()
            .context("failed to create HTTP client")?;

        Ok(Self {
            query: "rating:safe".to_string(),
            page: 1,
            posts: Vec::new(),
            selected: 0,
            focus: Focus::Search,
            status: "Enter tags, then press Enter to search.".to_string(),
            loading: false,
            should_quit: false,
            show_help: false,
            client,
        })
    }

    pub async fn search(&mut self) {
        self.loading = true;
        self.status = format!("Loading page {}...", self.page);

        let encoded_tags = urlencoding::encode(self.query.trim());
        let url = format!(
            "{API_BASE}/posts.json?tags={encoded_tags}&page={}&limit={PAGE_LIMIT}",
            self.page
        );

        match self.client.get(url).send().await {
            Ok(response) => match response.error_for_status() {
                Ok(response) => match response.json::<Vec<Post>>().await {
                    Ok(posts) => {
                        self.posts = posts;
                        self.selected = self.selected.min(self.posts.len().saturating_sub(1));
                        self.status = if self.posts.is_empty() {
                            "No posts found for this search.".to_string()
                        } else {
                            format!("Loaded {} posts from Danbooru.", self.posts.len())
                        };
                        self.focus = Focus::Results;
                    }
                    Err(error) => {
                        self.status = format!("Could not parse Danbooru response: {error}")
                    }
                },
                Err(error) => self.status = format!("Danbooru returned an error: {error}"),
            },
            Err(error) => self.status = format!("Request failed: {error}"),
        }

        self.loading = false;
    }

    pub fn selected_post(&self) -> Option<&Post> {
        self.posts.get(self.selected)
    }

    pub fn next(&mut self) {
        if !self.posts.is_empty() {
            self.selected = (self.selected + 1).min(self.posts.len() - 1);
        }
    }

    pub fn previous(&mut self) {
        self.selected = self.selected.saturating_sub(1);
    }

    pub async fn next_page(&mut self) {
        self.page += 1;
        self.search().await;
    }

    pub async fn previous_page(&mut self) {
        if self.page > 1 {
            self.page -= 1;
            self.search().await;
        }
    }

    pub fn open_selected_post(&mut self) {
        let Some(post) = self.selected_post() else {
            self.status = "No post selected.".to_string();
            return;
        };

        match webbrowser::open(&post.post_url()) {
            Ok(_) => self.status = format!("Opened post {}.", post.id),
            Err(error) => self.status = format!("Could not open browser: {error}"),
        }
    }

    pub fn open_selected_image(&mut self) {
        let Some(post) = self.selected_post() else {
            self.status = "No post selected.".to_string();
            return;
        };

        let Some(url) = post.best_image_url() else {
            self.status = "Selected post has no image URL.".to_string();
            return;
        };

        match webbrowser::open(url) {
            Ok(_) => self.status = format!("Opened image for post {}.", post.id),
            Err(error) => self.status = format!("Could not open browser: {error}"),
        }
    }
}
