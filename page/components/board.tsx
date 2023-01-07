import { useEffect, useState } from "react"
import { get, getUrl } from "../lib/main"
import { NoteResponse, NoteResponseTag, Status } from "../lib/types/type"
import styles from "./scss/board.module.scss"
import stylesT from "./scss/table.module.scss"

const Board = ({
    notes,
    setNotes,
    setSelected
}:{
    notes: Array<NoteResponse>
    setNotes: Function
    setSelected: Function
}) => {
    const [status, setStatus] = useState<Array<Status>>([])
    useEffect(() => {(async () => {
        setStatus([{id: -1, name: "none", user_id: -1},...await (await get(getUrl("/note/status"))).data])
    })()},[])
    console.log(status)
    return <div className={styles._}>
        {status.map((s: Status, key: number) => {
            const ns = notes.concat().filter((note: NoteResponse) => note.status_id == s.id)
            return <div style={{width: `calc(100% / ${status.length} - ${key ? 12 : 0}px)`}} className={styles.cards}>
                <CardsTop notes={ns} status={s} />
                <Cards setSelected={setSelected} notes={ns} setNotes={setNotes} />
            </div>
        })}
    </div>
}
const CardsTop = ({
    notes,
    status
}:{
    notes: Array<NoteResponse>
    status: Status
}) => {
    return <div className={styles.card}>
        <p className={styles.title}>{status.name}</p>
        <p>{`${notes.length} notes`}</p>
    </div>
}
const Cards = ({
    notes,
    setNotes,
    setSelected
}:{
    notes: Array<NoteResponse>
    setNotes: Function
    setSelected: Function
}) => {
    return <>
    {notes.map((note: NoteResponse) => <Card onSelected={(note: any) => setSelected(note)}  setNotes={setNotes} note={note} />)}
    </>
}
const Card = ({
    note,
    onSelected,
    setNotes
}:{
    note: NoteResponse
    onSelected: Function
    setNotes: Function
}) => {
    return <div className={styles.card} onClick={async ()=> await onSelected(note)}>
        <p className={styles.title}>{note.title}</p>
        <div style={{display: "flex"}}>
            {note.tags && note.tags.map((tag: NoteResponseTag, i:number) => (
                <div className={stylesT.tag} style={{backgroundColor: tag.color}}>
                    <p>{tag.name}</p>
                </div>
            ))}
        </div>
    </div>
}

export default Board