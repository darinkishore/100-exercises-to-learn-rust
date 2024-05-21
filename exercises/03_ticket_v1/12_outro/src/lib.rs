// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 characters.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
//Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.

pub struct Order {
    product_name: String,
    quantity: u32,
    unit_price: u32,
}

impl Order {
    fn valid_name(name: &str) {
        assert!(name.len() < 300);
        assert!(!name.is_empty());
    }
    fn valid_num(num: u32) {
        assert!(num > 0);
    }

    pub fn product_name(&self) -> &String {
        &self.product_name
    }

    pub fn quantity(&self) -> &u32 {
        &self.quantity
    }

    pub fn unit_price(&self) -> &u32 {
        &self.unit_price
    }

    pub fn set_product_name(&mut self, name: String) {
        Self::valid_name(&name);
        self.product_name = name
    }

    pub fn set_quantity(&mut self, quantity: u32) {
        Self::valid_num(quantity);
        self.quantity = quantity;
    }

    pub fn set_unit_price(&mut self, price: u32) {
        Self::valid_num(price);
        self.unit_price = price;
    }

    pub fn total(&self) -> u32 {
        &self.quantity * &self.unit_price
    }

    pub fn new(product_name: String, quantity: u32, price: u32) -> Self {
        Self::valid_num(quantity);
        Self::valid_num(price);
        Self::valid_name(&product_name);
        Self {
            product_name,
            quantity,
            unit_price: price,
        }
    }
}
