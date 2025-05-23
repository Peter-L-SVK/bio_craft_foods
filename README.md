# Bio Craft Foods Backend

This is the asynchronous backend service for the Bio Craft Foods mockup website, built using Rust, Tokio, Axum, and SQLx.
For for admin page, react-admin is used.
It provides RESTful APIs for managing orders, products and customers.

![Admin page demo Demo](demo.png)
 *(Backend and admin frontend running)*

## Key Updates for Alpha-2

- **Error Handling:**  
Introduced a custom AppError enum for consistent error handling across the application.  
Added validation utilities:  
&nbsp;&nbsp;&nbsp;&nbsp;validate_date: Ensures order dates are valid (e.g., after 2020-01-01).  
&nbsp;&nbsp;&nbsp;&nbsp;validate_customer_exists: Checks if a customer exists before creating or updating an order.  
&nbsp;&nbsp;&nbsp;&nbsp;validate_product_exists: Checks if a product exists before creating or updating an order.  

- **Route Handlers:**  
Updated handlers for customers, orders, and products:  
&nbsp;&nbsp;&nbsp;&nbsp;customers.rs, orders.rs, and products.rs now use AppError for error handling.  
&nbsp;&nbsp;&nbsp;&nbsp;Utility functions like json_response and content_range_header are used for consistent responses.  
&nbsp;&nbsp;&nbsp;&nbsp;Added date validation for orders using validate_date.  

- **Code Cleanup:**  
Ensured all handlers and routes are consistent and well-organized.  
Removed redundant code.  

- **Testing with Postman:**  
Kept post, put, and delete imports for testing API endpoints with Postman.  

- **Logging:**  
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
- **Tokio**: An asynchronous multithreaded runtime.
- **Axum**: A web framework for Rust that focuses on ergonomics and modularity.
- **SQLx**: An async SQL crate for Rust with compile-time SQL query checking.
- **MariaDB**: A relational database management system used for storing data, based of MySQL.
- **react-admin**: A frontend Framework for single-page applications on top of REST/GraphQL APIs, using TypeScript, React and Material Design

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
   git@github.com:Peter-L-SVK/bio_craft_foods.git
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
