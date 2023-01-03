import { useEffect } from "react"
import { deletes, getUrl, post } from "../lib/main"
import { NoteResponse, NoteResponseTag } from "../lib/types/type"
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
            <svg  onClick={async (e:any) => {
                e.preventDefault();
                const res = await deletes(getUrl(`/note/${note.id}`))
                setNotes((notes: Array<NoteResponse>) => notes.concat().filter((n: NoteResponse) => n.id != note.id))
            }} xmlns="http://www.w3.org/2000/svg" className="icon icon-tabler icon-tabler-trash" width="21" height="21" viewBox="0 0 24 24" stroke-width="1.5" stroke="#ff2825" fill="none" stroke-linecap="round" stroke-linejoin="round">
              <path stroke="none" d="M0 0h24v24H0z" fill="none"/>
              <line x1="4" y1="7" x2="20" y2="7" />
              <line x1="10" y1="11" x2="10" y2="17" />
              <line x1="14" y1="11" x2="14" y2="17" />
              <path d="M5 7l1 12a2 2 0 0 0 2 2h8a2 2 0 0 0 2 -2l1 -12" />
              <path d="M9 7v-3a1 1 0 0 1 1 -1h4a1 1 0 0 1 1 1v3" />
            </svg>
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