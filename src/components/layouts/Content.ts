import { onRouteChange, type Route } from "../../lib/router";
import "./Content.scss";

type ViewFactory = () => HTMLElement;

export function Content(views: Record<Route, ViewFactory>): HTMLElement {
    const el = document.createElement("main");
    el.className = "content";

    function render(route: Route): void {
        el.innerHTML = "";
        el.appendChild(views[route]());
    }

    onRouteChange(render);
    render("dashboard");

    return el;
}
