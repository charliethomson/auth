use std::time::Duration;

use data::repository::{
    application::ApplicationRepository, connect, grant::GrantRepository, user::UserRepository,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();
    dotenvy::dotenv()?;
    let conn = connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL is not set")).await?;

    let grant_repository = GrantRepository::new(conn.clone());
    let application_repository = ApplicationRepository::new(conn.clone());
    let user_repository = UserRepository::new(conn.clone());

    let mut interval = tokio::time::interval(Duration::from_secs(1));

    let app = application_repository
        .create(
            "testing_create_application",
            "dev.thmsn.app.note",
            "Testing app",
            "an app for testing shit with",
        )
        .await?;
    println!("\napp: {:#?}", app);
    interval.tick().await;

    let grants = [
        (
            "dev.thmsn.app.note.create",
            "Create a Note",
            "Allows the user to create a note",
        ),
        (
            "dev.thmsn.app.note.read",
            "Read notes",
            "Allows the user to read notes",
        ),
        (
            "dev.thmsn.app.note.edit",
            "Edit notes",
            "Allows the user to edit a note",
        ),
        (
            "dev.thmsn.app.note.delete",
            "Delete notes",
            "Allows the user to delete a note",
        ),
    ];

    let mut created_grants = vec![];

    for (id, display_name, description) in grants {
        created_grants.push(
            grant_repository
                .create(
                    &format!("testing_create_grant_{id}"),
                    id,
                    &app.application.application_id,
                    display_name,
                    description,
                )
                .await?,
        );
    }
    println!("\ncreated_grants: {:#?}", created_grants);
    interval.tick().await;

    let grants_for_app = grant_repository
        .by_application(&app.application.application_id)
        .await?;
    println!("\ngrants_for_app: {:#?}", grants_for_app);
    interval.tick().await;

    let first_grant = grant_repository.by_id(grants[0].0).await?;
    println!("\nfirst_grant: {:#?}", first_grant);
    interval.tick().await;

    let app_with_grants = application_repository
        .by_id(&app.application.application_id)
        .await?;
    println!("\napp_with_grants: {:#?}", app_with_grants);
    interval.tick().await;

    let updated_app = application_repository
        .update(
            "testing_update_application",
            &app.application.application_id,
            Some("Testing app (edited display name !)"),
            None,
        )
        .await?;
    println!("\nupdated_app: {:#?}", updated_app);
    interval.tick().await;

    let created = user_repository
        .create(
            "testing",
            "testing_create_user",
            "a fake password!",
            Some("Charlie is testing"),
            Some("charlie@thmsn.dev"),
            None,
        )
        .await?;

    println!("\ncreated: {:#?}", created);
    interval.tick().await;

    let list = user_repository.list(None).await?;
    println!("\nlist: {:#?}", list);
    interval.tick().await;

    let updated = user_repository
        .update(
            "testing_update_user",
            created.user.user_id,
            None,                                              // enabled,
            Some("Charlie is testing a changed display name"), // display_name,
            None,                                              // password,
            None,                                              // email,
            None,                                              // image_url,
        )
        .await?;
    println!("\nupdated: {:#?}", updated);
    interval.tick().await;

    let detail = user_repository.by_id(created.user.user_id).await?;
    println!("\ndetail: {:#?}", detail);
    interval.tick().await;

    for (i, (grant_id, _, _)) in grants.iter().enumerate() {
        let enabled = i % 2 == 0;

        user_repository
            .update_grant(
                &format!(
                    "testing_insert_user_grant_{grant_id}_{}",
                    if enabled { "ena" } else { "dis" },
                ),
                created.user.user_id,
                grant_id,
                enabled,
            )
            .await?;
    }
    interval.tick().await;

    let detail_with_grants = user_repository.by_id(created.user.user_id).await?;
    println!("\ndetail_with_grants: {:#?}", detail_with_grants);
    interval.tick().await;
    for (i, (grant_id, _, _)) in grants.iter().enumerate() {
        let enabled = i % 2 == 1;

        user_repository
            .update_grant(
                &format!(
                    "testing_update_user_grant_{grant_id}_{}",
                    if enabled { "ena" } else { "dis" },
                ),
                created.user.user_id,
                grant_id,
                enabled,
            )
            .await?;
    }
    interval.tick().await;

    let detail_with_grants = user_repository.by_id(created.user.user_id).await?;
    println!("\ndetail_with_grants: {:#?}", detail_with_grants);
    interval.tick().await;

    Ok(())
}
