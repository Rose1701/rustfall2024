use crate::status::WebsiteStatus;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use ureq;

pub struct Monitor {
    urls: Vec<String>,
    timeout: Duration,
    retries: usize,
}

impl Monitor {
    pub fn new(urls: Vec<String>) -> Self {
        Self {
            urls,
            timeout: Duration::new(5, 0),
            retries: 3,
        }
    }

    pub fn run(self) {
        let (tx, rx) = mpsc::channel();

        let valid_urls: Vec<String> = self.urls.into_iter()
        .filter(|url| Monitor::is_valid_url(url))
        .collect();

        println!("Valid URLs to monitor: {:?}", valid_urls);

        let handles: Vec<_> = valid_urls.iter().cloned().map(|url| {
            let tx = tx.clone();
            let timeout = self.timeout;
            let retries = self.retries;

            thread::spawn(move || {
                println!("Checking URL; {}", url);
                let status = Monitor::check_website(&url, timeout, retries);
                tx.send(status).unwrap();
            })
        }).collect();

        for _ in 0..handles.len() {
            let status = rx.recv().unwrap();
            println!("{:?}", status);
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }

    fn is_valid_url(url_str: &str) -> bool {
        //Simple validation: check if the URL starts with http or https
        if url_str.starts_with("http://") || url_str.starts_with("https://") {
            true
        } else {
            println!("Invalid URL: {}", url_str);
            false
        }
    }

    fn check_website(url: &str, timeout: Duration, retries: usize) -> WebsiteStatus {
        let start_time = Instant::now();
        let mut attempts = 0;

        while attempts < retries {
            attempts += 1;
            println!("Attempting to check URL: {}", url);
            let response = ureq::get(url)
                .timeout(timeout)
                .call();
            
            match response {
                Ok(resp) => {
                    let duration = start_time.elapsed();
                    return WebsiteStatus {
                        url: url.to_string(),
                        status: Ok(resp.status()),
                        response_time: duration,
                        timestamp: start_time.elapsed().as_secs_f64(),
                    };
                }

                Err(err) => {
                    if attempts == retries {
                        return WebsiteStatus {
                            url: url.to_string(),
                            status: Err(err.to_string()),
                            response_time: start_time.elapsed(),
                            timestamp: start_time.elapsed().as_secs_f64(),
                        };
                    }
                }
            }
        }

        WebsiteStatus {
            url: url.to_string(),
            status: Err("Max retries reached".to_string()),
            response_time: start_time.elapsed(),
            timestamp: start_time.elapsed().as_secs_f64(),
        }
    }
}