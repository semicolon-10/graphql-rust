use async_graphql::{ID, Object};

#[derive(Clone)]
pub struct User {
    pub id: ID,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub address: String,
    pub city: String,
    pub organization: String
}


#[Object]
impl User {
    async fn id(&self) -> &str {
        &self.id
    }

    async fn name(&self) -> &str {
        &self.name
    }
    async fn email(&self) -> &str {
        &self.email
    }
    async fn phone(&self) -> &str {
        &self.phone
    }
    async fn address(&self) -> &str {
        &self.address
    }
    async fn city(&self) -> &str {
        &self.city
    }
    async fn organization(&self) -> &str {
        &self.organization
    }
}