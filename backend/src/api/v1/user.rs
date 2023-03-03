use actix_web::{ get, post, web, App, HttpResponse, HttpServer, Responder };

async fn get_user(path: web::Path<usize>, app_state: web::Data<AppState>) -> HttpResponse {
    let user_id: usize = path.into_inner();

    let user: sqlx::Result<Option<MySqlRow>> = sqlx::query("SELECT post_id FROM post_id")
        .bind(user_id as u64)
        .fetch_optional(&app_state.pool)
        .await;
}