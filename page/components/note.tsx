import { useEffect, useRef, useState } from "react";
import { deletes, get, getUrl, post } from "../lib/main";
import { NoteResponse, NoteResponseTag, Status, Tag } from "../lib/types/type";
import useClickAway from "./hook/useClickAway";
import styles from "./scss/note.module.scss"
import stylesTable from "./scss/table.module.scss"

const Note = ({
    note,t,b,st,sb
}:{
    note: any,
    t: any,
    b: any,
    st: any,
    sb: any
}) => {
    return <div className={styles.note}>
        {/*<button onClick={async () => {
            const r = await post(
                getUrl(`/note/${note.id}`),
                {
                    title:t,
                    description:note.description,
                    body:b
                }
            )
        }}>{`save: ${note.id}`}</button>*/}
        <input className={styles.title}value={t}onChange={(e:any)=>st(e.target.value)} type="text" />
        <div>
            <Infos note={note} />
        </div>
        <textarea className={styles.body}value={b}onChange={(e:any)=>sb(e.target.value)} />
    </div>
}

const Infos = ({
    note
}:{
    note: NoteResponse
}) => {
    const [status, setStatus] = useState<Array<Status>>([])
    const [tags, setTags] = useState<Array<Tag>>([])
    const [now, setNow] = useState(note.status_id)
    useEffect(()=> {
        setNow(note.status_id)
        const rs = async () => {
            const res = await get(getUrl("/note/status"))
            const re  = await get(getUrl("/note/tag"))
            if(!res.data || !re.data)
                return
            setStatus([...res.data])
            setTags(re.data)
        }
        rs()
    },[note])
    return <div>
        <div style={{display: "flex"}}>
            <p className={styles.button} style={{margin: 0}}>Status</p>
            <StatusSelector status={status} note={note} />
        </div>
        <div style={{display: "flex"}}>
            <p className={styles.button} style={{margin: 0}}>Tags</p>
            <TagSelector setTags={setTags} tags={tags} note={note} />
        </div>
        {/*{now==-1&&<div>
            <div className={stylesTable.status}>
                <p>None</p>
            </div>
        </div>}
        {status.map((status: Status, i:number) => (
            <div onClick={async () => {
                const res = await post(getUrl(`/note/${note.id}/status/${status.id}`))
            }} className={stylesTable.status}>
                <p>{status.name}</p>
            </div>
        ))}*/}
        {/*tags.map((tag: Tag, i:number) => (
            <div onClick={async ()=> {
                if(note.tags.filter(t => t.id == tag.id).length) {
                    const res = await deletes(getUrl(`/note/${note.id}/tag/${tag.id}`))
                    return
                }
                const res = await post(getUrl(`/note/${note.id}/tag/${tag.id}`))
            }} style={{backgroundColor: tag.color}} className={stylesTable.tag}>
                <p>{tag.name}</p>
            </div>
        ))*/}
    </div>
}

const Status = ({
    status,
    note,
    AllowChange = true
}:{
    status: Status
    note: NoteResponse
    AllowChange?: boolean
}) => {
    return <div onClick={async () => {
        if(status.id < 1 || !AllowChange)
            return
        const res = await post(getUrl(`/note/${note.id}/status/${status.id}`))
    }} className={stylesTable.status}>
        <p>{status.name}</p>
    </div>
}

const StatusSelector = ({
    status,
    note,
}:{
    status: Array<Status>,
    note: NoteResponse,
}) => {
    const [now, setNow] = useState<Status>({id: -1, name: "Name", user_id: -1})
    const [open, setOpen] = useState(false)
    const [openCreateStatus, setOpenCreateStatus] = useState(false)
    const ref = useRef(null)
    useEffect(()=> {
        if(note.status_id)
            setNow({id: note.status_id, name: note.status_name, user_id: note.user_id})
    },[note])
    useClickAway(ref, ()=> {
        setOpen(false)
    })
    return <>
        <div ref={ref} className={`${styles.StatusSelector} ${open && styles.open}`}>
            <div className={styles.button} onClick={()=> setOpen(true)}>
                <div>
                    <Status AllowChange={false} note={note} status={now}  />
                </div>
            </div>
            { open &&
                <div className={styles.status}>
                    {status.map((s: Status) => (
                        <Status note={note} status={s}/>
                    ))}
                    <div style={{margin: "4px 0"}} className={styles.button} onClick={()=> {
                        setOpenCreateStatus(true)
                        setOpen(false)
                    }} >
                        <p>create</p>
                    </div>
                </div>
            }
        </div>
        <CreateStatus open={openCreateStatus} setOpen={setOpenCreateStatus} />
    </>
}

