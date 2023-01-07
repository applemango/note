import axios from "axios"
import { useRouter } from "next/router"
import { useEffect, useRef, useState } from "react"
import useClickAway from "../components/hook/useClickAway"
import Note from "../components/note"
import Table from "../components/table"
import { get, getUrl, post } from "../lib/main"
import { getToken, isLogin } from "../lib/token"
import { NoteResponse } from "../lib/types/type"
import styles from "./scss/index.module.scss"
import stylesN from "../components/scss/note.module.scss"
import stylesT from "../components/scss/table.module.scss"
import Board from "../components/board"

export default function Home() {
  const router = useRouter()
  const [notes, setNotes] = useState<Array<NoteResponse>>([])
  const [selected, setSelected] = useState()
  const [type, setType] = useState("Table")
  useEffect(() => {
    if(!isLogin())
      router.replace("/login")
    const fn = async () => {
      try {
        const res = await get(
          getUrl("/note")
        )
        setNotes(res.data)
      } catch (e) {/* not found */}
    }
    fn()
  },[])
  return <div className={styles.main}>
    <div className={stylesT._}>
      <Menu setType={setType} type={type} />
      {type == "Table" && <Table notes={notes} setSelected={setSelected} setNotes={setNotes} />}
      {type == "Board" && <Board notes={notes} setSelected={setSelected} setNotes={setNotes} />}
    </div>
    <Open reload={async() => {
      try {
        const res = await get(
          getUrl("/note")
        )
        setNotes(res.data)
      } catch (e) {/* not found */}
    }} note={selected} setSelected={setSelected} />
  </div>
}

const Menu = ({
  setType,
  type
}:{
  setType: Function
  type: string
}) => {
  const Button = ({
    text
  }:{
    text: string
  }) => {
    return <div onClick={() => {
      setType(text)
    }} className={styles.button}>
      <div className={styles._}>
        <p>{text}</p>
      </div>
    </div>
  }
  return <div className={styles.menu}>
    <Button text="Board" />
    <Button text="Table" />
  </div>
}

const Open = ({
  note,
  setSelected,
  reload
}:{
  note: any,
  setSelected: any,
  reload: Function
}) => {
  const ref = useRef(null)
  const[t,st]=useState("")
  const[b,sb]=useState("")
  useEffect(()=> {
      if(!note)
        return
      st(note.title)
      sb(note.body)
  },[note])
  useClickAway(ref, ()=>{
    const r = async  () => {
      const r_ = await post(
        getUrl(`/note/${note.id}`),
        {
            title:t,
            description:note.description,
            body:b
        }
      )
    }
    r()
    reload()
    setSelected(null)
  })
  if (!note)
    return <div />
  return <div className={styles.note} ref={ref}>
    <Note t={t} b={b} st={st} sb={sb} note={note} />
  </div>
}