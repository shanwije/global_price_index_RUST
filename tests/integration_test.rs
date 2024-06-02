use actix_web::{test, App};
use global_price_index::{config::create_redis_pool, controller::init_routes, service::AppService};

#[actix_rt::test]
async fn test_get_global_price_index() {
    dotenv::dotenv().ok();
    let config = global_price_index::config::Config::from_env().expect("Failed to load config");
    let redis_pool = create_redis_pool(&config);

    let app_service = AppService::new(redis_pool.clone());
    tokio::spawn(app_service.start_collecting_prices());

    let mut app = test::init_service(
        App::new()
            .app_data(actix_web::web::Data::new(app_service))
            .app_data(actix_web::web::Data::new(redis_pool))
            .configure(init_routes),
    )
        .await;

    let req = test::TestRequest::get().uri("/global-price-index").to_request();
    let resp = test::call_service(&mut app, req).await;
    assert!(resp.status().is_success());
}
