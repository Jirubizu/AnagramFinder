use std::io::Write;

pub async fn obtain_latest_wordlist() -> Result<(), Box<dyn std::error::Error>> {
    let uri = "https://raw.githubusercontent.com/dwyl/english-words/master/words.txt";
    let resp = reqwest::get(uri).await?.text().await?;
    let mut file = std::fs::File::create("words.txt").expect("create failed");
    file.write_all(resp.as_bytes()).expect("write failed");

    Ok(())
}
