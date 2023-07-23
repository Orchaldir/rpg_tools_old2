function updateAppearancePreview() {
  console.log("Update appearance preview");
  document.getElementById("appearanceForm").submit();
}

function reloadAppearancePreview() {
  console.log("Reload appearance preview");
  var img = document.getElementById("preview");
  var d = new Date;
  img.src="/appearance/preview/" + d.getTime() + "/front.svg";
}