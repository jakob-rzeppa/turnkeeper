import dotenv from "dotenv";

dotenv.config({ path: [".env.local", ".env"] });

interface Config {
    port: number;
    dbPath: string | null;
}

const config: Config = {
    port: Number(process.env.PORT) || 3000,
    dbPath: process.env.DB_PATH || null,
};

export default config;