const Tag = ({
    setTags,
    now,
    setNow,
    tag,
    note,
    AllowChange = true
}:{
    setTags: Function
    setNow: Function
    tag: NoteResponseTag
    note: NoteResponse
    AllowChange?: boolean,
    now: Array<NoteResponseTag>
}) => {
    return <div onClick={async () => {
        if(now.filter(t => t.id == tag.id).length) {
            const res = await deletes(getUrl(`/note/${note.id}/tag/${tag.id}`))
            setNow((tags: Array<NoteResponseTag>):Array<NoteResponseTag> => {
                const t = tags.concat().filter((t:NoteResponseTag) => t.id != tag.id)
                return t
            })
            return
        }
        const res = await post(getUrl(`/note/${note.id}/tag/${tag.id}`))
        setNow((tags: Array<NoteResponseTag>) => [...tags, tag])
    }} style={{backgroundColor: tag.color}} className={stylesTable.tag}>
        <p>{tag.name}</p>
    </div>
}

const TagSelector = ({
    setTags,
    tags,
    note
}:{
    setTags: Function
    tags: Array<Tag>
    note: NoteResponse
}) => {
    const [now, setNow] = useState<Array<NoteResponseTag>>([])
    const [open, setOpen] = useState(false)
    const [openCreateTag, setOpenCreateTag] = useState(false)
    const ref = useRef(null)
    useEffect(()=> {
        if(note.tags.length)
            setNow([...note.tags])
    },[note])
    useClickAway(ref, ()=> {
        setOpen(false)
    })
    return <>
        <div ref={ref} className={`${styles.StatusSelector} ${open && styles.open}`}>
            <div className={styles.button} onClick={()=> setOpen(true)}>
                <div style={{display: "flex"}}>
                    {now.map((s: NoteResponseTag) => (
                            <Tag now={now} setNow={setNow} setTags={setTags} note={note} tag={s}/>
                        ))}
                </div>
            </div>
            { open &&
                <div className={`${styles.status} ${styles._tags}`}>
                    <div className={styles.tags}>
                        {tags.map((s: Tag) => (
                            <Tag now={now} setNow={setNow} setTags={setTags} note={note} tag={s}/>
                        ))}
                    </div>
                    <div style={{margin: "4px 0"}} className={styles.button} onClick={()=> {
                        setOpenCreateTag(true)
                        setOpen(false)
                    }} >
                        <p>create</p>
                    </div>
                </div>
            }
        </div>
        <CreateTag open={openCreateTag} setOpen={setOpenCreateTag} />
    </>
}

const CreateTag = ({
    open,
    setOpen
}:{
    open: boolean
    setOpen: Function
}) => {
    const ref = useRef(null)
    const [name, setName] = useState("")
    const [color, setColor] = useState("#000000")
    useClickAway(ref, ()=> {
        setOpen(false)
    })
    return <div ref={ref} className={`${styles.createTagMenu} ${open && styles.open}`}>
        <input type="text" placeholder="name" value={name} onChange={(e:any) => setName(e.target.value)} />
        <input type="color" value={color} onChange={(e) => {setColor(e.target.value)}} />
        <button onClick={async ()=> {
            const res = await post(getUrl("/note/tag/create"), {
                name: name,
                color: color
            })
        }}>Create</button>
    </div>
}

const CreateStatus = ({
    open,
    setOpen
}:{
    open: boolean
    setOpen: Function
}) => {
    const [name, setName] = useState("")
    const ref = useRef(null)
    useClickAway(ref, ()=> {
        setOpen(false)
    })
    return <div ref={ref} className={`${styles.createTagMenu} ${open && styles.open}`}>
        <input type="text" placeholder="name" value={name} onChange={(e:any) => setName(e.target.value)} />
        <button onClick={async ()=> {
            const res = await post(getUrl("/note/status/create"), {
                name: name
            })
        }}>Create</button>
    </div>
}
export default Note;