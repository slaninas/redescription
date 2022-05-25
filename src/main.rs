mod descriptions;
mod items;
mod matching;
mod screen;

fn main() {
    let mut capturer = screen::get_capturer();

    let args: Vec<_> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} PATH_TO_UNPACKED_RESOURCES", args[0]);
        std::process::exit(1);
    }

    let items_path = &args[1];
    let descriptions_path = "descriptions.json";

    let items = items::get_items(std::path::Path::new(&items_path));
    let item_descriptions = items::get_descriptions(std::path::Path::new(descriptions_path));

    let threaded_matcher = matching::ThreadedMatcher::new(4, items);

    let mut matches_to_display = Vec::<matching::MatchTimestamp>::new();

    loop {
        // let start = std::time::Instant::now();
        let mut screens = vec![];

        screens.push(screen::get_prepared_screenshot(&mut capturer));

        let match_ids = matching::get_uniq_ids(threaded_matcher.get_matches(&screens));

        // Update timestamps for already found items, add newly found
        for id in match_ids {
            let mut found = false;
            for m in &mut matches_to_display {
                if m.description.id == id {
                    found = true;

                    m.timestamp = std::time::Instant::now();
                    break;
                }
            }
            if !found {
                let description = item_descriptions
                    .get(&id)
                    .expect("Item not found in the set");
                matches_to_display.push(matching::MatchTimestamp {
                    description,
                    timestamp: std::time::Instant::now(),
                });
            }
        }

        print!("{}c", 27 as char);

        // let duration = start.elapsed();
        // println!("Time elapsed between frames: {} ms", duration.as_millis());
        // println!("--------------------");

        matches_to_display.retain(|desc| {
            return desc.timestamp.elapsed().as_secs() <= 2;
        });

        for description in &matches_to_display {
            let item = description.description;
            println!("{}\n  {}", item.item_title, item.quote);

            for desc in &item.descriptions {
                println!("  - {}", desc);
            }
            println!();
        }
    }
}
