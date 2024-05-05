use async_graphql::ID;
use crate::user_service::User;

pub struct DB;
impl DB {
    pub fn get_data(&self) -> Vec<User> {
        vec![
            User {
                id: ID::from("1"),
                name: String::from("Alice"),
                email: String::from("alice@example.com"),
                phone: String::from("123-456-7890"),
                address: String::from("123 Main St"),
                city: String::from("New York"),
                organization: String::from("Tech Inc."),
            },
            User {
                id: ID::from("2"),
                name: String::from("Bob"),
                email: String::from("bob@example.com"),
                phone: String::from("987-654-3210"),
                address: String::from("456 Elm St"),
                city: String::from("Los Angeles"),
                organization: String::from("Soft Co."),
            },
            User {
                id: ID::from("3"),
                name: String::from("Charlie"),
                email: String::from("charlie@example.com"),
                phone: String::from("555-123-4567"),
                address: String::from("789 Oak St"),
                city: String::from("Chicago"),
                organization: String::from("Web Devs"),
            },
            User {
                id: ID::from("4"),
                name: String::from("Diana"),
                email: String::from("diana@example.com"),
                phone: String::from("111-222-3333"),
                address: String::from("321 Pine St"),
                city: String::from("San Francisco"),
                organization: String::from("Design Co."),
            },
            User {
                id: ID::from("5"),
                name: String::from("Eve"),
                email: String::from("eve@example.com"),
                phone: String::from("444-555-6666"),
                address: String::from("555 Cedar St"),
                city: String::from("Seattle"),
                organization: String::from("Cloud Solutions"),
            },
        ]
    }
}