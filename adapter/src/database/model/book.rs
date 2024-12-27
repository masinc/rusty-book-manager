use kernel::model::book::Book;
use uuid::Uuid;

#[derive(Debug)]
pub struct BookRow {
    pub book_id: Uuid,
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub description: String,
}

impl From<BookRow> for Book {
    fn from(row: BookRow) -> Self {
        let BookRow {
            book_id,
            title,
            author,
            isbn,
            description,
        } = row;

        Book {
            id: book_id,
            title,
            author,
            isbn,
            description,
        }
    }
}
