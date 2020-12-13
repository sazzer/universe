use crate::TestService;
use actix_web::test::TestRequest;
use assert2::check;

#[actix_rt::test]
async fn list_authentication_providers() {
    let sut = TestService::new().await;

    let response = sut
        .inject(TestRequest::get().uri("/authentication").to_request())
        .await;

    check!(response.status == 200);
    check!(response.headers.get("content-type").unwrap() == "application/hal+json");
    insta::assert_json_snapshot!(response.to_json().unwrap(), @r###"
    {
      "_links": {
        "item": {
          "href": "/authentication/google/start",
          "name": "google"
        },
        "self": {
          "href": "/authentication"
        }
      }
    }
    "###);
}
