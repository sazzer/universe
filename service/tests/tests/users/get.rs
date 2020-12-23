use crate::TestService;
use actix_web::test::TestRequest;
use assert2::check;
use universe_testdatabase::seed::SeedUser;

#[actix_rt::test]
async fn get_invalid_id() {
    let sut = TestService::new().await;

    let response = sut
        .inject(TestRequest::get().uri("/users/invalid").to_request())
        .await;

    check!(response.status == 404);
    check!(response.headers.get("content-type").unwrap() == "application/problem+json");
    insta::assert_json_snapshot!(response.to_json().unwrap(), @r###"
    {
      "type": "tag:universe/2020:problems/not_found",
      "title": "The requested resource was not found",
      "status": 404
    }
    "###);
}

#[actix_rt::test]
async fn get_unknown_user() {
    let sut = TestService::new().await;

    let response = sut
        .inject(
            TestRequest::get()
                .uri("/users/0f71cb77-9b98-4db8-8b6f-4e736a34133c")
                .to_request(),
        )
        .await;

    check!(response.status == 404);
    check!(response.headers.get("content-type").unwrap() == "application/problem+json");
    insta::assert_json_snapshot!(response.to_json().unwrap(), @r###"
    {
      "type": "tag:universe/2020:problems/not_found",
      "title": "The requested resource was not found",
      "status": 404
    }
    "###);
}

#[actix_rt::test]
async fn get_populated_user() {
    let test_user = SeedUser {
        user_id: "0f71cb77-9b98-4db8-8b6f-4e736a34133c".parse().unwrap(),
        version: "588fa1b7-19f1-4366-a637-ab247238557b".parse().unwrap(),
        display_name: "Test User".to_string(),
        email: "testuser@example.com".to_owned(),
        username: "testuser".to_owned(),
        ..SeedUser::default()
    };

    let sut = TestService::new().await;
    sut.seed(&test_user).await;

    let response = sut
        .inject(
            TestRequest::get()
                .uri("/users/0f71cb77-9b98-4db8-8b6f-4e736a34133c")
                .to_request(),
        )
        .await;

    check!(response.status == 200);
    check!(response.headers.get("content-type").unwrap() == "application/vnd.siren+json");
    check!(response.headers.get("cache-control").unwrap() == "public, max-age=3600");
    check!(response.headers.get("etag").unwrap() == "\"588fa1b7-19f1-4366-a637-ab247238557b\"");
    insta::assert_json_snapshot!(response.to_json().unwrap(), @r###"
    {
      "class": [
        "tag:universe,2020:classes/user"
      ],
      "properties": {
        "displayName": "Test User",
        "email": "testuser@example.com",
        "username": "testuser"
      },
      "links": [
        {
          "href": "/users/0f71cb77-9b98-4db8-8b6f-4e736a34133c",
          "rel": [
            "self"
          ]
        }
      ]
    }
    "###);
}
