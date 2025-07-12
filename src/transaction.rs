use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    id: uuid::Uuid,
    amount: f64,
    description: String,
    date: String,
    category: String,
    operation: TransactionType,
}
#[warn(dead_code)]
pub struct TransactionBuilder {
    pub id: uuid::Uuid,
    pub amount: f64,
    pub description: String,
    pub date: String,
    pub category: String,
    pub operation: TransactionType,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub enum TransactionType {
    Income,
    #[default]
    Expense,
}

impl Default for TransactionBuilder {
    fn default() -> Self {
        TransactionBuilder {
            id: uuid::Uuid::new_v4(),
            amount: 0.0,
            description: String::new(),
            date: String::new(),
            category: String::new(),
            operation: TransactionType::Expense,
        }
    }
}

impl TransactionBuilder {
    pub fn new() -> Self {
        TransactionBuilder::default()
    }

    pub fn amount(mut self, amount: f64) -> Self {
        self.amount = amount;
        self
    }

    pub fn description<S: Into<String>>(mut self, description: S) -> Self {
        self.description = description.into();
        self
    }

    pub fn date<S: Into<String>>(mut self, date: S) -> Self {
        self.date = date.into();
        self
    }

    pub fn category<S: Into<String>>(mut self, category: S) -> Self {
        self.category = category.into();
        self
    }

    pub fn operation(mut self, operation: TransactionType) -> Self {
        self.operation = operation;
        self
    }

    pub fn build(self) -> Transaction {
        Transaction {
            id: self.id,
            amount: self.amount,
            description: self.description,
            date: self.date,
            category: self.category,
            operation: self.operation,
        }
    }
}

impl Transaction {
    pub fn get_id(&self) -> uuid::Uuid {
        self.id
    }
    pub fn get_amount(&self) -> f64 {
        self.amount
    }
    pub fn get_description(&self) -> &str {
        &self.description
    }
    pub fn get_date(&self) -> &str {
        &self.date
    }
    pub fn get_category(&self) -> &str {
        &self.category
    }
    pub fn get_operation(&self) -> &TransactionType {
        &self.operation
    }
    pub fn set_amount(&mut self, amount: f64) -> Result<(), &'static str> {
        if amount < 0.0 {
            return Err("Amount cannot be negative");
        }
        self.amount = amount;
        Ok(())
    }
    pub fn set_description<S: Into<String>>(&mut self, description: S) {
        self.description = description.into();
    }
    pub fn set_date<S: Into<String>>(&mut self, date: S) {
        self.date = date.into();
    }
    pub fn set_category<S: Into<String>>(&mut self, category: S) {
        self.category = category.into();
    }
    pub fn set_operation(&mut self, operation: TransactionType) {
        self.operation = operation;
    }
}
