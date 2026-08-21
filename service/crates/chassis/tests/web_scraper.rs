use chassis::connectors::web_scraper;

#[tokio::test]
async fn search_mentions_returns_empty_stub() {
    let mentions = web_scraper::search_mentions("zhihu", "rust").await.unwrap();
    assert!(mentions.is_empty());
}
