use super::task::{NewTask, Task};
use sqlx::PgPool;

impl Task {
    pub async fn find_all(pool: &PgPool) -> Result<Vec<Task>, sqlx::Error> {
        sqlx::query_as!(
            Task,
            r#"
SELECT *
FROM tasks
ORDER BY id
            "#
        )
        .fetch_all(pool)
        .await
    }

    pub async fn insert(pool: &PgPool, new_task: NewTask) -> Result<Task, sqlx::Error> {
        sqlx::query_as!(
            Task,
            r#"
INSERT INTO tasks ( description )
VALUES ( $1 )
RETURNING *
            "#,
            new_task.description
        )
        .fetch_one(pool)
        .await
    }
}
