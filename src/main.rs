mod error;
use error::DomainError;

use std::{collections::HashMap, fmt::Display};
use url::Url;

use fake::{Dummy, Fake, Faker, StringFaker};

// -------------------------------------------------------------------------------------------------
// YoutubeUrl

#[derive(Clone, Debug)]
struct YoutubeUrl {
    video_id: String,
}

impl TryFrom<String> for YoutubeUrl {
    type Error = DomainError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let url = Url::parse(&value)?;

        if "www.youtube.com" == url.host_str().ok_or(DomainError::InvalidYoutubeUrlError)? {
            let query_map = url
                .query_pairs()
                .into_owned()
                .collect::<HashMap<String, String>>();
            let video_id = query_map
                .get("v")
                .ok_or(DomainError::InvalidYoutubeUrlError)?;
            Ok(YoutubeUrl {
                video_id: video_id.clone(),
            })
        } else {
            Err(DomainError::InvalidYoutubeUrlError)
        }
    }
}

impl Display for YoutubeUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "https://www.youtube.com/watch?v={}", self.video_id)
    }
}

// -------------------------------------------------------------------------------------------------
// impl Dummy trait

impl Dummy<Faker> for YoutubeUrl {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_config: &Faker, rng: &mut R) -> Self {
        const ID_CHARS: &str = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_";
        let video_id = StringFaker::with(Vec::from(ID_CHARS), 11).fake_with_rng::<String, R>(rng);
        format!("https://www.youtube.com/watch?v={video_id}")
            .try_into()
            .expect("Generate Fake Error")
    }
}

// -------------------------------------------------------------------------------------------------
// YtBookMark

#[derive(Clone, Debug, Dummy)]
struct YtBookMark {
    url: YoutubeUrl,
    tags: Vec<String>,
    // account_name: String,
}

impl YtBookMark {
    fn new(
        url_str: String,
        tags: Vec<String>,
        // account_name: String,
    ) -> Result<YtBookMark, DomainError> {
        Ok(Self {
            url: url_str.try_into()?,
            tags,
            // account_name,
        })
    }
}

// -------------------------------------------------------------------------------------------------
// add_tag

fn add_tag(bookmarks: &mut [YtBookMark], tag: String) {
    bookmarks.iter_mut().for_each(|bookmark| {
        bookmark.tags.push(tag.clone());
    })
}

#[cfg(test)]
mod test {
    use super::{add_tag, YoutubeUrl, YtBookMark};
    use fake::{Fake, Faker};

    #[test]
    fn generate_fake_youtube_url() {
        let _ = Faker.fake::<YoutubeUrl>();
    }

    #[test]
    fn generate_fake_yt_bookmark() {
        let _ = Faker.fake::<YtBookMark>();
    }

    #[test]
    fn test_add_tag_v1() {
        let mut bookmarks = vec![
            YtBookMark::new(
                "https://www.youtube.com/watch?v=5gGha71avA5".to_string(),
                vec!["Rust".to_string()],
            )
            .unwrap(),
            YtBookMark::new(
                "https://www.youtube.com/watch?v=IN9jNrGAlJW".to_string(),
                vec!["Math".to_string()],
            )
            .unwrap(),
        ];
        // let mut bookmarks = vec![
        //     YtBookMark::new(
        //         "https://www.youtube.com/watch?v=5gGha71avA5".to_string(),
        //         vec!["Rust".to_string()],
        //         "John".to_string(),
        //     )
        //     .unwrap(),
        //     YtBookMark::new(
        //         "https://www.youtube.com/watch?v=IN9jNrGAlJW".to_string(),
        //         vec!["Math".to_string()],
        //         "Mark".to_string(),
        //     )
        //     .unwrap(),
        // ];

        let added_tag = "Computer".to_string();

        add_tag(&mut bookmarks, added_tag.clone());

        bookmarks.into_iter().for_each(|bookmark| {
            assert!(bookmark.tags.into_iter().any(|tag| { tag == added_tag }));
        });
    }

    #[test]
    fn test_add_tag_v2() {
        let mut bookmarks = vec![Faker.fake::<YtBookMark>(); 100];

        let added_tag = "Computer".to_string();

        add_tag(&mut bookmarks, added_tag.clone());

        bookmarks.into_iter().for_each(|bookmark| {
            assert!(bookmark.tags.into_iter().any(|tag| { tag == added_tag }));
        });
    }
}

fn main() {
    use fake::faker::name::raw::*;
    use fake::locales::JA_JP;

    use fake::faker::phone_number::raw::*;

    let youtube_url: YoutubeUrl = Faker.fake();
    println!("youtube_url: {youtube_url}");

    let bookmark = Faker.fake::<YtBookMark>();
    println!("bookmark: {bookmark:?}");

    let fake_name = LastName(JA_JP).fake::<String>();
    println!("fake_name: {fake_name}");
    // 宮本など

    let phone_number = PhoneNumber(JA_JP).fake::<String>();
    println!("fake_phone_number: {phone_number}");
    // 0595-33-4629など
}
