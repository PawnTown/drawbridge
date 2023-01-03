import { Store } from 'tauri-plugin-store-api';

let storeSingleton: Store | null = null;

export const GetStore = () => {
    if (!storeSingleton) {
        storeSingleton = new Store('.settings.dat');
    }
    return storeSingleton;
};