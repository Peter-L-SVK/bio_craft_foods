# Bio Craft Foods Backend

This is the backend service for the Bio Craft Foods application, built using Rust, Axum, and SQLx. It provides RESTful APIs for managing orders.

## Features

- **List Orders**: Retrieve a list of all orders.
- **Get Order**: Retrieve details of a specific order by ID.
- **Create Order**: Create a new order.
- **Update Order**: Update an existing order.
- **Delete Order**: Delete an order by ID.

Same methods with customers and products. 

## Technologies Used

- **Rust**: A systems programming language that is fast and memory-efficient.
- **Axum**: A web framework for Rust that focuses on ergonomics and modularity.
- **SQLx**: An async SQL crate for Rust with compile-time SQL query checking.
- **MariaDB**: A relational database management system used for storing order data based of MySQL.

## Getting Started

### Prerequisites

- Rust and Cargo (install from [rustup.rs](https://rustup.rs/))
- SQLx (install by: `cargo install sqlx-cli`)
- MariaDB (install from [MariaDB](https://mariadb.com/downloads/))
- React Admin from [React Admin](https://github.com/marmelab/react-admin) )

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
   cargo run --build
```
4. **Run  the backend:**
   
   Open new terminal window in bio_craft_foods
   
```bash
       cd ./frontend
       npm install
       npm run build
       npm run
```
4. **Run  the  test:**
    
   To run the tests, use the following command:
```bash
   cd ./backend
   cargo test
```
