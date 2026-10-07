import { Search, Bell, Globe, createIcons } from "lucide";
import "./Header.scss";

export function StatusBar(): void {}

export function Header(): void {
    createIcons({
        icons: {
            Search,
            Bell,
            Globe,
        },
    });
}
