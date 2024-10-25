use std::io::{self, Write};

fn main() {
    // Unsigned because balance will always be positive.
    let mut balance: f32 = 0.0;
    // Will store transactions in this format : +25, -32, etc
    let mut transactions: [(char, f32); 5] = [('0', 0.0); 5];

    println!("\nWelcome to MiniBank Program!");
    println!("-------------------------------");

    // Main program loop
    loop {
        let operation_id = select_operation();
        if !is_operation_valid(&operation_id) {
            println!("\n❗ Error: Invalid input provided for 'operation'!!\n");
            continue;
        }

        if operation_id == 1 {
            check_balance(&balance);
        } else if operation_id == 2 {
            let amount = get_amount();
            if !is_valid_amount(&amount) {
                println!("\n❗ Error: Invalid input provided for 'amount'!!\n");
                continue;
            }
            balance = deposit_amount(&amount, &balance);
            add_transaction(&amount, &'+', &mut transactions);
            check_balance(&balance);
        } else if operation_id == 3 {
            let amount = get_amount();
            if !is_valid_amount(&amount) {
                println!("\n❗ Error: Invalid input provided for 'amount'!!\n");
                continue;
            }
            if !is_valid_withdrawal(&amount, &balance) {
                println!("\n❗ Error: You don't have enough balance!!\n");
                check_balance(&balance);
                continue;
            }
            balance = withdraw_amount(&amount, &balance);
            add_transaction(&amount, &'−', &mut transactions);
            check_balance(&balance);
        } else if operation_id == 4 {
            view_transactions(&transactions);
        } else {
            println!("\n🔴 Exiting the application...");
            break;
        }
    }
}

fn select_operation() -> i8 {
    let mut operation_id = String::new();

    println!();
    println!("1. Check Balance");
    println!("2. Deposit Money");
    println!("3. Withdraw Money");
    println!("4. View Transaction");
    println!("5. Exit");

    // stdout is "line buffered".
    // it means it waits until a newline to actually display the message.
    // stdout.flush() would force stdout to send its content to display immediately.
    print!("Please select an operation from the above menu (eg: 1): ");
    io::stdout().flush().expect("Failed flushing buffer");

    io::stdin()
        .read_line(&mut operation_id)
        .expect("Failed to receive user input");

    match operation_id.trim().parse() {
        Ok(num) => num,
        Err(_) => -1,
    }
}

fn is_operation_valid(operation_id: &i8) -> bool {
    *operation_id > 0 && *operation_id < 6
}

fn check_balance(balance: &f32) -> () {
    println!("\n💵 Your current balance is : {balance}$");
}

fn get_amount() -> f32 {
    let mut deposit = String::new();

    println!();
    print!("Please enter the amount: ");
    io::stdout().flush().expect("Failed flushing buffer");

    io::stdin()
        .read_line(&mut deposit)
        .expect("Failed to receive user input");

    match deposit.trim().parse::<f32>() {
        Ok(float) => float,
        Err(_) => -1.0,
    }
}

fn is_valid_amount(amount: &f32) -> bool {
    *amount > 0.0
}

fn deposit_amount(amount: &f32, balance: &f32) -> f32 {
    println!("\n✅ Depositing amount...");
    *balance + *amount
}

fn is_valid_withdrawal(amount: &f32, balance: &f32) -> bool {
    *amount < *balance
}

fn withdraw_amount(amount: &f32, balance: &f32) -> f32 {
    println!("\n❎ Withdrawing amount...");
    *balance - *amount
}

fn add_transaction(amount: &f32, transaction_type: &char, transactions: &mut [(char, f32); 5]) {
    println!("\n📝 Recording transaction...");
    let transaction: (char, f32) = (*transaction_type, *amount);
    shift_transactions(transaction, transactions);
}

fn shift_transactions(
    transaction: (char, f32),
    transactions: &mut [(char, f32); 5],
) -> (char, f32) {
    // this is de-queue operation
    for i in (0..4).rev() {
        transactions[i + 1] = transactions[i]
    }
    // this is enqueue operation
    transactions[0] = transaction;
    return transactions[0];
}

fn view_transactions(transactions: &[(char, f32); 5]) {
    println!("\n   |   ");
    for item in transactions {
        println!(" {} | {} \n", item.0, item.1);
    }
}
