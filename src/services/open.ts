import { open } from '@tauri-apps/api/shell';
export const OpenUrl = (url: String) : Promise<void> => {
    return open('https://pawntown.github.io/drawbridge_web/');
};