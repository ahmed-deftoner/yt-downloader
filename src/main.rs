
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
}

fn main() {
    
}