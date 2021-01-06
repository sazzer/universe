use crate::TestService;
use actix_web::test::TestRequest;
use assert2::check;
use regex::Regex;
use std::collections::HashMap;
use test_case::test_case;
use universe_testdatabase::seed::SeedUser;

#[actix_rt::test]
async fn correct_password() {
    let test_user = SeedUser {
        user_id: "d6fc9429-1ee8-49f5-a218-b3d42b74de21".parse().unwrap(),
        username: "known".to_owned(),
        ..SeedUser::default()
    }
    .with_password("correct");

    let sut = TestService::new().await;
    sut.seed(&test_user).await;

    let mut form = HashMap::new();
    form.insert("username", "known");
    form.insert("password", "correct");

    let response = sut
        .inject(
            TestRequest::post()
                .uri("/authentication/authenticate")
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
    }, @r###"
    {
      "accessToken": "<redactedAccessToken>",
      "expires": "<redactedExpires>",
      "_links": {
        "related": {
          "href": "/users/d6fc9429-1ee8-49f5-a218-b3d42b74de21"
        }
      }
    }
    "###);
}

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
