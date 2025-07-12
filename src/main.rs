mod database;
mod transaction;

use database::Database;
use dialoguer::Select;
use std::env;
use std::io::{self, Write};

use crate::transaction::{TransactionBuilder, TransactionType};

fn main() {
    // Detectar si estamos en modo desarrollo o producción
    let is_development = is_development_mode();
    
    let db = if is_development {
        // Development mode - use in-memory database with sample data
        let db = Database::new_in_memory().expect("Failed to create in-memory database");
        populate_sample_data(&db);
        db
    } else {
        // Production mode - use persistent database
        Database::new("finance.db").expect("Failed to create database")
    };

    println!("Welcome to the Finance Manager!");
    if is_development {
        println!("🚧 Running in development mode with sample data");
    }

    loop {
        let options = vec![
            "💰 View account balance",
            "➕ Add a transaction",
            "📋 View all transactions",
            "🚪 Exit",
        ];

        let selection = Select::new()
            .with_prompt("What would you like to do today?")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();

        match selection {
            0 => show_balance(&db),
            1 => add_transaction(&db),
            2 => show_transactions(&db),
            3 => break,
            _ => println!("Invalid option, please try again."),
        }
    }
}

fn populate_sample_data(db: &Database) {
    println!("📝 Adding sample data for development...");

    let sample_transactions = vec![
        TransactionBuilder::new()
            .amount(1000.0)
            .description("Initial deposit")
            .category("Income")
            .date("2025-01-01")
            .operation(TransactionType::Income)
            .build(),
        TransactionBuilder::new()
            .amount(50.0)
            .description("Groceries")
            .category("Food")
            .date("2025-01-02")
            .operation(TransactionType::Expense)
            .build(),
        TransactionBuilder::new()
            .amount(25.0)
            .description("Coffee")
            .category("Food")
            .date("2025-01-03")
            .operation(TransactionType::Expense)
            .build(),
    ];

    for transaction in sample_transactions {
        if let Err(e) = db.add_transaction(&transaction) {
            eprintln!("Error adding sample transaction: {}", e);
        }
    }
}

fn show_balance(db: &Database) {
    match db.get_balance() {
        Ok(balance) => {
            println!("\n💰 Current Balance");
            println!("{}", "=".repeat(30));
            if balance >= 0.0 {
                println!("💵 S/{:.2}", balance);
            } else {
                println!("💸 S/{:.2} (Negative)", balance);
            }
            println!("{}", "=".repeat(30));
        }
        Err(e) => println!("❌ Error getting balance: {}", e),
    }
}

fn add_transaction(db: &Database) {
    println!("\n💰 Enter transaction details:");

    // Get amount with proper input handling
    let amount = loop {
        print!("Amount: S/");
        io::stdout().flush().unwrap();
        let mut amount = String::new();
        io::stdin()
            .read_line(&mut amount)
            .expect("Failed to read line");

        match amount.trim().parse::<f64>() {
            Ok(amt) if amt > 0.0 => break amt,
            Ok(_) => println!("❌ Amount must be positive. Try again."),
            Err(_) => println!("❌ Invalid amount. Please enter a number."),
        }
    };

    // Get description
    print!("Description: ");
    io::stdout().flush().unwrap();
    let mut description = String::new();
    io::stdin()
        .read_line(&mut description)
        .expect("Failed to read line");

    // Get category with selection
    let category = select_category();

    // Get transaction type with selection
    let operation = select_transaction_type();

    let transaction = TransactionBuilder::new()
        .amount(amount)
        .description(description.trim())
        .category(&category)
        .date(chrono::Utc::now().format("%Y-%m-%d").to_string())
        .operation(operation.clone())
        .build();

    match db.add_transaction(&transaction) {
        Ok(_) => {
            println!("\n✅ Transaction added successfully!");
            println!("📝 Amount: S/{:.2}", amount);
            println!("📝 Description: {}", description.trim());
            println!("📝 Category: {}", category);
            println!(
                "📝 Type: {}",
                match operation {
                    TransactionType::Income => "Income 💵",
                    TransactionType::Expense => "Expense 💸",
                }
            );
        }
        Err(e) => println!("❌ Error adding transaction: {}", e),
    }
}

fn show_transactions(db: &Database) {
    match db.get_all_transactions() {
        Ok(transactions) => {
            if transactions.is_empty() {
                println!("📭 No transactions found.");
            } else {
                println!("\n🗃️  All Transactions");
                println!("{}", "=".repeat(60));
                for transaction in transactions {
                    let (op_symbol, op_emoji) = match transaction.get_operation() {
                        TransactionType::Income => ("+", "💵"),
                        TransactionType::Expense => ("-", "💸"),
                    };
                    println!(
                        "{} {} | {}{:.2} | {} | {} {}",
                        transaction.get_date(),
                        op_emoji,
                        op_symbol,
                        transaction.get_amount(),
                        transaction.get_description(),
                        transaction.get_category(),
                        "📂"
                    );
                }
                println!("{}", "=".repeat(60));
            }
        }
        Err(e) => println!("❌ Error getting transactions: {}", e),
    }
}

fn select_category() -> String {
    let categories = vec![
        "🍔 Food",
        "🚗 Transportation",
        "🎮 Entertainment",
        "🛒 Shopping",
        "💡 Bills",
        "🏥 Health",
        "📚 Education",
        "✈️ Travel",
        "💰 Income",
        "📦 Other",
    ];

    let selection = Select::new()
        .with_prompt("📂 Select a category")
        .items(&categories)
        .default(0)
        .interact()
        .unwrap();

    // Remove emoji from the selected category
    categories[selection]
        .split(' ')
        .nth(1)
        .unwrap_or("Other")
        .to_string()
}

fn select_transaction_type() -> TransactionType {
    let options = vec!["💸 Expense", "💵 Income"];

    let selection = Select::new()
        .with_prompt("💰 Select transaction type")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match selection {
        0 => TransactionType::Expense,
        1 => TransactionType::Income,
        _ => TransactionType::Expense,
    }
}

/// Detecta si estamos en modo desarrollo basándose en varios factores
fn is_development_mode() -> bool {
    // Método 1: Variable de entorno RUST_ENV (manual override)
    if let Ok(rust_env) = env::var("RUST_ENV") {
        return rust_env != "production";
    }
    
    // Método 2: Variable de entorno CARGO (presente cuando se ejecuta con cargo)
    if env::var("CARGO").is_ok() {
        return true;
    }
    
    // Método 3: Detectar si el binario está en target/debug/
    if let Ok(current_exe) = env::current_exe() {
        if let Some(path_str) = current_exe.to_str() {
            // Si está en target/debug/, es desarrollo
            if path_str.contains("target/debug/") {
                return true;
            }
            // Si está en target/release/ pero ejecutándose desde el directorio de desarrollo
            if path_str.contains("target/release/") {
                // Verificar si estamos en el directorio de desarrollo
                if let Ok(current_dir) = env::current_dir() {
                    if current_dir.join("Cargo.toml").exists() && current_dir.join("src").exists() {
                        return true;
                    }
                }
            }
        }
    }
    
    // Por defecto, usar modo producción
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in_memory_database() {
        let db = Database::new_in_memory().expect("Failed to create in-memory database");

        let transaction = TransactionBuilder::new()
            .amount(100.0)
            .description("Test transaction")
            .category("Test")
            .date("2025-01-01")
            .operation(TransactionType::Income)
            .build();

        assert!(db.add_transaction(&transaction).is_ok());
        assert_eq!(db.get_balance().unwrap(), 100.0);
    }
}
