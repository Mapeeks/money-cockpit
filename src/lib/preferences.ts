type Preferences = {
    theme: "light" | "dark" | null;
    sidebarCompact: boolean;
};

export type Theme = Preferences["theme"];

const DEFAULTS: Preferences = {
    theme: null,
    sidebarCompact: false,
};

const KEY = "mc:preferences";

function load(): Preferences {
    try {
        const raw = localStorage.getItem(KEY);
        return raw ? { ...DEFAULTS, ...JSON.parse(raw) } : { ...DEFAULTS };
    } catch {
        return { ...DEFAULTS };
    }
}

function save(prefs: Preferences): void {
    localStorage.setItem(KEY, JSON.stringify(prefs));
}

let current: Preferences = load();

export function getPreference<K extends keyof Preferences>(key: K): Preferences[K] {
    return current[key];
}

export function setPreference<K extends keyof Preferences>(key: K, value: Preferences[K]): void {
    current = { ...current, [key]: value };
    save(current);
}
