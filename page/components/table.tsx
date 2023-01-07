import { useEffect } from "react"
import { deletes, getUrl, post } from "../lib/main"
import { NoteResponse, NoteResponseTag } from "../lib/types/type"
import { IconDelete } from "./icon"
import styles from "./scss/table.module.scss"

const Table = ({
    notes,
    setNotes,
    setSelected
}:{
    notes: Array<any>
    setNotes: Function
    setSelected: Function
}) => {
    return <div className={styles.table}>
        <TableTop />
        {notes.map((note:any, i:number) => <div key={i} ><TableLine setNotes={setNotes} onSelected={(note: any) => setSelected(note)} note={note} /></div>)}
        <TableNew setNotes={setNotes} setSelected={setSelected} />
    </div>
}

const TableLine = ({
    note,
    onSelected,
    setNotes
}:{
    note: NoteResponse
    onSelected: Function
    setNotes: Function
}) => {
    return <div className={`${styles.row}`}>
        <div className={styles.column} onClick={()=> {
            onSelected(note)
        }}>
            <p>{note.title}</p>
        </div>
        <div className={styles.column}>
            <div className={styles.status}>
                <p>{note.status_name}</p>
            </div>
        </div>
        <div className={styles.column}>
            {note.tags && note.tags.map((tag: NoteResponseTag, i:number) => (
                <div style={{backgroundColor: tag.color}} className={styles.tag}>
                    <p>{tag.name}</p>
                </div>
            ))}
        </div>
        <div className={styles.column}>
            <IconDelete  onClick={async (e:any) => {
                e.preventDefault();
                const res = await deletes(getUrl(`/note/${note.id}`))
                setNotes((notes: Array<NoteResponse>) => notes.concat().filter((n: NoteResponse) => n.id != note.id))
            }} />
        </div>
    </div>
}

const TableTop = () => {
    return <div className={`${styles.top} ${styles.row}`}>
        <div className={styles.column}>
            <p>Name</p>
        </div>
        <div className={styles.column}>
            <p>Status</p>
        </div>
        <div className={styles.column}>
            <p>Tags</p>
        </div>
        <div className={styles.column}></div>
    </div>
}

const TableNew = ({
    setSelected,
    setNotes
}:{
    setSelected: Function
    setNotes: Function
}) => {
    return <div onClick={async ()=>{
        const res = await post(getUrl("/note"))
        if(!res.data)
            return
        setSelected(res.data)
        setNotes((d:any) => [...d, res.data])
    }} className={`${styles.new} ${styles.row}`}>
        <p>New</p>
    </div>
}

export default Table