use crate::helpers;
use fake::Fake;
use uuid::Uuid;

#[tokio::test]
async fn must_be_logged_in_to_see_the_change_password_form() {
    let app = helpers::spawn_app().await;

    let response = app.get_change_password().await;

    helpers::assert_is_redirect_to(&response, "/login");
}

#[tokio::test]
async fn must_be_logged_in_to_change_password() {
    let app = helpers::spawn_app().await;
    let new_password = Uuid::new_v4().to_string();

    let response = app
        .post_change_password(&serde_json::json!({
            "current_password": Uuid::new_v4().to_string(),
            "new_password": &new_password,
            "new_password_check": &new_password,
        }))
        .await;

    helpers::assert_is_redirect_to(&response, "/login");
}

#[tokio::test]
async fn new_password_fields_must_match() {
    let app = helpers::spawn_app().await;
    let new_password = Uuid::new_v4().to_string();
    let another_new_password = Uuid::new_v4().to_string();

    app.post_login(&serde_json::json!({
        "username": &app.test_user.username(),
        "password": &app.test_user.password()
    }))
    .await;

    let response = app
        .post_change_password(&serde_json::json!({
            "current_password": &app.test_user.password(),
            "new_password": &new_password,
            "new_password_check": &another_new_password
        }))
        .await;
    helpers::assert_is_redirect_to(&response, "/admin/password");

    let html_page = app.get_change_password_html().await;
    assert!(html_page.contains(
        "<p><i>You entered two different new passwords - \
        the field values must match.</i></p>"
    ));
}

#[tokio::test]
async fn current_password_must_be_valid() {
    let app = helpers::spawn_app().await;
    let new_password = Uuid::new_v4().to_string();
    let wrong_password = Uuid::new_v4().to_string();

    app.post_login(&serde_json::json!({
        "username": &app.test_user.username(),
        "password": &app.test_user.password()
    }))
    .await;

    let response = app
        .post_change_password(&serde_json::json!({
            "current_password": &wrong_password,
            "new_password": &new_password,
            "new_password_check": &new_password
        }))
        .await;

    helpers::assert_is_redirect_to(&response, "/admin/password");

    let html_page = app.get_change_password_html().await;
    assert!(html_page.contains("<p><i>The current password is incorrect.</i></p>"));
}

#[tokio::test]
async fn password_has_to_be_at_least_12_characters_long() {
    let app = helpers::spawn_app().await;
    let mut new_password = Uuid::new_v4().to_string();
    new_password.truncate(11);

    app.post_login(&serde_json::json!({
        "username": &app.test_user.username(),
        "password": &app.test_user.password()
    }))
    .await;

    let response = app
        .post_change_password(&serde_json::json!({
            "current_password": &app.test_user.password(),
            "new_password": &new_password,
            "new_password_check": &new_password
        }))
        .await;

    helpers::assert_is_redirect_to(&response, "/admin/password");

    let html_page = app.get_change_password_html().await;
    assert!(html_page.contains("<p><i>Password has to have at least 12 characters.</i></p>"));
}

#[tokio::test]
async fn password_has_to_be_shorter_than_128_characters() {
    let app = helpers::spawn_app().await;
    let new_password: String = (128..256).fake();

    app.post_login(&serde_json::json!({
        "username": &app.test_user.username(),
        "password": &app.test_user.password()
    }))
    .await;

    let response = app
        .post_change_password(&serde_json::json!({
            "current_password": &app.test_user.password(),
            "new_password": &new_password,
            "new_password_check": &new_password
        }))
        .await;

    helpers::assert_is_redirect_to(&response, "/admin/password");

    let html_page = app.get_change_password_html().await;
    assert!(html_page.contains("<p><i>Password has to be shorter than 128 characters.</i></p>"));
}

#[tokio::test]
async fn changing_password_works() {
    let app = helpers::spawn_app().await;
    let new_password = Uuid::new_v4().to_string();

    let login_body = serde_json::json!({
        "username": &app.test_user.username(),
        "password": &app.test_user.password()
    });
    let response = app.post_login(&login_body).await;
    helpers::assert_is_redirect_to(&response, "/admin/dashboard");

    let response = app
        .post_change_password(&serde_json::json!({
            "current_password": &app.test_user.password(),
            "new_password": &new_password,
            "new_password_check": &new_password,
        }))
        .await;
    helpers::assert_is_redirect_to(&response, "/admin/password");
    let html_page = app.get_change_password_html().await;
    assert!(html_page.contains("<p><i>Your password has been changed.</i></p>"));

    let response = app.post_logout().await;
    helpers::assert_is_redirect_to(&response, "/login");
    let html_page = app.get_login_html().await;
    assert!(html_page.contains("<p><i>You have successfully logged out.</i></p>"));

    let login_body = serde_json::json!({
        "username": &app.test_user.username(),
        "password": &new_password
    });
    let response = app.post_login(&login_body).await;
    helpers::assert_is_redirect_to(&response, "/admin/dashboard");
}
