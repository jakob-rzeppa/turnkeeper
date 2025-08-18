import dotenv from 'dotenv';

dotenv.config({ path: `.env.local` });

interface Config {
  port: number;
}

const config: Config = {
  port: Number(process.env.PORT) || 3000,
};

export default config;