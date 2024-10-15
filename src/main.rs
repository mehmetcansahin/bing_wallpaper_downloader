use regex::Regex;
use reqwest::blocking::get;
use std::fs::create_dir_all;
use std::fs::File;
use std::io::copy;
use std::path::Path;

fn download_image(url: &str, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let response = get(url)?;
    let mut dest = File::create(path)?;
    copy(&mut response.bytes()?.as_ref(), &mut dest)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://raw.githubusercontent.com/asvow/bing-wallpaper/refs/heads/main/bing_en.json";
    let response = get(url)?.text()?;
    let json: serde_json::Value = serde_json::from_str(&response)?;

    let download_dir = Path::new("bing_wallpapers");
    create_dir_all(download_dir)?;

    for data in json.as_object().unwrap() {
        let image_url: &str = data.1["url"].as_str().unwrap();
        let re = Regex::new(r"th\?id=OHR\.(?P<name>[^_]+)_").unwrap();
        let caps = re.captures(image_url).unwrap();
        let file_name = format!("{}.jpg", &caps["name"]);
        let file_path = download_dir.join(file_name);
        download_image(image_url, &file_path)?;
    }

    println!("Images downloaded successfully!");
    Ok(())
}
