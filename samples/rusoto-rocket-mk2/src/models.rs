use schema::hits;

#[derive(Queryable, Insertable, Debug)]
#[table_name="hits"]
pub struct Hit {
    pub id: i32,
    pub hits_so_far: i32,
}