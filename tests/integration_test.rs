use actix_web::{test, web, App};
use global_price_index::{controller, config};

#[actix_rt::test]
async fn test_health_check() {
    dotenv::dotenv().ok();
    let config = config::Config::from_env().expect("Failed to load configuration");
    let redis_pool = config::create_redis_pool(&config);
    let mut app = test::init_service(
        App::new()
            .app_data(web::Data::new(redis_pool.clone()))
            .configure(controller::init_routes)
    ).await;
    let req = test::TestRequest::get().uri("/health").to_request();
    let resp = test::call_service(&mut app, req).await;
    assert!(resp.status().is_success());
}

#[actix_rt::test]
async fn test_global_price_index() {
    dotenv::dotenv().ok();
    let config = config::Config::from_env().expect("Failed to load configuration");
    let redis_pool = config::create_redis_pool(&config);

    let mut app = test::init_service(
        App::new()
            .app_data(web::Data::new(redis_pool.clone()))
            .configure(controller::init_routes)
    ).await;

    let req = test::TestRequest::get().uri("/global-price-index").to_request();
    let resp = test::call_service(&mut app, req).await;
    assert!(resp.status().is_success());
}
