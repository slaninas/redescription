use super::items::*;

use opencv::prelude::*;
use std::sync::mpsc::channel;

#[derive(Debug)]
pub struct Match {
    pub id: u32,
    pub filename: String,
    pub position: opencv::core::Rect,
    pub match_val: f64,
}

pub struct MatchTimestamp<'a> {
    pub description: &'a ItemDescription,
    pub timestamp: std::time::Instant,
}


pub struct ThreadedMatcher {
    threadpool: threadpool::ThreadPool,
    items: Vec<Item>,
}

impl ThreadedMatcher {
    pub fn new(num_workers: usize, items: Vec<Item>) -> Self {
        ThreadedMatcher {
            threadpool: threadpool::ThreadPool::new(num_workers),
            items,
        }
    }

    pub fn get_matches(&self, screenshots: &Vec<Mat>) -> Vec<Match> {
        let (tx, rx) = channel();
        let num_screenshots = screenshots.len();

        let thread_count = self.threadpool.max_count();
        for screenshot in screenshots {
            let collectibles_per_thread = self.items.len() / thread_count + 1;

            for thread_id in 0..thread_count {
                let tx = tx.clone();
                let screenshot = screenshot.clone();
                let mut items = vec![];

                for j in 0..collectibles_per_thread {
                    let index = thread_id * collectibles_per_thread + j;
                    if index >= self.items.len() {
                        break;
                    }
                    items.push(self.items[index].clone());
                }

                self.threadpool.execute(move || {
                    let matches = get_matches(&screenshot, &items);
                    tx.send(matches).expect("Message send failed.");
                });
            }
        }

        let mut all_matches = vec![];
        for _ in 0..num_screenshots * thread_count {
            let mut matches = rx.recv().expect("Message receive failed.");
            all_matches.append(&mut matches);
        }

        all_matches
    }
}

pub fn get_matches(image: &Mat, items: &Vec<Item>) -> Vec<Match> {
    let (width, height) = crate::screen::ACTUAL_PIXEL_COUNT;
    let (item_width, item_height) = crate::items::ACTUAL_ITEM_SIZE;
    let mut matches = vec![];

    for item in items {
        let mut result = opencv::core::Mat::zeros(
            (height - item_height + 1) as i32,
            (width - item_width + 1) as i32,
            0,
        )
        .expect("Zero array creation failed.")
        .to_mat()
        .expect("Conversion from array to matrix failed.");

        opencv::imgproc::match_template(&image, &item.pic, &mut result, 5, &item.mask)
            .expect("Error in match_template.");

        let mut min_val = 1.0f64;
        let mut max_val = 1.0f64;
        let mut min_loc = opencv::core::Point::new(0, 0);
        let mut max_loc = opencv::core::Point::new(0, 0);

        opencv::core::min_max_loc(
            &result,
            Some(&mut min_val),
            Some(&mut max_val),
            Some(&mut min_loc),
            Some(&mut max_loc),
            &opencv::core::no_array(),
        )
        .expect("Error in min_max finding function.");

        if max_val > 0.85 && max_val != std::f64::INFINITY {
            let rect = opencv::core::Rect {
                x: max_loc.x as i32,
                y: max_loc.y as i32,
                width: item_width as i32,
                height: item_height as i32,
            };

            matches.push(Match {
                id: item.id,
                filename: item.filename.clone(),
                position: rect,
                match_val: max_val,
            });
        }
    }

    matches
}

pub fn get_uniq_ids(matches: Vec<Match>) -> Vec<u32> {
    let mut match_ids = std::collections::HashSet::<u32>::new();
    for m in matches {
        match_ids.insert(m.id);
    }

    let mut match_ids_sorted = match_ids.into_iter().collect::<Vec<_>>();
    match_ids_sorted.sort_unstable();

    match_ids_sorted
}
