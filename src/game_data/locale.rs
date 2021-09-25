use anyhow::anyhow;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs::File;

lazy_static! {
    pub static ref DISPLAY_NAMES: HashMap<String, String> = get_display_names().unwrap();
}

fn get_display_names() -> Result<HashMap<String, String>, anyhow::Error> {
    let locale_file = File::open("data/config/Localization.txt")?;
    let mut rdr = csv::Reader::from_reader(locale_file);
    let headers = rdr.headers()?;
    let key_index = headers
        .iter()
        .position(|h| h == "Key")
        .ok_or(anyhow!("Failed to find Key header"))?;
    // let file_index = headers
    //     .iter()
    //     .position(|h| h == "File")
    //     .ok_or(anyhow!("Failed to find File header"))?;
    let eng_index = headers
        .iter()
        .position(|h| h == "english")
        .ok_or(anyhow!("Failed to find english header"))?;
    let display_names: HashMap<String, String> = rdr
        .records()
        .filter_map(|r| match r {
            Ok(r) => {
                // let file = r.get(file_index)?;
                // // We only care about the names of items
                // if file != "items" {
                //     return None;
                // }
                let key = r.get(key_index)?;
                let eng = r.get(eng_index)?;
                Some((key.to_string(), eng.to_string()))
            }
            Err(_) => None,
        })
        .collect();
    Ok(display_names)
}
