use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

pub const ACTUAL_ITEM_SIZE: (u32, u32) = (32, 32);

#[derive(Clone, Debug)]
pub struct Item {
    pub id: u32,
    pub filename: String,
    pub pic: opencv::core::Mat,
    pub mask: opencv::core::Mat,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Items {
    pub items: Vec<ItemDescription>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ItemDescription {
    pub item_title: String,
    pub id: u32,
    pub item_type: ItemType,
    pub quote: String,
    pub descriptions: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ItemType {
    Item,
    Trinket,
}

pub fn get_uniq_id(item_type: ItemType, id: u32) -> u32 {
    match item_type {
        ItemType::Item => id,
        ItemType::Trinket => id + 10_000,
    }
}

pub fn get_items(path: &std::path::Path) -> Vec<Item> {
    let mut items = vec![];

    let collectibles: Vec<_> = std::fs::read_dir(path.join(std::path::Path::new(
        "graphics_unpack/resources/gfx/items/collectibles/",
    )))
    .expect("collectibles loading failed.")
    .filter_map(|res| res.ok())
    .collect();
    let mut trinkets: Vec<_> = std::fs::read_dir(path.join(std::path::Path::new(
        "graphics_unpack/resources/gfx/items/trinkets/",
    )))
    .expect("trinkets loading failed.")
    .filter_map(|res| res.ok())
    .collect();

    let mut all_paths = collectibles;
    all_paths.append(&mut trinkets);

    for item_path in all_paths {
        let item_path = item_path.path();
        let filename = item_path
            .file_name()
            .expect("Can't get filename from the path.")
            .to_os_string()
            .into_string()
            .expect("Can't convert filename to string.");
        let filename_split: Vec<_> = filename.split('_').collect();
        if filename_split.len() != 3 {
            continue;
        }
        let id_from_filename = filename_split[1]
            .parse::<u32>()
            .expect("Item id parsing failed.");

        let is_trinket = filename_split[0] == "trinket";
        let id = if is_trinket {
            get_uniq_id(ItemType::Trinket, id_from_filename)
        } else {
            get_uniq_id(ItemType::Item, id_from_filename)
        };

        let pic = opencv::imgcodecs::imread(
            item_path
                .as_os_str()
                .to_str()
                .expect("Path conversion to str failed."),
            0,
        )
        .expect("Item image loading failed.");
        let pic_tmp = opencv::imgcodecs::imread(
            item_path
                .as_os_str()
                .to_str()
                .expect("Path conversion to str failed."),
            -1,
        )
        .expect("Item image loading failed.");

        let mut channels = opencv::types::VectorOfMat::new();
        opencv::core::split(&pic_tmp, &mut channels).expect("Image split to channels failed");
        let alpha_channel = channels.get(3).expect("Can't get channel 3.");

        items.push(Item {
            id,
            filename,
            pic,
            mask: alpha_channel,
        });
    }

    items
}

pub fn get_descriptions(path: &std::path::Path) -> HashMap<u32, ItemDescription> {
    let descriptions = if std::path::Path::new(path).is_file() {
        let mut descriptions_file = File::open(path).expect("Can't open file");
        let mut content = String::new();

        descriptions_file
            .read_to_string(&mut content)
            .expect("Read from file to string failed.");
        serde_json::from_str::<Items>(content.as_str()).expect("Conversion to json failed.")
    } else {
        let descriptions =
            crate::descriptions::scrap_descriptions().expect("Scrapping from web failed.");
        let serialized =
            serde_json::to_string(&descriptions).expect("Conversion from json to string failed.");

        let mut file = File::create(path).expect("File open failed.");
        println!("Saving descriptions to {}", path.display());
        file.write_all(serialized.as_bytes())
            .expect("File write failed");

        descriptions
    };

    let mut descriptions_map = HashMap::new();

    for item in descriptions.items {
        descriptions_map.insert(item.id, item);
    }

    descriptions_map
}
