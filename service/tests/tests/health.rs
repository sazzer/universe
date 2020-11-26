use crate::TestService;
use actix_web::test::TestRequest;
use assert2::check;

#[actix_rt::test]
async fn test_healthchecks() {
    let sut = TestService::new().await;

    let response = sut
        .inject(TestRequest::get().uri("/health").to_request())
        .await;

    check!(response.status == 404); // Change when the healthchecks are implemented
}
