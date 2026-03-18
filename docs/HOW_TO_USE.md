# How to use the project

## Prerequisites

- **Backend:** Rust toolchain (install via [rustup](https://rustup.rs/))
- **Frontends (GM & User):** Node.js and npm

## Environment Variables

The backend requires a `.env` file (or environment variables) with:

```
DATABASE_URL=sqlite:var/turnkeeper.db
GM_JWT_SECRET=<your-secret>
USER_JWT_SECRET=<your-secret>
GM_PASSWORD=<your-gm-password>
```

## Development

1. **Backend:** In the `backend/` directory, run `cargo run`. This starts the Axum server (default port 8080) and automatically runs SQLite migrations.
2. **GM Frontend:** In the `gm/` directory, run `npm install` then `npm run dev`.
3. **User Frontend:** In the `user/` directory, run `npm install` then `npm run dev`.

The GM frontend defaults to `http://localhost:8080/gm` as the API base URL (configurable via `VITE_API_BASE_URL`). The user frontend defaults to `http://localhost:8080`.
