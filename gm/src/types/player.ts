export type Player = {
    name: string
    secret: string
    stats: { [key: string]: number | boolean | string | string[] }[]
}
