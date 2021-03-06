use crate::database::schema::{auth, product, bid};
use serde::{Serialize};

pub type Id = i64;

#[derive(Debug, Queryable)]
pub struct Auth {
    pub id: Id,
    pub public_key: String,
    pub username: String,
    pub hashed_password: String,
    pub encrypted_private_key: String,
}

#[derive(Debug, Serialize, Identifiable, Queryable)]
#[table_name = "product"]
pub struct Product {
    pub id: Id,
    pub record_id: String,
    pub auth_id: i64,
    pub title: String,
    pub price: i64,
    pub latitude: i64,
    pub longitude: i64,
}

#[derive(Debug, Associations, Identifiable, Serialize, Queryable)]
#[belongs_to(Product, foreign_key="product_id")]
#[table_name = "bid"]
pub struct Bid {
    pub id: Id,
    pub product_id: i64,
    pub price: i64,
}

#[derive(Insertable)]
#[table_name = "auth"]
pub struct NewAuth<'a> {
    pub public_key: &'a str,
    pub username: &'a str,
    pub hashed_password: &'a str,
    pub encrypted_private_key: &'a str,
}

#[derive(Insertable)]
#[table_name = "bid"]
pub struct NewBid {
    pub product_id: i64,
    pub price: i64,
}

#[derive(Insertable)]
#[table_name = "product"]
pub struct NewProduct<'a> {
    pub record_id: &'a str,
    pub auth_id: i64,
    pub title: &'a str,
    pub price: i64,
    pub latitude: i64,
    pub longitude: i64,
}
