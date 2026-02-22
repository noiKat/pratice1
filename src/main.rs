struct GroceryList {
    item_quantity: i32,
    price: i32,
}

// Renamed parameter to 'item' for clarity
fn display_item(item: &GroceryList) {
    println!("Number of items: {:?}", item.item_quantity);
}

// Fixed typo in function name and updated logic
fn display_price(item: &GroceryList) {
    println!("The price is: {:?}", item.price);
}

fn main() {
    let grocery_item = GroceryList {
        item_quantity: 32,
        price: 149,
    };

    display_item(&grocery_item);
    display_price(&grocery_item);
}