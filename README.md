# GraphQL API with Rust, async-graphql, actix-web, and Postgres

This repository contains a template for building a GraphQL API using Rust🦀, async-graphql, actix-web, and Postgres.

## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes.

### Prerequisites

Before you begin, ensure you have met the following requirements:

* You have installed Rust. If not, follow the instructions on the official [Rust website](https://www.rust-lang.org/tools/install).
* You have installed Postgres. If not, follow the instructions on the official [Postgres website](https://www.postgresql.org/download/).
* You have installed `diesel-cli`. If not, follow instruction from their official site [Diesel.rs](https://diesel.rs/guides/getting-started#:~:text=Installing%20Diesel%20CLI) or using the following

```bash
cargo install diesel_cli --no-default-features --features postgres
```

### Getting started

1. Clone the repository

```bash
git clone https://github.com/korir248/actixweb-async-graphql.git
```

2. Navigate to the project directory:

```bash
 cd actixweb-async-graphql
 ```

3. Setup database with diesel cli tool using .env file

Create a `.env` file using `.env.example.txt` as a template

```bash
diesel setup
```

5. Run the application

```bash
cargo run
```

## Usage

After starting the server, you can interact with the interactive GraphiQL interface at <http://localhost:4000>.

## Contributing

If you want to contribute to this project, please fork it and send a pull request.

## License

This project uses the following license: [LICENSE](https://github.com/korir248/postgres-async-graphql/blob/main/LICENSE)
