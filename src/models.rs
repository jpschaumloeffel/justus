use super::schema::accounts;


#[derive(Insertable)]
#[table_name="accounts"]
pub struct NewAccount<'a> {
    pub name: &'a str,
}

#[derive(Queryable)]
pub struct Account {
    pub id: i32,
    pub name: String,
}
