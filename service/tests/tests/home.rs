use crate::TestService;
use actix_web::test::TestRequest;
use assert2::check;

#[actix_rt::test]
async fn test_home() {
    let sut = TestService::new().await;

    let response = sut.inject(TestRequest::get().uri("/").to_request()).await;

    check!(response.status == 200);
    check!(response.headers.get("content-type").unwrap() == "application/hal+json");
    check!(response.headers.get("cache-control").unwrap() == "public, max-age=3600");
    insta::assert_json_snapshot!(response.to_json().unwrap(), @r###"
    {
      "_links": {
        "tag:universe,2020:rels/authentication": {
          "href": "/authentication"
        }
      }
    }
    "###);
}
