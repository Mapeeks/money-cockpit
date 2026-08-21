import "./styles/global.scss";

import { loadLayout } from "./lib/loader";
import { initTheme } from "./lib/theme";
import { Sidebar } from "./components/layouts/Sidebar";
import { StatusBar } from "./components/layouts/StatusBar";
import { router } from "./lib/router";

loadLayout();
initTheme();
Sidebar();
StatusBar();
router();
