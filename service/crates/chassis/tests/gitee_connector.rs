use chassis::{
    connectors::{gitee, github},
    projects,
};
use sqlx::PgPool;

fn sample_repo() -> github::GithubRepo {
    github::GithubRepo {
        owner: github::Owner {
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

#[tokio::test]
async fn find_mirror_is_no_op_stub() {
    // Avoids any external HTTP calls and always returns Ok(None).
    let result = gitee::find_mirror("octocat", "hello").await;
    assert_eq!(result.unwrap(), None);
}

#[sqlx::test(migrations = "../../migrations")]
async fn update_gitee_mirror_sets_mirror_fields(pool: PgPool) {
    let repo = sample_repo();
    let project = projects::upsert(&pool, &repo).await.unwrap();
    assert!(!project.has_gitee_mirror);

    let updated = projects::update_gitee_mirror(&pool, project.id, "mirror", "hello-mirror")
        .await
        .unwrap();

    assert_eq!(updated.gitee_owner.as_deref(), Some("mirror"));
    assert_eq!(updated.gitee_name.as_deref(), Some("hello-mirror"));
    assert!(updated.has_gitee_mirror);
}

#[sqlx::test(migrations = "../../migrations")]
async fn update_chinese_readme_sets_flag(pool: PgPool) {
    let repo = sample_repo();
    let project = projects::upsert(&pool, &repo).await.unwrap();
    assert!(!project.has_chinese_readme);

    let updated = projects::update_chinese_readme(&pool, project.id, true)
        .await
        .unwrap();

    assert!(updated.has_chinese_readme);
}
