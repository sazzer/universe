use crate::TestService;
use actix_web::test::TestRequest;
use assert2::check;
use std::collections::HashMap;
use test_case::test_case;
use universe_testdatabase::seed::SeedUser;

#[actix_rt::test]
async fn index() {
    let sut = TestService::new().await;

    let response = sut
        .inject(TestRequest::get().uri("/authentication").to_request())
        .await;

    check!(response.status == 200);
    check!(response.headers.get("content-type").unwrap() == "application/vnd.siren+json");
    check!(response.headers.get("cache-control").unwrap() == "public, max-age=3600");
    insta::assert_json_snapshot!(response.to_json().unwrap(), @r###"
    {
      "class": [
        "tag:universe,2020:classes/authentication"
      ],
      "properties": null,
      "links": [
        {
          "href": "/authentication",
          "rel": [
            "self"
          ]
        }
      ],
      "actions": [
        {
          "name": "tag:universe,2020:actions/authentication/start",
          "method": "POST",
          "href": "/authentication",
          "fields": [
            {
              "name": "username",
              "type": "text"
            }
          ],
          "type": "application/x-www-form-urlencoded"
        }
      ]
    }
    "###);
}

#[actix_rt::test]
async fn start_unknown_user() {
    let sut = TestService::new().await;

    let mut form = HashMap::new();
    form.insert("username", "unknown");

    let response = sut
        .inject(
            TestRequest::post()
                .uri("/authentication")
                .set_form(&form)
                .to_request(),
        )
        .await;

    check!(response.status == 200);
    check!(response.headers.get("content-type").unwrap() == "application/vnd.siren+json");
    check!(response.headers.get("cache-control").unwrap() == "no-cache");
    insta::assert_json_snapshot!(response.to_json().unwrap(), @r###"
    {
      "class": [
        "tag:universe,2020:classes/authentication"
      ],
      "properties": null,
      "links": [
        {
          "href": "/authentication",
          "rel": [
            "self"
          ]
        }
      ],
      "actions": [
        {
          "name": "tag:universe,2020:actions/authentication/register",
          "method": "POST",
          "href": "/authentication/register",
          "fields": [
            {
              "name": "username",
              "type": "hidden",
              "value": "unknown"
            },
            {
              "name": "email",
              "type": "email"
            },
            {
              "name": "display_name",
              "type": "text"
            },
            {
              "name": "password",
              "class": [
                "tag:universe,2020:classes/authentication/password/set"
              ],
              "type": "password"
            }
          ],
          "type": "application/x-www-form-urlencoded"
        }
      ]
    }
    "###);
}

#[actix_rt::test]
async fn start_known_user() {
    let test_user = SeedUser {
        username: "known".to_owned(),
        ..SeedUser::default()
    };

    let sut = TestService::new().await;
    sut.seed(&test_user).await;

    let mut form = HashMap::new();
    form.insert("username", "known");

    let response = sut
        .inject(
            TestRequest::post()
                .uri("/authentication")
                .set_form(&form)
                .to_request(),
        )
        .await;

    check!(response.status == 200);
    check!(response.headers.get("content-type").unwrap() == "application/vnd.siren+json");
    check!(response.headers.get("cache-control").unwrap() == "no-cache");
    insta::assert_json_snapshot!(response.to_json().unwrap(), @r###"
    {
      "class": [
        "tag:universe,2020:classes/authentication"
      ],
      "properties": null,
      "links": [
        {
          "href": "/authentication",
          "rel": [
            "self"
          ]
        }
      ],
      "actions": [
        {
          "name": "tag:universe,2020:actions/authentication/authenticate",
          "method": "POST",
          "href": "/authentication/authenticate",
          "fields": [
            {
              "name": "username",
              "type": "hidden",
              "value": "known"
            },
            {
              "name": "password",
              "class": [
                "tag:universe,2020:classes/authentication/password/enter"
              ],
              "type": "password"
            }
          ],
          "type": "application/x-www-form-urlencoded"
        }
      ]
    }
    "###);
}

#[test_case("" ; "Blank")]
#[test_case(" " ; "Whitespace")]
#[actix_rt::test]
async fn start_invalid_username(input: &str) {
    let test_user = SeedUser {
        username: "known".to_owned(),
        ..SeedUser::default()
    };

    let sut = TestService::new().await;
    sut.seed(&test_user).await;

    let mut form = HashMap::new();
    form.insert("username", input);

    let response = sut
        .inject(
            TestRequest::post()
                .uri("/authentication")
                .set_form(&form)
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
async fn start_missing_username() {
    let test_user = SeedUser {
        username: "known".to_owned(),
        ..SeedUser::default()
    };

    let sut = TestService::new().await;
    sut.seed(&test_user).await;

    let form = HashMap::<&str, &str>::new();

    let response = sut
        .inject(
            TestRequest::post()
                .uri("/authentication")
                .set_form(&form)
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
