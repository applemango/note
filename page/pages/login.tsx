import { useRouter } from "next/router";
import { useEffect, useState } from "react";
import { get, getUrl, post } from "../lib/main";
import { setToken } from "../lib/token";

export default function Login() {
    const [u, su] = useState("")
    const [p, sp] = useState("")
    const router = useRouter()
    return <div>
        <form onSubmit={async (e: any)=> {
            e.preventDefault();
            const r:any=await post(
                getUrl("/token"),
                {
                    username: u,
                    password: p
                },false
            )
            if(! (r.data.refresh_token && r.data.token))
                return
            setToken(r.data.refresh_token, true)
            setToken(r.data.token)
            router.replace("/")
        }}>
            <input value={u}onChange={(e:any)=>su(e.target.value)}type="text"/>
            <input value={p}onChange={(e:any)=>sp(e.target.value)}type="text"/>
            <button>login</button>
        </form>
    </div>
}