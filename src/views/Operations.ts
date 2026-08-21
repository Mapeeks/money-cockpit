export function OperationsView(): HTMLElement {
  const el = document.createElement("div");
  el.className = "view";
  el.innerHTML = `
    <div class="view__header">
      <h1 class="view__title">Operations</h1>
      <p class="view__subtitle">Track your transactions</p>
    </div>
  `;
  return el;
}
