use sqlx::SqlitePool;

pub async fn run_migrations(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query(include_str!("../migrations/001_usuario.sql"))
        .execute(pool)
        .await?;

    sqlx::query(include_str!("../migrations/002_categoria.sql"))
        .execute(pool)
        .await?;

    sqlx::query(include_str!("../migrations/003_conta.sql"))
        .execute(pool)
        .await?;

    sqlx::query(include_str!("../migrations/004_transacao.sql"))
        .execute(pool)
        .await?;

    sqlx::query(include_str!("../migrations/005_meta_financeira.sql"))
        .execute(pool)
        .await?;

    sqlx::query(include_str!("../migrations/006_indices.sql"))
        .execute(pool)
        .await?;

    sqlx::query(include_str!("../migrations/007_trigger.sql"))
        .execute(pool)
        .await?;

    Ok(())
}
