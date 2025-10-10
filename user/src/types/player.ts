export type PlayerStat = {
    name: string
    value: boolean | number | string | string[]
}

export type Player = {
    id: string
    name: string
    stats: PlayerStat[]
}
