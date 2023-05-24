import {Settings} from './main.d.ts'
import { render } from './renderer'

let settings: Settings = {
  zoom: 30,
  ini_y: 0,
  ini_angle: 20,

  circleAmountX: 3,
  circleAmountY: 3,
  circleRadius: 1,
  circleDistance: 3,
}

const formSubmition = document.getElementById('form-submition') as HTMLFormElement;
formSubmition.addEventListener('submit', (e) => {
  e.preventDefault();

  const formData = new FormData(formSubmition);
  let angle = formData.get("angle");
  if (angle == "") {
    angle = "0";
  }
  settings.ini_angle = Number(angle);
  render(settings);
});

render(settings)