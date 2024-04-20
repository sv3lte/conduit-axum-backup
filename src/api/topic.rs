use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::json;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use super::{AppError, AppResponse, user};
use crate::db::{NewTopic, Topic, User};

pub async fn create_topic(
    State(pool): State<Pool<Postgres>>,
    Json(new_topic): Json<NewTopic>,
) -> Result<Json<AppResponse<Topic>>, AppError> {
    let topic: Topic = sqlx::query_as(
        r#"
            insert into topics (content, title, user_id)
            values ($1, $2, $3)
            returning *
        "#,
    )
    .bind(&new_topic.content)
    .bind(&new_topic.title)
    .bind(&new_topic.user_id)
    .fetch_one(&pool)
    .await?;

    let res = AppResponse::new(
        StatusCode::CREATED.into(),
        topic,
        "Topic create succeed.".to_string(),
    );

    println!("{:?}", res);

    Ok(Json(res))
}

pub async fn get_topic(
    State(pool): State<Pool<Postgres>>,
    Path(topic_id): Path<Uuid>,
) -> Result<Json<AppResponse<Topic>>, AppError> {
    let topic: Topic = sqlx::query_as(
        r#"
            select comments, content, create_at, favorite, _id, tags, title, update_at, user_id
            from topics
            where _id = $1
        "#,
    )
    .bind(&topic_id)
    .fetch_one(&pool)
    .await?;

    let user: User = user::query_user(pool, topic.user_id).await?;

    let mut res = AppResponse::new(
        StatusCode::OK.into(),
        topic,
        "User query succeed.".to_string(),
    );

    res.data.user = Some(json!(user));

    println!("{:?}\n", res);

    Ok(Json(res))
}

pub async fn get_topics(
    State(pool): State<Pool<Postgres>>,
) -> Result<Json<AppResponse<Vec<Topic>>>, AppError> {
    let topics: Vec<Topic> = sqlx::query_as(
        r#"
            select comments, content, create_at, favorite, _id, tags, title, update_at, user_id
            from topics
        "#
    )
    .fetch_all(&pool)
    .await?;

    let res = AppResponse::new(
        StatusCode::OK.into(),
        topics,
        "Topics query succeed.".to_string(),
    );

    println!("{:?}", res);

    Ok(Json(res))
}