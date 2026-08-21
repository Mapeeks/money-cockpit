import { getPreference, setPreference, type Theme } from "./preferences";

function applyTheme(theme: Theme): void {
    let newTheme = theme;
    if (theme == null) {
        newTheme = getSystemTheme();
    }
    document.documentElement.setAttribute("data-theme", newTheme!);
}

function getSystemTheme(): Theme {
    return window.matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light";
}

export function initTheme(): void {
    const theme = getPreference("theme");
    applyTheme(theme);

    window.matchMedia("(prefers-color-scheme: dark)").addEventListener("change", (e) => {
        const theme = getPreference("theme");
        applyTheme(theme);
    });
}

export function toggleTheme(): void {
    const current = document.documentElement.getAttribute("data-theme") as Theme;
    const next: Theme = current === "dark" ? "light" : "dark";
    setPreference("theme", next);
    applyTheme(next);
}
