use crate::items;

fn parse_item(element: scraper::element_ref::ElementRef) -> items::ItemDescription {
    let fselector = scraper::Selector::parse("p").expect("Error parsing <p>");
    let fv: Vec<_> = element.select(&fselector).collect();

    let mut item_title = String::from("");
    let mut quote = String::from("");
    let mut descriptions = vec![];
    let mut id = 0u32;
    let mut item_type = items::ItemType::Item;

    for p in fv {
        match p.value().attr("class") {
            Some("item-title") => {
                item_title = p.inner_html();
            }
            Some("r-itemid") => {
                let html = p.inner_html();
                let itemid = html.split(' ').collect::<Vec<_>>();
                item_type = if itemid[0] == "TrinketID:" {
                    items::ItemType::Trinket
                } else {
                    items::ItemType::Item
                };
                id = itemid[1].parse::<u32>().expect("Item id parsing error.");
            }
            Some("pickup") => {
                quote = p.inner_html();
            }
            Some("tags") | Some("ab-red") => {}
            Some("r-unlock") | Some("r-special") | Some("abp-red") => {}
            Some(x) => panic!("FOUND UNKNOWN CLASS >{}<", x),
            _ => {
                descriptions.push(p.inner_html());
            }
        }
    }

    let item = items::ItemDescription {
        item_title,
        id,
        item_type,
        quote,
        descriptions,
    };

    item
}

pub fn scrap_descriptions() -> Result<items::Items, ureq::Error> {
    let body: String = ureq::get("https://platinumgod.co.uk/rebirth")
        .set("Example-Header", "header value")
        .call()?
        .into_string()?;

    let parsed_html = scraper::Html::parse_document(&body);

    let selector = scraper::Selector::parse("li").expect("List parsing error.");

    let li = parsed_html.select(&selector);
    let v: Vec<_> = li
        .filter(|x| {
            let has_class = x.value().attr("class");
            match has_class {
                Some(class) => {
                    if class.contains("textbox") {
                        return true;
                    }
                }
                None => return false,
            }
            false
        })
        .collect();

    let mut descriptions = items::Items { items: vec![] };
    let mut last_item_id = 0;
    let mut trinkets = false;

    for (i, item) in v.iter().enumerate() {
        let item_description = parse_item(*item);
        let item_id = item_description.id;
        descriptions.items.push(item_description);

        if item_id < last_item_id {
            trinkets = true;
        }

        let uniq_id = if trinkets {
            items::get_uniq_id(items::ItemType::Trinket, item_id)
        } else {
            items::get_uniq_id(items::ItemType::Item, item_id)
        };
        descriptions
            .items
            .last_mut()
            .expect("last_mut() call failed.")
            .id = uniq_id;

        last_item_id = item_id;
        // Take items and trinkets only
        if i >= 400 {
            break;
        }
    }

    Ok(descriptions)
}
