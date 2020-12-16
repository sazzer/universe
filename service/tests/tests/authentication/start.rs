use std::collections::HashMap;

use crate::TestService;
use actix_web::test::TestRequest;
use assert2::check;
use universe_testdatabase::seed::SeedUser;

#[actix_rt::test]
async fn index() {
    let sut = TestService::new().await;

    let response = sut
        .inject(TestRequest::get().uri("/authentication").to_request())
        .await;

    check!(response.status == 200);
    insta::assert_json_snapshot!(response.to_json().unwrap(), @r###"
    {
      "class": [
        "authentication"
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
          "name": "start-authentication",
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
    insta::assert_json_snapshot!(response.to_json().unwrap(), @r###"
    {
      "class": [
        "authentication"
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
          "name": "register",
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
                "set-password"
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
    insta::assert_json_snapshot!(response.to_json().unwrap(), @r###"
    {
      "class": [
        "authentication"
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
          "name": "authenticate",
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
                "enter-password"
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
