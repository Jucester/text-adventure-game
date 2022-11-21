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
    life: i32,
    options: Vec<HistoryData>,
}

impl HistoryData {
    pub fn new(row: StringRecord) -> Self {
        let life: i32 = row.get(3).unwrap().trim().parse().unwrap_or(0);

        Self {  
            typeData: row.get(0).unwrap().trim().to_string(),
            tag: row.get(1).unwrap().trim().to_string(),
            text: row.get(2).unwrap().trim().to_string(),
            life,
            options: vec![],
        }
    }
}


fn main() {
    // let mut history_data: Vec<HistoryData> = vec![];
    let mut history_data: HashMap<String, HistoryData> = HashMap::new();
    let mut life = 100;
    let mut current_tag = "INICIO";

    let mut last_record: String = "".to_string();
    

    let content = fs::read_to_string(FILENAME).unwrap();

    let mut rdr = ReaderBuilder::new().delimiter(b';').from_reader(content.as_bytes());

    for result in rdr.records() {
        let result = result.unwrap();
        let data = HistoryData::new(result);
        // history_data.push(data);
        
        if data.typeData == "SITUACION" {
            let record_tag = data.tag.clone();
            history_data.insert(data.tag.clone(), data);
            last_record = record_tag;
        } else if data.typeData == "OPCION" {
            if let Some(row) = history_data.get_mut(&last_record) {
                (*row).options.push(data);
            }
        }
        
    }
   
    // Game loop  
    loop {
        println!("You have {} life points", life);

        if let Some(row) = history_data.get(current_tag) {
            println!("{}", row.text);
            for (indice, option) in row.options.iter().enumerate() {
                println!("[{}]. {}", indice, option.text) ;
            }

                let mut selection = String::new();

                std::io::stdin().read_line(&mut selection);
            
                let select = selection.trim().parse().unwrap_or(99);

                if let Some(selected_option) = &row.options.get(select) {
                    current_tag = &selected_option.tag;
                } else {
                    println!("Invalid command");
                }

                life += row.life;

                println!("");
        } else {
            break;
        }
       
        if life <= 0 {
            println!("You lose");
            break;
        }
        
    }
}
