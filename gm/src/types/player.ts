export type Player = {
    id: string
    name: string
    secret: string
    stats: { [key: string]: number | boolean | string | string[] }[]
}
