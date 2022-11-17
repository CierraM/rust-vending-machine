use std::io;

fn main() {
    // each product should have a name, quantity, and price
    #[derive(Debug)]
    struct Product {
        name: String,
        price: i64,
        quantity: i64
    }

    //initialize product list
    let mut products = vec![
        Product {
            name: String::from("Umbrella"),
            price: 15,
            quantity: 1
        },
        Product {
            name: String::from("Spaghetti"),
            price: 6,
            quantity: 5
        },
        Product {
            name: String::from("Hand Sanitizer"),
            price: 1,
            quantity: 5
        },
        Product {
            name: String::from("USB-C to HDMI Cable"),
            price: 15,
            quantity: 5
        },
        Product {
            name: String::from("Socks"),
            price: 6,
            quantity: 5
        },
        Product {
            name: String::from("Tylenol"),
            price: 6,
            quantity: 5
        },
        Product {
            name: String::from("Pencil"),
            price: 1,
            quantity: 5
        },
        Product {
            name: String::from("Tape"),
            price: 1,
            quantity: 5
        },
        Product {
            name: String::from("Gold Coin"),
            price: 2000,
            quantity: 7
        },
        Product {
            name: String::from("Tree Branch"),
            price: 1,
            quantity: 1
        },
        Product {
            name: String::from("Ostriche Egg"),
            price: 5,
            quantity: 10
        }
    ];

        'main: loop {
        // prompt to choose a product
        println!("Welcome to this virtual vending machine.");
        println!("Please enter a product number to choose a product.");
        println!("");
        let mut selection = String::new();
        
        // display available products and quantities in a formatted way
        // Create the table
        println!(
            "{0: <2} | {1: <25} | {2: <10} | {3: <10}",
            "#", "PRODUCT", "PRICE", "STOCK"
        );
        println!("----------------------------------------------------");

        for (index, product) in products.iter().enumerate() {
            if product.quantity > 0 {
                println!("{0: <2} | {1: <25} | {2: <10} | {3: <10}", index, product.name, product.price, product.quantity);
            }
        }
        println!("");

        io::stdin()
            .read_line(&mut selection)
            .expect("Failed to read line");

        let selection: usize = match selection.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if products[selection].quantity == 0 {
            println!("Sorry, we're all out of that");
            println!("Buy something else? [y] [n]");
            println!("");

        let mut goAgain = String::new();
        io::stdin()
            .read_line(&mut goAgain)
            .expect("Failed to read line");

        if goAgain.trim().eq(&String::from("n")) || goAgain.trim().eq(&String::from("q")) {
            break;
        }
        println!("");
            continue
        }
        println!("You chose {:?}", products[selection].name);
        // prompt to insert coins until the correct amount is inserted
        println!("Press enter to insert a coin. Press q to cancel.");
        // display total inserted
        let mut coins: i32 = 0;
        let mut remaining: i64 = products[selection].price;
        loop {
            let mut input = String::new();
            io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
            
            if input.trim().eq(&String::from("q")) {
                println!("Returning {} coins", coins);
                println!("Goodbye!");
                break 'main;
            }

            coins += 1;
            remaining -= 1;
            println!("Coins inserted: {}", coins);
            if remaining == 0 {
                println!("You received {}!", products[selection].name);
                //remove item from stock
                products[selection].quantity -= 1;
                //maybe play a sound
                break;
            }
            println!("You still need {} coins", remaining)
        }
        println!("Buy something else? [y] [n]");
        println!("");

        let mut goAgain = String::new();
        io::stdin()
            .read_line(&mut goAgain)
            .expect("Failed to read line");

        if goAgain.trim().eq(&String::from("n")) || goAgain.trim().eq(&String::from("q")) {
            break;
        }
        println!("");
    }
}

