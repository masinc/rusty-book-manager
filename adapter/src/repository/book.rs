use crate::database::model::book::BookRow;
use crate::database::ConnectionPool;
use derive_new::new;
use kernel::model::book::event::CreateBook;
use kernel::model::book::Book;
use kernel::repository::book::BookRepository;
use uuid::Uuid;

#[derive(Debug, new)]
pub struct BookRepositoryImpl {
    db: ConnectionPool,
}

#[async_trait::async_trait]
impl BookRepository for BookRepositoryImpl {
    async fn create(&self, event: CreateBook) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
                INSERT INTO books (title, author, isbn, description)
                VALUES ($1, $2, $3, $4)
            "#,
            event.title,
            event.author,
            event.isbn,
            event.description
        )
        .execute(self.db.inner_ref())
        .await?;

        Ok(())
    }

    async fn find_all(&self) -> anyhow::Result<Vec<Book>> {
        let rows: Vec<BookRow> = sqlx::query_as!(
            BookRow,
            r#"
                SELECT
                    book_id,
                    title,
                    author,
                    isbn,
                    description
                FROM books
                ORDER BY created_at DESC
            "#
        )
        .fetch_all(self.db.inner_ref())
        .await?;

        Ok(rows.into_iter().map(Book::from).collect())
    }

    async fn find_by_id(&self, id: Uuid) -> anyhow::Result<Option<Book>> {
        let rows: Option<BookRow> = sqlx::query_as!(
            BookRow,
            r#"
                SELECT
                    book_id,
                    title,
                    author,
                    isbn,
                    description
                FROM books
                WHERE book_id = $1
            "#,
            id
        )
        .fetch_optional(self.db.inner_ref())
        .await?;

        Ok(rows.map(Book::from))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test]
    async fn test_register_book(pool: sqlx::PgPool) -> anyhow::Result<()> {
        let repo = BookRepositoryImpl::new(ConnectionPool::new(pool));

        let book = CreateBook {
            title: "Test Book".to_string(),
            author: "Test Author".to_string(),
            isbn: "1234567890".to_string(),
            description: "Test Description".to_string(),
        };

        repo.create(book).await?;

        let res = repo.find_all().await?;
        assert_eq!(res.len(), 1);

        let book_id = res[0].id;
        let res = repo.find_by_id(book_id).await?;
        assert_eq!(res.is_some(), true);

        let Book {
            id,
            title,
            author,
            isbn,
            description,
        } = res.unwrap();

        assert_eq!(id, book_id);
        assert_eq!(title, "Test Book");
        assert_eq!(author, "Test Author");
        assert_eq!(isbn, "1234567890");
        assert_eq!(description, "Test Description");

        Ok(())
    }
}
