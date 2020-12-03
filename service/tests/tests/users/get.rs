use crate::TestService;
use actix_web::test::TestRequest;
use assert2::check;

#[actix_rt::test]
async fn test_get_invalid_id() {
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
async fn test_get_unknown_user() {
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
