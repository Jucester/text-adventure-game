use csv::{ReaderBuilder, StringRecord};
use std::{fs};
use std::collections::{HashMap};

const FILENAME: &str = "history.csv";

// TIPO, TAG, TEXTO, VIDA
#[derive(Debug)]
struct HistoryData {
    typeData: String,
    text: String,
    tag: String,
    life: i32
}

impl HistoryData {
    pub fn new(row: StringRecord) -> Self {
        let life: i32 = row.get(3).unwrap().trim().parse().unwrap_or(0);

        Self {  
            typeData: row.get(0).unwrap().trim().to_string(),
            tag: row.get(1).unwrap().trim().to_string(),
            text: row.get(2).unwrap().trim().to_string(),
            life,
        }
    }
}


fn main() {
    // let mut history_data: Vec<HistoryData> = vec![];
    let mut history_data: HashMap<String, HistoryData> = HashMap::new();

    let content = fs::read_to_string(FILENAME).unwrap();

    let mut rdr = ReaderBuilder::new().delimiter(b';').from_reader(content.as_bytes());

    for result in rdr.records() {
        let result = result.unwrap();
        let data = HistoryData::new(result);
        // history_data.push(data);
        history_data.insert(data.tag.to_owned(), data);
    }


    println!("{:?}", history_data["DERECHA"]);
}
