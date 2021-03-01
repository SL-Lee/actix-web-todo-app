window.addEventListener("load", (e) => {
  document
    .querySelectorAll(".alert")
    .forEach((toast) => halfmoon.toastAlert(toast.getAttribute("id"), 5000));
});
