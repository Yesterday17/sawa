use sawa_core::{
    models::misc::{NonEmptyString, Tag},
    repositories::TagRepository,
};

fn make_string(s: &str) -> NonEmptyString {
    unsafe { NonEmptyString::new_unchecked(s.to_string()) }
}

fn create_test_tag(name: &str) -> Tag {
    Tag::new(make_string(name))
}

/// Test save and find_by_id.
pub async fn test_save_and_find_by_id<R: TagRepository>(repo: R) {
    let tag = create_test_tag("Test Tag");
    let tag_id = tag.id;

    repo.save(&tag).await.unwrap();

    let found = repo.find_by_id(&tag_id).await.unwrap();
    assert!(found.is_some());
    assert_eq!(found.unwrap().id, tag_id);

    // Clean up
    repo.delete(&tag_id).await.unwrap();
}

/// Test find_all returns all tags.
pub async fn test_find_all<R: TagRepository>(repo: R) {
    let tag1 = create_test_tag("Tag 1");
    let tag2 = create_test_tag("Tag 2");

    repo.save(&tag1).await.unwrap();
    repo.save(&tag2).await.unwrap();

    let all = repo.find_all().await.unwrap();
    assert!(all.len() >= 2);
    assert!(all.iter().any(|t| t.id == tag1.id));
    assert!(all.iter().any(|t| t.id == tag2.id));

    // Clean up
    repo.delete(&tag1.id).await.unwrap();
    repo.delete(&tag2.id).await.unwrap();
}

/// Test find_by_name_prefix searches by prefix.
pub async fn test_find_by_name_prefix<R: TagRepository>(repo: R) {
    let tag1 = create_test_tag("Hatsune Miku");
    let tag2 = create_test_tag("Kagamine Rin");
    let tag3 = create_test_tag("KAITO");

    repo.save(&tag1).await.unwrap();
    repo.save(&tag2).await.unwrap();
    repo.save(&tag3).await.unwrap();

    // Search for "Ka" - should find Kagamine and KAITO
    let results = repo.find_by_name_prefix("Ka").await.unwrap();
    assert_eq!(results.len(), 2);
    assert!(results.iter().any(|t| t.id == tag2.id));
    assert!(results.iter().any(|t| t.id == tag3.id));

    // Clean up
    repo.delete(&tag1.id).await.unwrap();
    repo.delete(&tag2.id).await.unwrap();
    repo.delete(&tag3.id).await.unwrap();
}

/// Test find_by_parent returns child tags.
pub async fn test_find_by_parent<R: TagRepository>(repo: R) {
    let parent = create_test_tag("VOCALOID");
    let parent_id = parent.id;

    let child1 = Tag::with_parent(make_string("Hatsune Miku"), parent_id);
    let child2 = Tag::with_parent(make_string("Kagamine Rin"), parent_id);
    let orphan = create_test_tag("Orphan Tag");

    repo.save(&parent).await.unwrap();
    repo.save(&child1).await.unwrap();
    repo.save(&child2).await.unwrap();
    repo.save(&orphan).await.unwrap();

    let children = repo.find_by_parent(&parent_id).await.unwrap();
    assert_eq!(children.len(), 2);
    assert!(children.iter().any(|t| t.id == child1.id));
    assert!(children.iter().any(|t| t.id == child2.id));

    // Clean up (delete children first, then parent)
    repo.delete(&child1.id).await.unwrap();
    repo.delete(&child2.id).await.unwrap();
    repo.delete(&orphan.id).await.unwrap();
    repo.delete(&parent_id).await.unwrap();
}

/// Test find_roots returns tags without parent.
pub async fn test_find_roots<R: TagRepository>(repo: R) {
    let root1 = create_test_tag("Root 1");
    let root2 = create_test_tag("Root 2");

    let parent_id = root1.id;
    let child = Tag::with_parent(make_string("Child"), parent_id);

    repo.save(&root1).await.unwrap();
    repo.save(&root2).await.unwrap();
    repo.save(&child).await.unwrap();

    let roots = repo.find_roots().await.unwrap();
    assert!(roots.len() >= 2);
    assert!(roots.iter().any(|t| t.id == root1.id));
    assert!(roots.iter().any(|t| t.id == root2.id));
    assert!(!roots.iter().any(|t| t.id == child.id));

    // Clean up (delete child first, then roots)
    repo.delete(&child.id).await.unwrap();
    repo.delete(&root1.id).await.unwrap();
    repo.delete(&root2.id).await.unwrap();
}

/// Test delete removes tag.
pub async fn test_delete<R: TagRepository>(repo: R) {
    let tag = create_test_tag("To Delete");
    let tag_id = tag.id;

    repo.save(&tag).await.unwrap();
    repo.delete(&tag_id).await.unwrap();

    let after_delete = repo.find_by_id(&tag_id).await.unwrap();
    assert!(after_delete.is_none());
}
