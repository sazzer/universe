use crate::TestService;
use actix_web::test::TestRequest;
use assert2::check;
use regex::Regex;
use std::collections::HashMap;
use universe_testdatabase::seed::SeedUser;

#[actix_rt::test]
async fn success() {
    let sut = TestService::new().await;

    let mut form = HashMap::new();
    form.insert("username", "testuser");
    form.insert("email", "testuser@example.com");
    form.insert("displayName", "Test User");
    form.insert("password", "password");

    let response = sut
        .inject(
            TestRequest::post()
                .uri("/authentication/register")
                .set_json(&form)
                .to_request(),
        )
        .await;

    check!(response.status == 200);
    check!(response.headers.get("content-type").unwrap() == "application/hal+json");
    insta::assert_json_snapshot!(response.to_json().unwrap(), {
      ".accessToken" => insta::dynamic_redaction(|value, _| {
        let value = value.as_str().unwrap();
        let regex = Regex::new(r#"^[A-Za-z0-9-_=]+\.[A-Za-z0-9-_=]+\.?[A-Za-z0-9-_.+/=]*$"#).unwrap();
        check!(regex.is_match(value));

        "<redactedAccessToken>"
      }),
      ".expires" => insta::dynamic_redaction(|value, _| {
        // 2022-01-06T07:25:35Z
        let value = value.as_str().unwrap();
        let regex = Regex::new(r#"^(-?(?:[1-9][0-9]*)?[0-9]{4})-(1[0-2]|0[1-9])-(3[01]|0[1-9]|[12][0-9])T(2[0-3]|[01][0-9]):([0-5][0-9]):([0-5][0-9])(\\.[0-9]+)?(Z)?$"#).unwrap();
        check!(regex.is_match(value));

        "<redactedExpires>"
      }),
      "[\"_links\"].related.href" => insta::dynamic_redaction(|value, _| {
        let value = value.as_str().unwrap();
        let regex = Regex::new(r#"^/users/[a-z0-9-]{36}$"#).unwrap();
        check!(regex.is_match(value));

        "<redactedUserUrl>"
      }),

    }, @r###"
    {
      "accessToken": "<redactedAccessToken>",
      "expires": "<redactedExpires>",
      "_links": {
        "related": {
          "href": "<redactedUserUrl>"
        }
      }
    }
    "###);
}

#[actix_rt::test]
async fn empty_input() {
    let sut = TestService::new().await;

    let form = HashMap::<String, String>::new();

    let response = sut
        .inject(
            TestRequest::post()
                .uri("/authentication/register")
                .set_json(&form)
                .to_request(),
        )
        .await;

    check!(response.status == 422);
    check!(response.headers.get("content-type").unwrap() == "application/problem+json");
    // TODO - This should include both username and password as missing.
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
    form.insert("email", "testuser@example.com");
    form.insert("displayName", "Test User");
    form.insert("password", "password");

    let response = sut
        .inject(
            TestRequest::post()
                .uri("/authentication/register")
                .set_json(&form)
                .to_request(),
        )
        .await;

    check!(response.status == 422);
    check!(response.headers.get("content-type").unwrap() == "application/problem+json");
    // TODO - This should include both username and password as missing.
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
async fn missing_email() {
    let sut = TestService::new().await;

    let mut form = HashMap::new();
    form.insert("username", "testuser");
    form.insert("displayName", "Test User");
    form.insert("password", "password");

    let response = sut
        .inject(
            TestRequest::post()
                .uri("/authentication/register")
                .set_json(&form)
                .to_request(),
        )
        .await;

    check!(response.status == 422);
    check!(response.headers.get("content-type").unwrap() == "application/problem+json");
    // TODO - This should include both username and password as missing.
    insta::assert_json_snapshot!(response.to_json().unwrap(), @r###"
    {
      "type": "tag:universe/2020:problems/validation_error",
      "title": "The incoming request was not valid",
      "status": 422,
      "fields": {
        "email": "tag:universe/2020:validations/missing_field"
      }
    }
    "###);
}

#[actix_rt::test]
async fn missing_display_name() {
    let sut = TestService::new().await;

    let mut form = HashMap::new();
    form.insert("username", "testuser");
    form.insert("email", "testuser@example.com");
    form.insert("password", "password");

    let response = sut
        .inject(
            TestRequest::post()
                .uri("/authentication/register")
                .set_json(&form)
                .to_request(),
        )
        .await;

    check!(response.status == 422);
    check!(response.headers.get("content-type").unwrap() == "application/problem+json");
    // TODO - This should include both username and password as missing.
    insta::assert_json_snapshot!(response.to_json().unwrap(), @r###"
    {
      "type": "tag:universe/2020:problems/validation_error",
      "title": "The incoming request was not valid",
      "status": 422,
      "fields": {
        "displayName": "tag:universe/2020:validations/missing_field"
      }
    }
    "###);
}

#[actix_rt::test]
async fn missing_password() {
    let sut = TestService::new().await;

    let mut form = HashMap::new();
    form.insert("username", "testuser");
    form.insert("email", "testuser@example.com");
    form.insert("displayName", "Test User");

    let response = sut
        .inject(
            TestRequest::post()
                .uri("/authentication/register")
                .set_json(&form)
                .to_request(),
        )
        .await;

    check!(response.status == 422);
    check!(response.headers.get("content-type").unwrap() == "application/problem+json");
    // TODO - This should include both username and password as missing.
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

#[actix_rt::test]
async fn duplicate_username() {
    let test_user = SeedUser {
        username: "testuser".to_owned(),
        ..SeedUser::default()
    }
    .with_password("correct");

    let sut = TestService::new().await;
    sut.seed(&test_user).await;

    let mut form = HashMap::new();
    form.insert("username", "testuser");
    form.insert("email", "testuser@example.com");
    form.insert("displayName", "Test User");
    form.insert("password", "password");

    let response = sut
        .inject(
            TestRequest::post()
                .uri("/authentication/register")
                .set_json(&form)
                .to_request(),
        )
        .await;

    check!(response.status == 422);
    check!(response.headers.get("content-type").unwrap() == "application/problem+json");
    // TODO - This should include both username and password as missing.
    insta::assert_json_snapshot!(response.to_json().unwrap(), @r###"
    {
      "type": "tag:universe/2020:problems/authentication/duplicate_username",
      "title": "Duplicate Username",
      "status": 422
    }
    "###);
}

#[actix_rt::test]
async fn duplicate_email() {
    let test_user = SeedUser {
        email: "testuser@example.com".to_owned(),
        ..SeedUser::default()
    }
    .with_password("correct");

    let sut = TestService::new().await;
    sut.seed(&test_user).await;

    let mut form = HashMap::new();
    form.insert("username", "testuser");
    form.insert("email", "testuser@example.com");
    form.insert("displayName", "Test User");
    form.insert("password", "password");

    let response = sut
        .inject(
            TestRequest::post()
                .uri("/authentication/register")
                .set_json(&form)
                .to_request(),
        )
        .await;

    check!(response.status == 422);
    check!(response.headers.get("content-type").unwrap() == "application/problem+json");
    // TODO - This should include both username and password as missing.
    insta::assert_json_snapshot!(response.to_json().unwrap(), @r###"
    {
      "type": "tag:universe/2020:problems/authentication/duplicate_email",
      "title": "Duplicate email address",
      "status": 422
    }
    "###);
}
