use crate::models::User;

pub async fn db_list_users(
    conn: &deadpool_postgres::Client,
) -> Result<Vec<User>, Box<dyn std::error::Error>> {
    let stmt = conn.prepare("SELECT * FROM users").await?;
    let rows = conn.query(&stmt, &[]).await?;
    let result = rows
        .iter()
        .map(|row| User::from(row))
        .collect::<Vec<User>>();
    Ok(result)
}
