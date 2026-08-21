export type Route = "dashboard" | "accounts" | "operations";

type RouteChangeCallback = (route: Route) => void;

const listeners: RouteChangeCallback[] = [];
const controllers: Partial<Record<Route, () => void>> = {};
let current: Route = "dashboard";

const templates = import.meta.glob("../views/*.html", {
    eager: true,
    query: "?raw",
    import: "default",
}) as Record<string, string>;

export function registerController(route: Route, controller: () => void): void {
    controllers[route] = controller;
}

function renderView(route: Route): void {
    const content = document.getElementById("content");
    if (!content) return;
    content.innerHTML =
        templates[`../views/${route.charAt(0).toUpperCase() + route.slice(1)}.html`];
    controllers[route]?.();
}

export function navigate(route: Route): void {
    current = route;
    renderView(route);
    listeners.forEach((cb) => cb(route));
}

export function currentRoute(): Route {
    return current;
}

export function onRouteChange(cb: RouteChangeCallback): void {
    listeners.push(cb);
}

export function router(): void {
    renderView(current);
}
