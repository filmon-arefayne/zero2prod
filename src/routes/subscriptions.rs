use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(
    form: web::Form<FormData>,
    db_pool: web::Data<PgPool>,
) -> Result<HttpResponse, HttpResponse> {
    let request_id = Uuid::new_v4();
    // by using % sigil we are telling `tracing` to use the fmt::Display implementation
    // let request_span = tracing::info_span!(
    //     "Adding a new subscriber",
    //     %request_id,
    //     email = %form.email,
    //     name = %form.name
    // );
    tracing::info!(
        "request_id {} - Adding '{}' '{}' as a new subscriber.",
        request_id,
        form.email,
        form.name,
    );
    tracing::info!(
        "request_id {} - Saving new subscriber details in the database",
        request_id
    );
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(db_pool.get_ref())
    .await
    .map_err(|e| {
        tracing::error!(
            "request_id {} - Failed to execute query: {:?}",
            request_id,
            e
        );
        HttpResponse::InternalServerError().finish()
    })?;
    tracing::info!("New subscriber details have been saved");
    Ok(HttpResponse::Ok().finish())
}
