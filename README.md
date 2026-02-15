# Rust Modular Hexagonal API Template 🦀

A production-ready template for building scalable, maintainable, and testable REST APIs in Rust using **Hexagonal Architecture (Ports and Adapters)**.

This template is designed to help you hit the ground running with a solid foundation, pre-configured with best practices for error handling, logging, configuration, and database management.

## 🚀 Features

- **Hexagonal Architecture**: Clear separation of concerns (Domain, Application, Infrastructure, Interfaces).
- **Modular Design**: Features are organized into self-contained modules (e.g., `auth`, `users`, `posts`).
- **Web Framework**: Built with [Actix Web](https://actix.rs/), a powerful and fast web framework.
- **Database**: [Diesel ORM](https://diesel.rs/) with PostgreSQL for type-safe database interactions.
- **Authentication**: JWT-based authentication and Argon2 password hashing.
- **Configuration**: Type-safe configuration management using `dotenvy`.
- **Logging & Tracing**: Structured logging with `tracing` and `tracing-subscriber`.
- **Error Handling**: Centralized and strict error handling using `thiserror`.

## 🏗️ Architecture

The project follows the **Hexagonal Architecture** pattern, dividing the application into distinct layers:

1.  **Domain**: The core business logic and entities. Contains the "Ports" (traits) that define interactions with the outside world. This layer has **no dependencies** on outer layers.
2.  **Application**: Contains the business rules and use cases. Implements the service logic using the domain entities.
3.  **Infrastructure**: The "Adapters" implementation. Contains code to talk to databases, external APIs, etc. (e.g., Diesel repositories).
4.  **Interfaces**: The entry points to the application. In this template, it's primarily the HTTP REST API (Actix Web handlers).

### Module Structure

Each feature is encapsulated in `src/modules/<feature_name>`:

```
src/modules/users/
├── domain/         # Entities, Repository Traits (Ports), Value Objects
├── application/    # Service implementations, DTOs
├── infrastructure/ # Database implementations (Adapters)
└── interfaces/     # HTTP Routes & Handlers
```

## 🛠️ Prerequisites

Before you begin, ensure you have the following installed:

- **Rust & Cargo**: [Install Rust](https://www.rust-lang.org/tools/install)
- **PostgreSQL**: Local installation or via Docker.
- **Diesel CLI**: Required for running migrations.
  ```bash
  # Install diesel_cli for Postgres only (faster build)
  cargo install diesel_cli --no-default-features --features postgres
  ```

## 🏁 Getting Started

### 1. Clone & Rename

Download or clone this repository to your machine.

If you are using this as a template for a new project, you should rename the project in `Cargo.toml`:

```toml
[package]
name = "your-new-project-name"
```

_Tip: user your IDE to find and replace "rust-modular-hexagonal-api-template" globally._

### 2. Environment Setup

Copy the template environment file:

```bash
cp .env.template .env
```

Edit `.env` and configure your database credentials and other settings:

```ini
DATABASE_URL=postgres://user:password@localhost/your_db_name
JWT_SECRET=your_super_secret_key
# ...
```

### 3. Database Setup

Make sure your Postgres server is running, then setup the database and run migrations:

```bash
diesel setup
diesel migration run
```

### 4. Run the Application

Start the development server:

```bash
cargo run
```

The server will start at `http://127.0.0.1:8080` (or the port defined in your `.env`).

## 📂 Project Structure

```
├── .env                  # Environment variables
├── Cargo.toml            # Project dependencies and metadata
├── diesel.toml           # Diesel CLI config
├── migrations/           # Database migrations
└── src/
    ├── common/           # Shared utilities (DbPool, Config, Errors, Logging)
    ├── modules/          # Feature modules (Auth, Users, Posts, etc.)
    │   └── <module>/
    │       ├── application/
    │       ├── domain/
    │       ├── infrastructure/
    │       └── interfaces/
    ├── schema.rs         # Auto-generated Diesel schema
    └── main.rs           # Application entry point
```

## 🧪 Testing

Run the test suite with:

```bash
cargo test
```

## 📝 License

This project is released under the **Unlicense**.
This means it is **Public Domain**. You can copy, modify, distribute, and use this code for any purpose, commercial or non-commercial, without any restrictions and without the need for attribution.

Enjoy building! 🚀
