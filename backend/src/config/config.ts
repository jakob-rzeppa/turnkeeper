import dotenv from "dotenv";

dotenv.config({ path: [".env.local", ".env"] });

interface Config {
    dbPath: null | string;
    port: number;
}

const config: Config = {
    dbPath: process.env.DB_PATH ?? null,
    port: Number(process.env.PORT ?? 3000),
};

export default config;
