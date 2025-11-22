//! Test macros for running repository contract tests

/// Run all repository tests for a complete set of implementations.
///
/// # Example
///
/// ```ignore
/// use sawa_repository_tests::test_all_repositories;
///
/// test_all_repositories! {
///     product => InMemoryProductRepository::new(),
///     product_variant => InMemoryProductVariantRepository::new(),
///     product_instance => InMemoryProductInstanceRepository::new(),
///     purchase_order => InMemoryPurchaseOrderRepository::new(),
///     user => InMemoryUserRepository::new(),
///     user_transaction => InMemoryUserTransactionRepository::new(),
///     media => InMemoryMediaRepository::new(),
///     tag => InMemoryTagRepository::new(),
/// }
/// ```
#[macro_export]
macro_rules! test_all_repositories {
    (
        product => $product_repo:expr,
        product_variant => $variant_repo:expr,
        product_instance => $instance_repo:expr,
        purchase_order => $order_repo:expr,
        user => $user_repo:expr,
        user_transaction => $transaction_repo:expr,
        media => $media_repo:expr,
        tag => $tag_repo:expr $(,)?
    ) => {
        use sawa_repository_tests::tokio;

        mod product_repository_tests {
            use super::*;

            #[$crate::tokio::test]
            async fn find_by_id_returns_none_for_non_existent() {
                let repo = $product_repo;
                $crate::suites::product::test_find_by_id_returns_none_for_non_existent(repo).await;
            }

            #[$crate::tokio::test]
            async fn save_and_find_by_id() {
                let repo = $product_repo;
                $crate::suites::product::test_save_and_find_by_id(repo).await;
            }

            #[$crate::tokio::test]
            async fn find_all() {
                let repo = $product_repo;
                $crate::suites::product::test_find_all(repo).await;
            }

            #[$crate::tokio::test]
            async fn delete() {
                let repo = $product_repo;
                $crate::suites::product::test_delete(repo).await;
            }
        }

        mod product_variant_repository_tests {
            use super::*;

            #[$crate::tokio::test]
            async fn save_and_find_by_id() {
                let repo = $variant_repo;
                $crate::suites::product_variant::test_save_and_find_by_id(repo).await;
            }

            #[$crate::tokio::test]
            async fn find_by_product_id() {
                let repo = $variant_repo;
                $crate::suites::product_variant::test_find_by_product_id(repo).await;
            }

            #[$crate::tokio::test]
            async fn find_by_tags_all() {
                let repo = $variant_repo;
                $crate::suites::product_variant::test_find_by_tags_all(repo).await;
            }

            #[$crate::tokio::test]
            async fn find_by_tags_any() {
                let repo = $variant_repo;
                $crate::suites::product_variant::test_find_by_tags_any(repo).await;
            }

            #[$crate::tokio::test]
            async fn delete() {
                let repo = $variant_repo;
                $crate::suites::product_variant::test_delete(repo).await;
            }
        }

        mod product_instance_repository_tests {
            use super::*;

            #[$crate::tokio::test]
            async fn save_and_find_by_id() {
                let repo = $instance_repo;
                $crate::suites::product_instance::test_save_and_find_by_id(repo).await;
            }

            #[$crate::tokio::test]
            async fn find_by_owner() {
                let repo = $instance_repo;
                $crate::suites::product_instance::test_find_by_owner(repo).await;
            }

            #[$crate::tokio::test]
            async fn find_by_owner_and_variant() {
                let repo = $instance_repo;
                $crate::suites::product_instance::test_find_by_owner_and_variant(repo).await;
            }

            #[$crate::tokio::test]
            async fn find_by_owner_and_status() {
                let repo = $instance_repo;
                $crate::suites::product_instance::test_find_by_owner_and_status(repo).await;
            }

            #[$crate::tokio::test]
            async fn delete() {
                let repo = $instance_repo;
                $crate::suites::product_instance::test_delete(repo).await;
            }

            // Permission isolation tests
            #[$crate::tokio::test]
            async fn find_by_owner_permission_isolation() {
                let repo = $instance_repo;
                $crate::suites::product_instance::test_find_by_owner_permission_isolation(repo)
                    .await;
            }

            #[$crate::tokio::test]
            async fn find_by_owner_and_variant_permission() {
                let repo = $instance_repo;
                $crate::suites::product_instance::test_find_by_owner_and_variant_permission(repo)
                    .await;
            }

            #[$crate::tokio::test]
            async fn find_by_owner_and_status_permission() {
                let repo = $instance_repo;
                $crate::suites::product_instance::test_find_by_owner_and_status_permission(repo)
                    .await;
            }
        }

        mod purchase_order_repository_tests {
            use super::*;

            #[$crate::tokio::test]
            async fn save_and_find_by_id() {
                let repo = $order_repo;
                $crate::suites::purchase_order::test_save_and_find_by_id(repo).await;
            }

            #[$crate::tokio::test]
            async fn find_by_user_without_status_filter() {
                let repo = $order_repo;
                $crate::suites::purchase_order::test_find_by_user_without_status_filter(repo).await;
            }

            #[$crate::tokio::test]
            async fn find_by_user_with_status() {
                let repo = $order_repo;
                $crate::suites::purchase_order::test_find_by_user_with_status(repo).await;
            }

            #[$crate::tokio::test]
            async fn delete() {
                let repo = $order_repo;
                $crate::suites::purchase_order::test_delete(repo).await;
            }

            // Permission isolation tests
            #[$crate::tokio::test]
            async fn find_by_user_permission_isolation() {
                let repo = $order_repo;
                $crate::suites::purchase_order::test_find_by_user_permission_isolation(repo).await;
            }

            #[$crate::tokio::test]
            async fn find_by_user_and_status_permission() {
                let repo = $order_repo;
                $crate::suites::purchase_order::test_find_by_user_and_status_permission(repo).await;
            }
        }

        mod user_repository_tests {
            use super::*;

            #[$crate::tokio::test]
            async fn save_and_find_by_id() {
                let repo = $user_repo;
                $crate::suites::user::test_save_and_find_by_id(repo).await;
            }

            #[$crate::tokio::test]
            async fn find_by_email() {
                let repo = $user_repo;
                $crate::suites::user::test_find_by_email(repo).await;
            }

            #[$crate::tokio::test]
            async fn find_by_username() {
                let repo = $user_repo;
                $crate::suites::user::test_find_by_username(repo).await;
            }

            #[$crate::tokio::test]
            async fn find_by_email_returns_correct_user() {
                let repo = $user_repo;
                $crate::suites::user::test_find_by_email_returns_correct_user(repo).await;
            }

            #[$crate::tokio::test]
            async fn find_by_username_returns_correct_user() {
                let repo = $user_repo;
                $crate::suites::user::test_find_by_username_returns_correct_user(repo).await;
            }

            #[$crate::tokio::test]
            async fn duplicate_and_not_found() {
                let repo = $user_repo;
                $crate::suites::user::test_duplicate_and_not_found(repo).await;
            }

            #[$crate::tokio::test]
            async fn delete() {
                let repo = $user_repo;
                $crate::suites::user::test_delete(repo).await;
            }
        }

        mod user_transaction_repository_tests {
            use super::*;

            #[$crate::tokio::test]
            async fn save_and_find_by_id() {
                let repo = $transaction_repo;
                $crate::suites::user_transaction::test_save_and_find_by_id(repo).await;
            }

            #[$crate::tokio::test]
            async fn find_by_from_user_permission() {
                let repo = $transaction_repo;
                $crate::suites::user_transaction::test_find_by_from_user_permission(repo).await;
            }

            #[$crate::tokio::test]
            async fn find_by_to_user_permission() {
                let repo = $transaction_repo;
                $crate::suites::user_transaction::test_find_by_to_user_permission(repo).await;
            }

            #[$crate::tokio::test]
            async fn find_by_from_user_with_status() {
                let repo = $transaction_repo;
                $crate::suites::user_transaction::test_find_by_from_user_with_status(repo).await;
            }

            #[$crate::tokio::test]
            async fn find_by_to_user_with_status() {
                let repo = $transaction_repo;
                $crate::suites::user_transaction::test_find_by_to_user_with_status(repo).await;
            }

            #[$crate::tokio::test]
            async fn delete() {
                let repo = $transaction_repo;
                $crate::suites::user_transaction::test_delete(repo).await;
            }
        }

        mod media_repository_tests {
            use super::*;

            #[$crate::tokio::test]
            async fn save_and_find_by_id() {
                let repo = $media_repo;
                $crate::suites::media::test_save_and_find_by_id(repo).await;
            }

            #[$crate::tokio::test]
            async fn find_by_ids() {
                let repo = $media_repo;
                $crate::suites::media::test_find_by_ids(repo).await;
            }

            #[$crate::tokio::test]
            async fn delete() {
                let repo = $media_repo;
                $crate::suites::media::test_delete(repo).await;
            }
        }

        mod tag_repository_tests {
            use super::*;

            #[$crate::tokio::test]
            async fn save_and_find_by_id() {
                let repo = $tag_repo;
                $crate::suites::tag::test_save_and_find_by_id(repo).await;
            }

            #[$crate::tokio::test]
            async fn find_all() {
                let repo = $tag_repo;
                $crate::suites::tag::test_find_all(repo).await;
            }

            #[$crate::tokio::test]
            async fn find_by_name_prefix() {
                let repo = $tag_repo;
                $crate::suites::tag::test_find_by_name_prefix(repo).await;
            }

            #[$crate::tokio::test]
            async fn find_by_parent() {
                let repo = $tag_repo;
                $crate::suites::tag::test_find_by_parent(repo).await;
            }

            #[$crate::tokio::test]
            async fn find_roots() {
                let repo = $tag_repo;
                $crate::suites::tag::test_find_roots(repo).await;
            }

            #[$crate::tokio::test]
            async fn delete() {
                let repo = $tag_repo;
                $crate::suites::tag::test_delete(repo).await;
            }
        }
    };
}
