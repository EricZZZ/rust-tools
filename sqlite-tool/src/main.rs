use anyhow::Result;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Word {
    id: String,
    jlpt_level: String,
    vocab_kanji: String,
}

fn main() -> Result<()> {
    println!("please enter the path to the database file:");

    let mut path = String::new();
    std::io::stdin().read_line(&mut path)?;
    path = path.trim().to_string();

    let conn = Connection::open(&path)?;
    let mut stmt = conn.prepare("select id, jlpt_level, vocab_kanji from words")?;
    let words = stmt.query_map([], |row| {
        Ok(Word {
            id: row.get(0)?,
            jlpt_level: row.get(1)?,
            vocab_kanji: row.get(2)?,
        })
    })?;

    let mut words_n1 = Vec::new();
    let mut words_n2 = Vec::new();
    let mut words_n3 = Vec::new();
    let mut words_n4n5 = Vec::new();

    for word in words {
        let word = word?;
        if word.jlpt_level.contains("N1") {
            words_n1.push(word);
        } else if word.jlpt_level.contains("N2") {
            words_n2.push(word);
        } else if word.jlpt_level.contains("N3") {
            words_n3.push(word);
        } else {
            words_n4n5.push(word);
        }
    }

    println!("words_n1: {}", words_n1.len());
    println!("words_n2: {}", words_n2.len());
    println!("words_n3: {}", words_n3.len());
    println!("words_n4n5: {}", words_n4n5.len());

    // words json 格式保存到文件中
    let mut file_n1 = std::fs::File::create("words_n1.json")?;
    serde_json::to_writer(&mut file_n1, &words_n1)?;
    let mut file_n2 = std::fs::File::create("words_n2.json")?;
    serde_json::to_writer(&mut file_n2, &words_n2)?;
    let mut file_n3 = std::fs::File::create("words_n3.json")?;
    serde_json::to_writer(&mut file_n3, &words_n3)?;
    let mut file_n4n5 = std::fs::File::create("words_n4n5.json")?;
    serde_json::to_writer(&mut file_n4n5, &words_n4n5)?;
    println!("words.json created");
    Ok(())
}
