import type { Settings } from "./main.d.ts";
import { render } from "./renderer";

let settings: Settings = {
  zoom: 30,

  ini_x: 0,
  ini_y: 0,
  ini_angle: 30,

  circleAmountX: 3,
  circleAmountY: 3,
  circleRadius: 1,
  circleDistance: 3,
};

const formSubmition = document.getElementById(
  "form-submition"
) as HTMLFormElement;
formSubmition.addEventListener("submit", (e) => {
  e.preventDefault();

  const formData = new FormData(formSubmition);
  let ini_x = formData.get("ini_x");
  let ini_y = formData.get("ini_y");
  let angle = formData.get("angle");
  if (angle == "") {
    angle = "0";
  }
  settings.ini_angle = Number(angle);
  settings.ini_x = Number(ini_x);
  settings.ini_y = Number(ini_y);
  render(settings);
});

render(settings);
