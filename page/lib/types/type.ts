export type NoteResponseTag = {
    id: number
    name: string
    color: string
}
export type NoteResponse = {
    id: number,
    user_id: number,
    status_id: number,
    status_name: string,
    title: string,
    description: string,
    body: string,
    tags: Array<NoteResponseTag>
}

export type Status = {
    id: number,
    name: string,
    user_id: number,
}
export type Tag = {
    id: number,
    user_id: number,
    name: string,
    color: string
}