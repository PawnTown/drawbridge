export interface Remote {
    // universal
    id?: string;
    driver: string;
    label: string;
    url: string;

    // ptcec exclusive
    engine?: string;
    mode?: string;
    token?: string;

    // ssh exclusive
    runCommand?: string;
    privateKeyFile?: string;
}