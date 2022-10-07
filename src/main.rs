use anyhow::Result;
use qstring::QString;
use reqwest::{blocking::Client, Url};
use serde_json::Value;

// read the passed by a user link
use std::env;

fn get_video_id(url: &str) -> Option<&str> {
    if let Some(id) = regex::Regex::new(r"https://www\.youtube\.com/watch\?v=(.*)")
        .expect("correct regex")
        .captures(url)
        .unwrap()
        .get(1)
    {
        Some(id.as_str())
    }else {
        None
    }
}

fn get_video_info(url: &str) -> Result<Value> {
    let json = if let Some(id) = get_video_id(url) {
        let video_url = format!(
            "https://www.youtube.com/get_video_info?video_id={}&el=embedded&ps=default",
            id
        );
        let res_body = reqwest::blocking::get(video_url.as_str())?.text()?;

        QString::from(res_body.as_str())
            .get("player_response")
            .unwrap_or("")
            .to_owned()
    } else {
        return Err(anyhow::Error::msg("couldn't get video id".to_string()));
    };

    
    Ok(serde_json::from_str(&json)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id_from_url_extraction_short() {
        let url = "https://www.youtube.com/watch?v=Bn40gUUd5m0";
        let id = get_video_id(url);

        assert!(id.is_some());
        assert_eq!(id.unwrap(), "Bn40gUUd5m0");
    }

    #[test]
    fn test_get_video_info() {
        let url = "https://www.youtube.com/watch?v=zCLOJ9j1k2Y";
 
        let info = get_video_info(url);
 
        assert!(info.is_ok());
    }

    #[test]
    fn test_get_video_download_url() {
        let url = "https://www.youtube.com/watch?v=zCLOJ9j1k2Y";
        let video_info = get_video_info(url).unwrap();
        let url = get_video_download_url(&video_info);

        assert!(url.is_some());
    }
}

fn main() {
    
}