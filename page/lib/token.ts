import axios from 'axios'
import { deletes, getUrl, post } from './main';

export function parseJwt (token: string | undefined | null):any {
    if (!token)
        return
    const base64Url = token.split('.')[1];
    const base64 = base64Url.replace(/-/g, '+').replace(/_/g, '/');
    const jsonPayload = decodeURIComponent(window.atob(base64).split('').map(function(c) {
        return '%' + ('00' + c.charCodeAt(0).toString(16)).slice(-2);
    }).join(''));
    return JSON.parse(jsonPayload);
};
export function setToken(token:string, refresh:boolean = false):void {
    if(refresh) {
        localStorage.setItem("refreshToken", token)
        return
    }
    localStorage.setItem("token", token)
    return
}
export function getToken(refresh:boolean = false):string | null {
    try {
        let Token;
        if (refresh) {Token = localStorage.getItem("refreshToken");
        } else {Token = localStorage.getItem("token");}
        if (Token && Date.now() >= parseJwt(Token).exp *1000) {
            return null;
        }
        return Token && Token
    } catch (error) {
        return null;
    }
}
export function isLogin(refresh:boolean = true):boolean  {
    if(getToken()) {
        return true
    }
    if(refresh && getToken(true)) {
        return true
    }
    return false
}

export async function isLoginAndLogin() {
    if(getToken()) {
        return true
    }
    if(getToken(true)) {
        try {
            const res = await refresh()
            return true
        } catch (error: any) {
            return false
        }
    }
    return false
}

export async function logout() {
    const d = () => {
        localStorage.removeItem("token");
        localStorage.removeItem("refreshToken")}
    try {
        const res = await deletes(getUrl("/token/logout"))
        d();return {"msg":"success","code": res.status,"text": res.statusText,"data":res}
    } catch (error: any) {
        d();return {"msg":"error(success?)", "code": error.response.status, "text": error.response.data.msg, "data": error}
    }
}

export async function refresh() {
    if(!getToken(true)) {
        return false
    }
    try {
        const res = await post(getUrl("/token/refresh"))
        localStorage.setItem("token", res.data.token)
        return {"msg":"success","code": res.status,"text": res.statusText,"data":res}
    } catch (error: any) {
        const r = await logout()
        return {"msg":"error", "code": error.response.status, "text": error.response.data.msg, "data": error}
    }
}