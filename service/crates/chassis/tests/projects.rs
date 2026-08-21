use chassis::{
    connectors::github::{GithubRepo, Owner},
    projects,
};
use sqlx::PgPool;

fn sample_repo() -> GithubRepo {
    GithubRepo {
        owner: Owner {
            login: "octocat".into(),
        },
        name: "hello".into(),
        description: Some("demo".into()),
        language: Some("Rust".into()),
        stargazers_count: 100,
        forks_count: 10,
        open_issues_count: 2,
        topics: vec!["demo".into()],
    }
}

#[sqlx::test(migrations = "../../migrations")]
async fn upsert_and_list_project(pool: PgPool) {
    let repo = sample_repo();
    let project = projects::upsert(&pool, &repo).await.unwrap();
    assert_eq!(project.github_name, "hello");
    let list = projects::list(&pool, 10).await.unwrap();
    assert_eq!(list.len(), 1);
}
