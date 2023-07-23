function updateAppearancePreview() {
  console.log("Update appearance preview");
  document.getElementById("appearanceForm").submit();
  const sleep = time => new Promise(res => setTimeout(res, time, "done sleeping"));
  sleep(100).then(msg => reloadAppearancePreview());
}

function reloadAppearancePreview() {
  console.log("Reload appearance preview");
  var img = document.getElementById("preview");
  var d = new Date;
  img.src="/appearance/preview/" + d.getTime() + "/front.svg";
}