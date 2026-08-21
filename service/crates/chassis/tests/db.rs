use sqlx::PgPool;

#[sqlx::test(migrations = "../../migrations")]
async fn test_projects_table_exists(pool: PgPool) {
    let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM projects")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(row.0, 0);
}
