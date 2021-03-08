use std::env;

use futures::StreamExt;
use sqlx::sqlite::SqlitePool;
use sqlx::types::chrono::Utc;
use telegram_bot::*;

async fn save_to_db(
    pool: &SqlitePool,
    first_name: &str,
    title: &str,
    content: &str,
) -> anyhow::Result<i64> {
    let mut conn = pool.acquire().await?;
    let ts = Utc::now();

    // Insert the task, then obtain the ID of this row
    let id = sqlx::query!(
        r#"
INSERT INTO telebins (first_name, title, content, ts)
VALUES (?, ?, ?, ?)
        "#,
        first_name,
        title,
        content,
        ts
    )
    .execute(&mut conn)
    .await?
    .last_insert_rowid();

    Ok(id)
}

async fn msg_handler(pool: &SqlitePool, api: &Api, message: Message) -> anyhow::Result<()> {
    if let MessageKind::Text { ref data, .. } = message.kind {
        println!("<{}>: {}", &message.from.first_name, data);

        let v: Vec<&str> = data.splitn(2, ' ').collect();

        if v.len() >= 2 {
            if v[0].len() <= 255 {
                let id = save_to_db(pool, &message.from.first_name, v[0], v[1]).await?;
                api.send(message.text_reply(format!("OK, record id: {}", id)))
                    .await?;
            } else {
                api.send(message.text_reply("标题过长，请输入：标题(长度不超过255) 内容"))
                    .await?;
            }
        } else {
            api.send(message.text_reply("格式有误，请输入：标题 内容"))
                .await?;
        }
    }
    return Ok(());
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    let api = Api::new(token);
    let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;

    let mut stream = api.stream();
    while let Some(update) = stream.next().await {
        let update = update?;
        if let UpdateKind::Message(message) = update.kind {
            msg_handler(&pool, &api, message).await?;
        }
    }
    Ok(())
}
