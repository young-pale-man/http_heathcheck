use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use std::io;
use std::{thread, time::Duration};


#[derive(Serialize, Deserialize, Debug)]
struct GETAPIResponse {
    origin: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct JSONResponse {
    json: HashMap<String, String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let client = reqwest::Client::new();   

//reading time value
println!("Amount of rounds:");
    let mut input_time = String::new();
    io::stdin()
        .read_line(&mut input_time)
        .expect("failed to read from stdin");
    let time = input_time.trim();

//reading user`s URL
    println!("Input your URL:");
    let mut s1 = String::new();
        io::stdin()
        .read_line(&mut s1)
        .expect("Failed to read line");

for i in 0..time.parse::<i32>().unwrap()
{
    //GET method
    let resp404 = client.get(&s1)
    .send()
    .await?;
    let result = resp404.status();

    //print round number
    println!("\nRound:{}", i + 1);    
    
    //result output
    if result == 200 {
        println!("Checking {}Status: OK({:?})", s1, result);
    }
    else if result != 200 {
        println!("Checking {}Status: ERR({:?}) \n", s1, result);
    }   
    
    //1s delay
    thread::sleep(Duration::from_millis(1000));
}    
    println!("End of checking.");
    Ok(())
}