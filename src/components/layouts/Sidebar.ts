import { navigate, onRouteChange, type Route } from "../../lib/router";
import { toggleTheme } from "../../lib/theme";
import {
    createIcons,
    LayoutDashboard,
    Landmark,
    CircleEuro,
    Sunrise,
    Sunset,
    PanelLeftClose,
    PanelLeftOpen,
    PiggyBank,
} from "lucide";
import "./Sidebar.scss";

const NAV_ITEMS: { route: Route; label: string; icon: string }[] = [
    { route: "dashboard", label: "Dashboard", icon: "layout-dashboard" },
    { route: "accounts", label: "Accounts", icon: "landmark" },
    { route: "operations", label: "Operations", icon: "circle-euro" },
];

function buildNavItems(): void {
    const nav = document.getElementById("sidebar-nav");
    if (!nav) return;

    NAV_ITEMS.forEach((item) => {
        const btn = document.createElement("button");
        btn.className = "sidebar__item";
        btn.dataset.route = item.route;
        btn.innerHTML = `
            <i data-lucide="${item.icon}"></i>
            <span class="sidebar__item-label">${item.label}</span>
        `;
        nav.appendChild(btn);
    });
}

function updateThemeIcon(): void {
    const current = document.documentElement.getAttribute("data-theme");
    const container = document.querySelector("#theme-toggle");
    if (!container) return;
    const iconName = current === "dark" ? "sunrise" : "sunset";
    const i = document.createElement("i");
    i.setAttribute("data-lucide", iconName);
    container.querySelector("svg")?.replaceWith(i);
    createIcons({ icons: { Sunset, Sunrise } });
}

function setActive(route: Route): void {
    document.querySelectorAll(".sidebar__item[data-route]").forEach((btn) => {
        btn.classList.toggle("sidebar__item--active", btn.getAttribute("data-route") === route);
    });
}

export function Sidebar(): void {
    buildNavItems();

    createIcons({
        icons: {
            LayoutDashboard,
            Landmark,
            CircleEuro,
            Sunrise,
            Sunset,
            PanelLeftClose,
            PanelLeftOpen,
            PiggyBank,
        },
    });

    updateThemeIcon();

    document.querySelectorAll(".sidebar__item[data-route]").forEach((btn) => {
        btn.addEventListener("click", () => {
            navigate(btn.getAttribute("data-route") as Route);
        });
    });

    document.getElementById("theme-toggle")?.addEventListener("click", () => {
        toggleTheme();
        updateThemeIcon();
    });

    onRouteChange(setActive);
    setActive("dashboard");
}
