use async_graphql::Object;
use crate::db::DB;
use crate::user_service::User;

pub struct Query {
    pub db: DB
}


#[Object]
impl Query {
    async fn get_user(&self, id: String) -> Option<User>{
        self.db.get_data().iter().find(|&x| x.id.0 == id)
            .cloned()
    }

    async fn get_users(&self) -> Vec<User> {
        self.db.get_data()
    }
}