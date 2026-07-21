use sequelles::Delete as _;
use sequelles::InsertOrIgnore as _;



use crate::structs::ToppingInsert;

pub async fn deletes(conn: &mut sqlx::SqliteConnection) {
    // Similar to update, delete only requires calling the function on the row

    let strawberry = ToppingInsert::builder()
        .name("Strawberry")
        .build()
        .insert_or_ignore(&mut *conn)
        .await
        .unwrap()
        .unwrap();

    strawberry.delete(conn).await.unwrap();
}
