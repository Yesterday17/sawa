use sawa_core::{
    models::misc::{Media, MediaId},
    repositories::MediaRepository,
};

fn create_test_media(url: &str) -> Media {
    Media {
        id: MediaId::new(),
        url: url::Url::parse(url).unwrap(),
    }
}

/// Test save and find_by_id.
pub async fn test_save_and_find_by_id<R: MediaRepository>(repo: R) {
    let media = create_test_media("https://example.com/image.png");
    let media_id = media.id;

    repo.save(&media).await.unwrap();

    let found = repo.find_by_id(&media_id).await.unwrap();
    assert!(found.is_some());
    assert_eq!(found.unwrap().id, media_id);

    // Clean up
    repo.delete(&media_id).await.unwrap();
}

/// Test find_by_ids batch query.
pub async fn test_find_by_ids<R: MediaRepository>(repo: R) {
    let media1 = create_test_media("https://example.com/1.png");
    let media2 = create_test_media("https://example.com/2.png");
    let media3 = create_test_media("https://example.com/3.png");

    repo.save(&media1).await.unwrap();
    repo.save(&media2).await.unwrap();
    repo.save(&media3).await.unwrap();

    let ids = vec![media1.id, media2.id];
    let results = repo.find_by_ids(&ids).await.unwrap();
    assert_eq!(results.len(), 2);
    assert!(results.iter().any(|m| m.id == media1.id));
    assert!(results.iter().any(|m| m.id == media2.id));

    // Clean up
    repo.delete(&media1.id).await.unwrap();
    repo.delete(&media2.id).await.unwrap();
    repo.delete(&media3.id).await.unwrap();
}

/// Test delete removes media.
pub async fn test_delete<R: MediaRepository>(repo: R) {
    let media = create_test_media("https://example.com/delete.png");
    let media_id = media.id;

    repo.save(&media).await.unwrap();
    repo.delete(&media_id).await.unwrap();

    let after_delete = repo.find_by_id(&media_id).await.unwrap();
    assert!(after_delete.is_none());
}
