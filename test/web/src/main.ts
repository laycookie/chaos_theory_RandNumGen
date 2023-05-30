import type { Settings, MassSetCircles } from "./main.d";
import { render } from "./renderer";
import { manny_circle_set } from "chaos_theory";

let settings: Settings = {
  zoom: 20,
  ini_x: 0,
  ini_y: 0,
  ini_angle: 0,
  reflectionsNum: 20n,
};

let massSetCircles: MassSetCircles = {
  circleAmountX: 0,
  circleAmountY: 0,
  circleRadius: 1,
  circleSpacing: 3,
};

const formSubmition = document.getElementById(
  "form-submition"
) as HTMLFormElement;
formSubmition.addEventListener("submit", (e) => {
  e.preventDefault();

  const formData = new FormData(formSubmition);
  let ini_x = formData.get("ini_x");
  let ini_y = formData.get("ini_y");
  let circleAmountX = formData.get("circleAmountX");
  let circleAmountY = formData.get("circleAmountY");
  let angle = formData.get("angle");
  let reflectionsAmount = formData.get("reflectionsAmount");
  let zoom = formData.get("zoom");

  settings.ini_angle = Number(angle);
  settings.ini_x = Number(ini_x);
  settings.ini_y = Number(ini_y);
  massSetCircles.circleAmountX = Number(circleAmountX);
  massSetCircles.circleAmountY = Number(circleAmountY);
  settings.reflectionsNum = BigInt(Number(reflectionsAmount));
  settings.zoom = Number(zoom);

  manny_circle_set(
    massSetCircles.circleAmountX,
    massSetCircles.circleAmountY,
    massSetCircles.circleSpacing,
    massSetCircles.circleRadius
  );
  render(settings);
});
