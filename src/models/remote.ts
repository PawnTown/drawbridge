import { v4 as uuidv4 } from 'uuid';

export interface Remote {
    // universal
    id?: string;
    driver: string;
    label: string;
    url: string;
    middlewareLua: string;

    // ptcec exclusive
    engine?: string;
    mode?: string;
    token?: string;

    // ssh exclusive
    runCommand?: string;
    privateKeyFile?: string;
}

export function saveRemote(remotes: Remote[], remote: Remote) : Remote[] {
    const filteredRemotes = (remotes ?? []).filter((r: Remote) => !remote.id || r.id !== remote.id);

    let newRemote: Remote;
    if (remote.driver === "ssh") {
        newRemote = {
            id: remote.id ?? uuidv4(),
            driver: "ssh",
            label: remote.label,
            url: remote.url,
            runCommand: remote.runCommand,
            privateKeyFile: remote.privateKeyFile,
            middlewareLua: remote.middlewareLua,
        };
    } else {
        newRemote = {
            id: remote.id ?? uuidv4(),
            driver: "ptcec",
            label: remote.label,
            url: remote.url,
            engine: remote.engine,
            mode: remote.mode,
            token: remote.token,
            middlewareLua: remote.middlewareLua,
        };
    }

    return [...filteredRemotes, newRemote];
};