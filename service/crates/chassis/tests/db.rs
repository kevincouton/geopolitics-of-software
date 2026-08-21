use sqlx::PgPool;

#[derive(sqlx::FromRow)]
struct CountRow {
    count: Option<i64>,
}

#[sqlx::test(migrations = "../../migrations")]
async fn test_projects_table_exists(pool: PgPool) {
    let row = sqlx::query_as!(CountRow, "SELECT COUNT(*) AS count FROM projects")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(row.count, Some(0));
}
