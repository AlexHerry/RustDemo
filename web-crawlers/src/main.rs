use std::collections::VecDeque;
use std::sync::{Arc};
use std::sync::atomic::AtomicBool;
use anyhow::Result;
use web_crawlers::{config::parse_config, json_utils::parse_json};
use reqwest;
// use reqwest::header::{HeaderMap, HeaderValue};
use tokio;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use tokio::runtime::Runtime;
use tokio::sync::{Mutex};
use url::Url;

type AsyncVecDeque = Arc<Mutex<VecDeque<String>>>;

fn main(){
    // println!("Hello, world!");
    let config = Arc::new(parse_config().unwrap());
    let async_config = config.clone();
    let pic_list: AsyncVecDeque = Arc::new(Mutex::new(VecDeque::new()));
    let shutdown_signal = Arc::new(AtomicBool::new(false));

    let rt = Runtime::new().unwrap();

    rt.block_on(async move {
        let url = Arc::new(async_config.target.url.clone());
        let need_page = async_config.target.page;
        let task_num = async_config.target.task_num;
        let mut handler_list = Vec::new();
        let mut pic_handlers = Vec::new();

        for i in 1..=need_page {
            let page_url = Arc::clone(&url);
            let list = Arc::clone(&pic_list);

            let handler = tokio::spawn(async move {
                get_url_body(&page_url, i, list).await
            });

            handler_list.push(handler);
        }

        for i in 0..task_num {
            let handler = tokio::spawn(get_pic(Arc::clone(&pic_list), shutdown_signal.clone(), i));

            pic_handlers.push(handler);
        }

        for handler in handler_list {
            handler.await.unwrap().unwrap();
        }

        shutdown_signal.store(true, std::sync::atomic::Ordering::Relaxed);

        for handler in pic_handlers {
            handler.await.unwrap().unwrap();
        }

        // println!("{:?}", pic_list.lock().unwrap());
        println!("len: {}", pic_list.lock().await.len());
    });

}

async fn get_url_body(url: &str, page: usize, save_list: AsyncVecDeque) -> Result<()>
{
    // let mut headers = HeaderMap::new();
    // headers.insert("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/140.0.0.0 Safari/537.36".parse()?);
    // headers.insert("Cookie", HeaderValue::from_str(cookie)?);
    // headers.insert("Accept", HeaderValue::from_static("application/json"));
    // headers.insert("Referer", HeaderValue::from_static("https://pixabay.com/images/search/?order=trending&pagi=3"));
    // // headers.insert("Accept-Encoding", HeaderValue::from_static("gzip, deflate, br, zstd"));
    // headers.insert("Accept-Language", HeaderValue::from_static("zh-CN,zh;q=0.9,en;q=0.8"));
    // headers.insert("Sec-Ch-Ua", HeaderValue::from_str("\"Chromium\";v=\"140\", \"Not=A?Brand\";v=\"24\", \"Google Chrome\";v=\"140\"")?);
    // let client = reqwest::Client::new();
    //
    // let body = client.get(url).headers(headers).send().await?.text().await?;
    let url = format!("{}{}", url, page);

    let body = reqwest::get(url).await?.text().await?;

    let body = parse_json(&body)?;
    let body = body.as_array().unwrap();

    for item in body {
        let download_url = item.get("download_url");
        if let Some(url) = download_url {
            save_list.lock().await.push_back(url.to_string().trim().trim_matches('"').to_string());
        }
    }

    Ok(())
}

async fn get_pic(pic_list: AsyncVecDeque, signal: Arc<AtomicBool>, i: usize) -> Result<()>
{
    loop {
        let url = pic_list.lock().await.pop_front();

        if let Some(url) = url {
            let parsed = Url::parse(&url)?;
            let segments = parsed.path_segments().unwrap().collect::<Vec<&str>>();

            // println!("{:?}", segments);

            // 期望路径格式: /id/14/2500/1667
            let id = segments[1];
            let width = segments[2];
            let height = segments[3];
            let filename = format!("static/id-{}-{}-{}.jpg", id, width, height);

            let response = reqwest::get(url).await;
            if let Err(_) = response {
                println!("get pic failed");
                continue;
            }

            let bytes = response?.bytes().await;
            if let Err(_) = bytes {
                println!("get pic failed");
                continue;
            }

            let mut file = fs::File::create(&filename).await?;
            file.write_all(&bytes.unwrap()).await?;

            println!("{} is download complete", filename);
        } else {
            if signal.load(std::sync::atomic::Ordering::Relaxed) && pic_list.lock().await.is_empty() {
                println!("Task number: {i} shutting down");
                break;
            }
        }
    }

    Ok(())
}

// fn get_download_url(config: &Config) -> Vec<String>
// {
//     let pic_list = Vec::new();
//
//     println!("{:?}", config.target.url);
//
//     pic_list
// }