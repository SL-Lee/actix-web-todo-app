function toggleDarkMode() {
  let [oldIconClass, newIconClass] = document
    .querySelector("body")
    .classList.contains("dark-mode")
    ? ["fa-sun", "fa-moon"]
    : ["fa-moon", "fa-sun"];

  halfmoon.toggleDarkMode();

  document
    .getElementById("toggle-dark-mode-button-icon")
    .classList.replace(oldIconClass, newIconClass);
}
