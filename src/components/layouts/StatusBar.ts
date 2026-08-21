import "./StatusBar.scss";
import { getVersion } from "@tauri-apps/api/app";
import { PanelLeftClose, PanelLeftOpen, ArrowDownToLine, createIcons } from "lucide";
import { getPreference, setPreference } from "../../lib/preferences";

export function StatusBar(): void {
    getVersion().then((version) => {
        const container = document.getElementById("status-version");
        if (container) {
            container.textContent = `v${version}`;
        }
    });

    createIcons({ icons: { PanelLeftClose, PanelLeftOpen, ArrowDownToLine } });

    const isCompact = getPreference("sidebarCompact");
    if (isCompact) {
        document.querySelector(".sidebar")?.classList.toggle("sidebar--compact", isCompact);
    }
    updateSidebarIcon();

    document.getElementById("toggle-sidebar")?.addEventListener("click", toggleSidebar);
}

function toggleSidebar(): void {
    const isCompact = !getPreference("sidebarCompact");
    document.querySelector(".sidebar")?.classList.toggle("sidebar--compact", isCompact);
    setPreference("sidebarCompact", isCompact);
    updateSidebarIcon();
}

function updateSidebarIcon(): void {
    const isCompact = getPreference("sidebarCompact");
    const iconName = isCompact ? "panel-left-open" : "panel-left-close";
    let button = document.getElementById("toggle-sidebar");
    let icon = document.createElement("i");
    icon.setAttribute("data-lucide", iconName);
    button?.querySelector("svg")?.replaceWith(icon);
    createIcons({ icons: { PanelLeftClose, PanelLeftOpen } });
}
