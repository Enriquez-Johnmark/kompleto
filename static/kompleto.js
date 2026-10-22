// The Client upload page's script, served from our own origin so the
// Content-Security-Policy can refuse every inline script.
(function () {
  "use strict";

  if (!document.querySelector("form.upload")) return;
  // Asks for a Payor only for a 2307, and for a note only for "something
  // else". Without this script both are shown, and the server checks either way.
  var choice = document.getElementById("document");
  function showWhen(id, relevant) {
    var field = document.getElementById(id);
    var label = document.querySelector('label[for="' + id + '"]');
    return function () {
      if (!field) return;
      var show = relevant(choice.value);
      field.hidden = !show;
      if (label) label.hidden = !show;
      if (!show) field.value = "";
    };
  }
  var updates = [
    showWhen("payor", function (value) { return value.indexOf("2307") !== -1; }),
    showWhen("note", function (value) { return value === "other"; }),
  ];
  function showRelevant() {
    updates.forEach(function (update) { update(); });
  }
  choice.addEventListener("change", showRelevant);
  showRelevant();
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
