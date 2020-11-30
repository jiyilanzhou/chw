/*

 */
use crate::model;
use deadpool_postgres::Client;
use std::io;
use crate::model::{List, Item};

pub async fn get_lists(client: &Client) -> Result<Vec<model::List>, io::Error> {
    let statement = client.prepare("select * from list order by id desc").await.unwrap();
    let lists = client.query(&statement, &[])
        .await
        .expect("Error getting Lists")
        .iter()
        .map(|row| List::from_row_ref(row).unwrap())
        .collect::<Vec<List>>();

    Ok(lists)
}

pub async fn get_items(client: &Client, list_id: i32) -> Result<Vec<model::Item>, io::Error> {
    let statement = client.prepare("select * from item where list_id = $1  order by id ").await.unwrap();
    let items = client.query(&statement, &[&list_id])
        .await
        .expect("Error getting Items")
        .iter()
        .map(|row| Item::from_row_ref(row).unwrap())
        .collect::<Vec<Item>>();

    Ok(items)
}

pub async fn insert_list(client: &Client, title: String) -> Result<List, io::Error> {
    let statement = client
        .prepare("insert into list(title) values($1) returning id,title")
        .await
        .unwrap();
    client.query(&statement, &[&title])
        .await
        .expect("Error insert List")
        .iter()
        .map(|row| List::from_row_ref(row).unwrap())
        .collect::<Vec<List>>()
        .pop()
        .ok_or(io::Error::new(io::ErrorKind::Other, "Error insert list"))
}

pub async fn inspect_item(client: &Client, list_id: i32, item_id: i32) -> Result<(), io::Error> {
    let statement = client
        .prepare("update item set checked=true where list_id = $1 and id = $2 and checked=false")
        .await.unwrap();
    let result = client
        .execute(&statement, &[&list_id, &item_id])
        .await.expect("Error checking item");

    match result {
        ref updated if *updated == 1 => Ok(()),
        _=>Err(io::Error::new(io::ErrorKind::Other,"Failed to check list"))
    }
}