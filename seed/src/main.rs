use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use data::repository::{
    application::ApplicationRepository, connect, grant::GrantRepository, user::UserRepository,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();
    dotenvy::dotenv()?;
    let conn = connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL is not set")).await?;

    let salt = SaltString::generate(&mut OsRng);
    let argon = Argon2::default();

    let grant_repository = GrantRepository::new(conn.clone());
    let application_repository = ApplicationRepository::new(conn.clone());
    let user_repository = UserRepository::new(conn.clone());

    let app_id = "dev.thmsn.auth";
    let app_display_name = "Auth Service";
    let app_description = "This system";
    let app_grants = vec![
        // user management
        (
            "dev.thmsn.auth.user.create".to_string(),
            "Create User".to_string(),
            "Ability to create new user accounts".to_string(),
        ),
        (
            "dev.thmsn.auth.user.delete".to_string(),
            "Delete User".to_string(),
            "Ability to remove user accounts from the system".to_string(),
        ),
        (
            "dev.thmsn.auth.user.list".to_string(),
            "List Users".to_string(),
            "Ability to view all user accounts".to_string(),
        ),
        (
            "dev.thmsn.auth.user.get".to_string(),
            "View User".to_string(),
            "Ability to retrieve individual user account details".to_string(),
        ),
        (
            "dev.thmsn.auth.user.grant.update".to_string(),
            "Modify User Grants".to_string(),
            "Ability to assign or revoke permissions for users".to_string(),
        ),
        // app management
        (
            "dev.thmsn.auth.application.create".to_string(),
            "Create Application".to_string(),
            "Ability to register new applications".to_string(),
        ),
        (
            "dev.thmsn.auth.application.get".to_string(),
            "View Application".to_string(),
            "Ability to retrieve individual application details".to_string(),
        ),
        (
            "dev.thmsn.auth.application.list".to_string(),
            "List Applications".to_string(),
            "Ability to view all registered applications".to_string(),
        ),
        (
            "dev.thmsn.auth.application.get_grants".to_string(),
            "View Application Grants".to_string(),
            "Ability to retrieve permissions assigned to an application".to_string(),
        ),
        // grant management
        (
            "dev.thmsn.auth.grant.create".to_string(),
            "Create Grant".to_string(),
            "Ability to define new permissions".to_string(),
        ),
        (
            "dev.thmsn.auth.grant.get".to_string(),
            "View Grant".to_string(),
            "Ability to retrieve individual permission details".to_string(),
        ),
    ];

    let admin_username = "admin";
    let admin_password_raw = "admin";
    let admin_password = argon
        .hash_password(admin_password_raw.as_bytes(), salt.as_salt())
        .unwrap();

    let agent = "seed";

    let app = application_repository
        .create(agent, app_id, app_display_name, app_description)
        .await?;

    for (grant_id, display_name, description) in &app_grants {
        grant_repository
            .create(
                agent,
                &grant_id,
                &app.application.application_id,
                &display_name,
                &description,
            )
            .await?;
    }

    let admin = user_repository
        .create(
            agent,
            admin_username,
            &admin_password.to_string(),
            Some("Admin"),
            None,
            None,
        )
        .await?;

    for (grant_id, _, _) in &app_grants {
        user_repository
            .update_grant(agent, admin.user.user_id, &grant_id, true)
            .await?;
    }

    Ok(())
}
