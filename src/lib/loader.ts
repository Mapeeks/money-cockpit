import sidebarTemplate from "../components/layouts/Sidebar.html?raw";
import statusBarTemplate from "../components/layouts/StatusBar.html?raw";

function inject(containerId: string, template: string): void {
    const container = document.getElementById(containerId);
    if (container) container.innerHTML = template;
}

export function loadLayout(): void {
    inject("sidebar", sidebarTemplate);
    inject("status-bar", statusBarTemplate);
}
