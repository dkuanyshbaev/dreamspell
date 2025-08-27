# Dreamspell Calendar

Dreamspell web application and bot.

## Setup

1. Clone the repository
2. Copy environment file:
   ```bash
   cp .env.example .env
   ```
3. Edit `.env` with your values:
   ```bash
   SECRET=your-secret-value
   DB_LOCATION=/path/to/dreambase.db
   ```

## Database Schema

### Users Table
```sql
CREATE TABLE users (
    id integer PRIMARY KEY,
    birthday text Not null
);
```

### Seals Table
```sql
CREATE TABLE seals (
    id integer PRIMARY KEY AUTOINCREMENT,
    name text Not null,
    image text Not null,
    archetype text Not null,
    archetype_description text Not null,
    portrait_description text Not null,
    type_description text Not null
);
```

## Development Commands

### Run Development
```bash
cargo run --bin dreamspell
cargo run --bin dreambot
cargo run --bin dreamadmin
```

### Production Build
```bash
cargo build --release
cargo build --bin dreamspell --release
cargo build --bin dreambot --release
cargo build --bin dreamadmin --release
```

## License

MIT
