use crate::transaction::{Transaction, TransactionType};
use rusqlite::{Connection, Result};

pub struct Database {
    connection: Connection,
}

impl Database {
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        Self::init_schema(&conn)?;
        Ok(Database { connection: conn })
    }

    // For development - always fresh in-memory database
    pub fn new_in_memory() -> Result<Self> {
        let conn = Connection::open(":memory:")?;
        Self::init_schema(&conn)?;
        Ok(Database { connection: conn })
    }

    fn init_schema(conn: &Connection) -> Result<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS transactions (
                id TEXT PRIMARY KEY,
                amount REAL NOT NULL,
                description TEXT NOT NULL,
                date TEXT NOT NULL,
                category TEXT NOT NULL,
                operation TEXT NOT NULL
            )",
            [],
        )?;
        Ok(())
    }

    pub fn add_transaction(&self, transaction: &Transaction) -> Result<()> {
        self.connection.execute(
            "INSERT INTO transactions (id, amount, description, date, category, operation)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            rusqlite::params![
                transaction.get_id().to_string(),
                transaction.get_amount(),
                transaction.get_description(),
                transaction.get_date(),
                transaction.get_category(),
                match transaction.get_operation() {
                    TransactionType::Income => "Income",
                    TransactionType::Expense => "Expense",
                }
            ],
        )?;
        Ok(())
    }

    pub fn get_balance(&self) -> Result<f64> {
        let mut stmt = self.connection.prepare(
            "SELECT SUM(CASE WHEN operation = 'Income' THEN amount ELSE -amount END) as balance
             FROM transactions",
        )?;

        let balance: f64 = stmt.query_row([], |row| Ok(row.get(0).unwrap_or(0.0)))?;

        Ok(balance)
    }

    pub fn get_all_transactions(&self) -> Result<Vec<Transaction>> {
        let mut stmt = self.connection.prepare(
            "SELECT id, amount, description, date, category, operation FROM transactions",
        )?;

        let transaction_iter = stmt.query_map([], |row| {
            Ok(crate::transaction::TransactionBuilder::new()
                .amount(row.get(1)?)
                .description(row.get::<_, String>(2)?)
                .date(row.get::<_, String>(3)?)
                .category(row.get::<_, String>(4)?)
                .operation(match row.get::<_, String>(5)?.as_str() {
                    "Income" => TransactionType::Income,
                    _ => TransactionType::Expense,
                })
                .build())
        })?;

        let mut transactions = Vec::new();
        for transaction in transaction_iter {
            transactions.push(transaction?);
        }

        Ok(transactions)
    }
}
