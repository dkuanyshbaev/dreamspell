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
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    name_en TEXT NOT NULL,
    image TEXT NOT NULL,
    archetype TEXT NOT NULL,
    archetype_en TEXT NOT NULL,
    archetype_description TEXT NOT NULL,
    archetype_description_short TEXT NOT NULL,
    archetype_description_en TEXT NOT NULL,
    archetype_description_short_en TEXT NOT NULL,
    portrait_description TEXT NOT NULL,
    portrait_description_short TEXT NOT NULL,
    portrait_description_en TEXT NOT NULL,
    portrait_description_short_en TEXT NOT NULL,
    type_description TEXT NOT NULL,
    type_description_short TEXT NOT NULL,
    type_description_en TEXT NOT NULL,
    type_description_short_en TEXT NOT NULL
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
