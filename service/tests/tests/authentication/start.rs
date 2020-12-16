use crate::TestService;
use actix_web::test::TestRequest;
use assert2::check;

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
