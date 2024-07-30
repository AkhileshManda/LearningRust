//serialise and deserialise
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>,
}

fn main() {
    let json = r#"
        {
            "article" : "How to work with json in rust",
            "author": "Akhilesh",
            "paragraph":[
                {
                    "name": "starting"
                },
                {
                    "name": "body"
                },
                {
                    "name": "conclusion"
                }
            ]
        }"#;

    let parsed_json: Article = read_json_typed(json);
    println!(
        "name is {}, author is {}",
        parsed_json.article, parsed_json.author
    );
    println!("first para is {}", parsed_json.paragraph[0].name);
}

fn read_json_typed(json: &str) -> Article {
    let parsed: Article = serde_json::from_str(json).unwrap();
    
    parsed
}
