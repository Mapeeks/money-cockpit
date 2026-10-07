export function ImportView(): HTMLElement {
    const el = document.createElement("div");
    el.className = "view";
    el.innerHTML = `
    <div class="view__header">
      <h1 class="view__title">Import</h1>
      <p class="view__subtitle">Import your file from your computer</p>
    </div>
  `;
    return el;
}
