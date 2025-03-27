use crate::api;
use crate::config::WikiConfig;
use crate::util::validate;
use log::{debug, info};
use std::collections::{HashMap, VecDeque};
use std::time::Instant;

fn make_path(
    visited_links: &HashMap<String, String>,
    visited_back_links: &HashMap<String, String>,
    mut cur: String,
    joint: &str,
) -> Vec<String> {
    let mut path = Vec::new();

    path.push(joint.to_string());
    while *visited_links.get(&cur).unwrap() != cur {
        path.push(cur.clone());
        cur = visited_links.get(&cur).unwrap().clone();
    }

    path.push(cur.clone());
    path.reverse();

    cur = visited_back_links.get(&joint.to_string()).unwrap().clone();
    while *visited_back_links.get(&cur).unwrap() != cur {
        path.push(cur.clone());
        cur = visited_back_links.get(&cur).unwrap().clone();
    }
    path.push(cur.clone());
    path
}

fn make_back_path(
    visited_links: &HashMap<String, String>,
    visited_back_links: &HashMap<String, String>,
    mut cur: String,
    joint: &str,
) -> Vec<String> {
    let mut path = Vec::new();
    let mut temp = joint.to_string();
    while *visited_links.get(&temp).unwrap() != temp {
        path.push(temp.clone());
        temp = visited_links.get(&temp).unwrap().clone();
    }
    path.push(temp.clone());
    path.reverse();

    while *visited_back_links.get(&cur).unwrap() != cur {
        path.push(cur.clone());
        cur = visited_back_links.get(&cur).unwrap().clone();
    }
    path.push(cur.clone());
    path
}

pub async fn run(start: &str, end: &str, config: &WikiConfig) -> Result<(), String> {
    let start_time = Instant::now();

    let mut visited_links = HashMap::new();
    let mut queue_links = VecDeque::new();

    let mut visited_back_links = HashMap::new();
    let mut queue_back_links = VecDeque::new();

    queue_links.push_back(start.to_string());
    visited_links.insert(start.to_string(), start.to_string());

    queue_back_links.push_back(end.to_string());
    visited_back_links.insert(end.to_string(), end.to_string());

    for _ in 0..config.max_iterations {
        let mut cur = queue_links.pop_front().unwrap();

        info!("Teкущая прямая статья: {}", cur);

        let links = api::get_links(&cur, &config)
            .await
            .map_err(|e| e.to_string())?;

        debug!("Найдено {} ссылок для статьи: {}", links.len(), cur);
        for link in links.iter().map(|s| s.to_string()) {
            if visited_links.contains_key(&link) || !validate(&link, &config) {
                continue;
            }

            if visited_back_links.contains_key(&link) {
                info!("Стык: {}", link);
                let path = make_path(&visited_links, &visited_back_links, cur, &link);

                info!("Путь найден: {:?}", path.join(" -> "));
                info!("Время посика: {:.2?}", start_time.elapsed());
                return Ok(());
            }

            visited_links.insert(link.clone(), cur.clone());
            queue_links.push_back(link.clone());
        }

        let mut cur = queue_back_links.pop_front().unwrap();

        info!("Teкущая обратная статья: {}", cur);

        let links = api::get_links_here(&cur, &config)
            .await
            .map_err(|e| e.to_string())?;

        debug!("Найдено {} ссылок для статьи: {}", links.len(), cur);
        for link in links.iter().map(|s| s.to_string()) {
            if visited_back_links.contains_key(&link) || !validate(&link.to_string(), &config) {
                continue;
            }
            if visited_links.contains_key(&link) {
                info!("Стык: {}", link);
                let mut path = make_back_path(&visited_links, &visited_back_links, cur, &link);

                info!("Путь найден: {:?}", path.join(" -> "));
                info!("Время посика: {:.2?}", start_time.elapsed());
                return Ok(());
            }

            visited_back_links.insert(link.clone(), cur.clone());
            queue_back_links.push_back(link.clone());
        }
    }

    Err("Путь не найден".to_string())
}
