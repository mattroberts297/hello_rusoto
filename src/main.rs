use std::default::Default;

use rusoto_core::Region;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, ListTablesInput};

// OK so this works as expected. Must be something to do with actix vs. rusoto (and tokio).
#[tokio::main]
async fn main() -> Result<(), String> {
    // DynamoDB.
    let region = match std::env::var("AWS_REGION") {
        Ok(r) => if String::eq(&r, &String::from("local")) {
            Region::Custom { name: String::from("eu-west-2"), endpoint: String::from("http://localhost:8000") }
        } else {
            Region::default()
        }
        Err(_e) => Region::default()
    };

    let client = DynamoDbClient::new(region);
    let list_tables_input: ListTablesInput = Default::default();

    match client.list_tables(list_tables_input).await {
        Ok(output) => {
            match output.table_names {
                Some(table_name_list) => {
                    println!("Tables in database:");
                    for table_name in table_name_list {
                        println!("{}", table_name);
                    }
                    Ok(())
                }
                None => {
                    println!("No tables in database!");
                    Ok(())
                },
            }
        }
        Err(error) => {
            println!("Error: {:?}", error);
            Err(error.to_string())
        }
    }
}
