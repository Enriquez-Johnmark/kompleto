// Kompleto's only script, served from our own origin so the
// Content-Security-Policy can refuse every inline script.
(function () {
  "use strict";

  // Drafts page: "Tick all" ticks every Draft's box.
  document.addEventListener("change", function (event) {
    var box = event.target;
    if (box.matches && box.matches("[data-tick-all]")) {
      document.querySelectorAll("input[name=ids]").forEach(function (each) {
        each.checked = box.checked;
      });
    }
  });

  // Viber page: "Copy" copies the message beside it.
  document.addEventListener("click", function (event) {
    var button = event.target.closest && event.target.closest("[data-copy]");
    if (button) {
      var text = document.getElementById(button.dataset.copy);
      if (text) navigator.clipboard.writeText(text.value);
    }
  });

  if (!document.querySelector("form.upload")) return;
  // Sends each chosen file as its own request, so several photos can go at
  // once without one large request. Without this script, one file per submit.
  var form = document.querySelector("form.upload");
  var input = document.getElementById("file");
  var error = document.getElementById("error");
  var progress = document.getElementById("progress");
  input.multiple = true;
  form.addEventListener("submit", async function (event) {
    event.preventDefault();
    var files = Array.from(input.files);
    var sent = 0;
    error.hidden = true;
    var sending = progress.dataset.sending || progress.textContent;
    progress.dataset.sending = sending;
    progress.hidden = false;
    for (var i = 0; i < files.length; i++) {
      progress.textContent = sending + " " + (i + 1) + "/" + files.length;
      var data = new FormData(form);
      data.set("file", files[i]);
      var response = await fetch(form.action, { method: "POST", body: data });
      if (!response.ok) {
        var page = new DOMParser().parseFromString(await response.text(), "text/html");
        var message = page.getElementById("error");
        error.textContent = files[i].name + ": " + (message ? message.textContent : response.status);
        error.hidden = false;
        progress.hidden = true;
        return;
      }
      sent++;
    }
    window.location.href = window.location.pathname + "?sent=" + sent;
  });
})();
