export interface Remote {
    // universal
    driver: string;
    label: string;
    url: string;

    // ptcec exclusive
    engine: string;
    mode: string;
    token: string;

    // ssh exclusive
    runCommand: string;
}