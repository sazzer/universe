use crate::TestService;
use actix_web::test::TestRequest;
use assert2::check;
use std::collections::HashMap;
use test_case::test_case;
use universe_testdatabase::seed::SeedUser;

#[actix_rt::test]
async fn unknown_user() {
    let sut = TestService::new().await;

    let mut form = HashMap::new();
    form.insert("username", "unknown");
    form.insert("password", "unknown");

    let response = sut
        .inject(
            TestRequest::post()
                .uri("/authentication/authenticate")
                .set_json(&form)
                .to_request(),
        )
        .await;

    check!(response.status == 422);
    check!(response.headers.get("content-type").unwrap() == "application/problem+json");
    insta::assert_json_snapshot!(response.to_json().unwrap(), @r###"
    {
      "type": "tag:universe/2020:problems/authentication/failed",
      "title": "Authentication failed",
      "status": 422
    }
    "###);
}

#[actix_rt::test]
async fn incorrect_password() {
    let test_user = SeedUser {
        username: "known".to_owned(),
        ..SeedUser::default()
    }
    .with_password("password");

    let sut = TestService::new().await;
    sut.seed(&test_user).await;

    let mut form = HashMap::new();
    form.insert("username", "known");
    form.insert("password", "incorrect");

    let response = sut
        .inject(
            TestRequest::post()
                .uri("/authentication/authenticate")
                .set_json(&form)
                .to_request(),
        )
        .await;

    check!(response.status == 422);
    check!(response.headers.get("content-type").unwrap() == "application/problem+json");
    insta::assert_json_snapshot!(response.to_json().unwrap(), @r###"
    {
      "type": "tag:universe/2020:problems/authentication/failed",
      "title": "Authentication failed",
      "status": 422
    }
    "###);
}

#[test_case("" ; "Blank")]
#[test_case(" " ; "Whitespace")]
#[actix_rt::test]
async fn invalid_username(input: &str) {
    let sut = TestService::new().await;

    let mut form = HashMap::new();
    form.insert("username", input);
    form.insert("password", "unknown");

    let response = sut
        .inject(
            TestRequest::post()
                .uri("/authentication/authenticate")
                .set_json(&form)
                .to_request(),
        )
        .await;

    check!(response.status == 422);
    check!(response.headers.get("content-type").unwrap() == "application/problem+json");
    insta::assert_json_snapshot!(response.to_json().unwrap(), @r###"
    {
      "type": "tag:universe/2020:problems/validation_error",
      "title": "The incoming request was not valid",
      "status": 422,
      "fields": {
        "username": "tag:universe/2020:validations/missing_field"
      }
    }
    "###);
}

#[actix_rt::test]
async fn missing_username() {
    let sut = TestService::new().await;

    let mut form = HashMap::new();
    form.insert("password", "unknown");

    let response = sut
        .inject(
            TestRequest::post()
                .uri("/authentication/authenticate")
                .set_json(&form)
                .to_request(),
        )
        .await;

    check!(response.status == 422);
    check!(response.headers.get("content-type").unwrap() == "application/problem+json");
    insta::assert_json_snapshot!(response.to_json().unwrap(), @r###"
    {
      "type": "tag:universe/2020:problems/validation_error",
      "title": "The incoming request was not valid",
      "status": 422,
      "fields": {
        "username": "tag:universe/2020:validations/missing_field"
      }
    }
    "###);
}

#[actix_rt::test]
async fn missing_password() {
    let sut = TestService::new().await;

    let mut form = HashMap::new();
    form.insert("username", "unknown");

    let response = sut
        .inject(
            TestRequest::post()
                .uri("/authentication/authenticate")
                .set_json(&form)
                .to_request(),
        )
        .await;

    check!(response.status == 422);
    check!(response.headers.get("content-type").unwrap() == "application/problem+json");
    insta::assert_json_snapshot!(response.to_json().unwrap(), @r###"
    {
      "type": "tag:universe/2020:problems/validation_error",
      "title": "The incoming request was not valid",
      "status": 422,
      "fields": {
        "password": "tag:universe/2020:validations/missing_field"
      }
    }
    "###);
}
