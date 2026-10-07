export function SettingsView(): HTMLElement {
    const el = document.createElement("div");
    el.className = "view";
    el.innerHTML = `
    <div class="view__header">
      <h1 class="view__title">Settings</h1>
      <p class="view__subtitle">Manage your settings</p>
    </div>
  `;
    return el;
}
