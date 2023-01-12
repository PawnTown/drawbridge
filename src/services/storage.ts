import { invoke } from "@tauri-apps/api/tauri";

const Store = {
    async get(key: string): Promise<any> {
        return JSON.parse(await invoke("load_data", {
            key,
        }));
    },
    async set(key: string, val: any): Promise<boolean> {
        return await invoke("save_data", {
            key,
            val: JSON.stringify(val),
        });
    }
}

export const GetStore = () => {
    return Store;
};