# Bio Craft Foods Backend

This is the backend service for the Bio Craft Foods website, built using Rust, Axum, and SQLx.  
It provides RESTful APIs for managing orders, products and customers.

#Key Updates for Alpha-2
-**Error Handling**
Introduced a custom AppError enum for consistent error handling across the application.  
Added validation utilities:  
validate_date: Ensures order dates are valid (e.g., after 2020-01-01).  
validate_customer_exists: Checks if a customer exists before creating or updating an order.  
validate_product_exists: Checks if a product exists before creating or updating an order.  

-**Route Handlers**
Updated handlers for customers, orders, and products:  
customers.rs, orders.rs, and products.rs now use AppError for error handling.  
Utility functions like json_response and content_range_header are used for consistent responses.  
Added date validation for orders using validate_date.  

-**Code Cleanup**
Ensured all handlers and routes are consistent and well-organized.  
Removed redundant code.  

-**Testing with Postman**
Kept post, put, and delete imports for testing API endpoints with Postman.

-**Logging**
Added tracing logs for better observability and debugging.  
Logs are included for key operations like database queries and validation checks.  

## Features

- **List Orders**: Retrieve a list of all orders.
- **Get Order**: Retrieve details of a specific order by ID.
- **Create Order**: Create a new order.
- **Update Order**: Update an existing order.
- **Delete Order**: Delete an order by ID.  

Same methods with customers and products. 

## Technologies Used

- **Rust**: A systems C/C++ like programming language that is fast and memory-efficient.
- **Axum**: A web framework for Rust that focuses on ergonomics and modularity.
- **SQLx**: An async SQL crate for Rust with compile-time SQL query checking.
- **MariaDB**: A relational database management system used for storing order data, based of MySQL.

## Getting Started

### Prerequisites
- Javascript and npm manager 
- Rust and Cargo (install from [rustup.rs](https://rustup.rs/))
- SQLx CLI (install by: `cargo install sqlx-cli`)
- MariaDB (install from [MariaDB](https://mariadb.com/downloads/))
- React Admin from [React Admin](https://github.com/marmelab/react-admin) (install by `npm install react-admin`)

### Installation

1. **Clone the repository:**

   ```bash
   git clone https://github.com/your-username/bio_craft_foods.git
   cd bio_craft_foods

2. **Run the sql migration**
    
   Ensure you have MariaDB running.
```bash
   cd ./backend
   sudo systemctl status mysql
```  
   Set up the database:  
   - Create a MySQL database named bio_craft_foods.  
   - Update the database connection string in .env file:
   
```env
   DATABASE_URL=mysql://username:password@localhost/bio_craft_foods
```
Run the database migrations (if any) using SQLx CLI:
```bash
    sqlx migrate run
```   
3. **Run  the backend:**
```bash
   cd ./backend
   cargo run --release
```
4. **Run  the frontend:**
   
   Open a new terminal window (or tab) in bio_craft_foods
   
```bash
       cd ./frontend
       npm install
       npm run build
       npm run start
```
4. **Run  the  test:**
    
   To run the tests, use the following command:
```bash
   cd ./backend
   cargo test
```
