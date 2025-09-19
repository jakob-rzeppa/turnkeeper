export type PlayerStat = {
    name: string
    value: boolean | number | string | string[]
}

export type Player = {
    id: string
    name: string
    secret: string
    stats: PlayerStat[]
}
