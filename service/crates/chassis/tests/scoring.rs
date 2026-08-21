use chassis::{
    connectors::github::{GithubRepo, Owner},
    projects, scoring, snapshots,
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
        stargazers_count: 150,
        forks_count: 10,
        open_issues_count: 2,
        topics: vec!["demo".into()],
    }
}

#[sqlx::test(migrations = "../../migrations")]
async fn score_ranges_0_to_100(pool: PgPool) {
    let repo = sample_repo();
    let project = projects::upsert(&pool, &repo).await.unwrap();
    let mentions = vec![];
    let score = scoring::score(&project, &mentions);

    assert!(score.docs >= 0 && score.docs <= 100);
    assert!(score.platform >= 0 && score.platform <= 100);
    assert!(score.social >= 0 && score.social <= 100);
    assert!(score.community >= 0 && score.community <= 100);
    assert!(score.total >= 0 && score.total <= 100);
}

#[sqlx::test(migrations = "../../migrations")]
async fn snapshot_records_score(pool: PgPool) {
    let repo = sample_repo();
    let project = projects::upsert(&pool, &repo).await.unwrap();
    let mentions = vec![];
    let score = scoring::score(&project, &mentions);

    let snapshot = snapshots::record(&pool, project.id, project.stars, project.forks, &score)
        .await
        .unwrap();

    assert_eq!(snapshot.project_id, project.id);
    assert_eq!(snapshot.asia_readiness_score, score.total);
    assert_eq!(snapshot.docs_score, score.docs);
    assert_eq!(snapshot.platform_score, score.platform);
    assert_eq!(snapshot.social_score, score.social);
    assert_eq!(snapshot.community_score, score.community);
    assert_eq!(snapshot.stars, project.stars);
    assert_eq!(snapshot.forks, project.forks);
}
