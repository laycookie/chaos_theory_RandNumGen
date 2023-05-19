import { simulate } from "chaos_theory";
import * as PIXI from "pixi.js";

const res_x = 500;
const res_y = 500;
let app = new PIXI.Application({
  width: res_x,
  height: res_y,
  antialias: true,
});
document.body.appendChild(app.view as unknown as Node);

const SCALER_CONST = 30;
// create circle sprite

const out = JSON.parse(simulate(2, 2, 4, 1, 0, 0, 180));
for (let i of out.circles) {
  let circle = new PIXI.Graphics();
  circle.beginFill(0x9966ff);
  circle.drawCircle(
    i.x * SCALER_CONST,
    -i.y * SCALER_CONST,
    i.radius * SCALER_CONST
  );
  circle.endFill();
  circle.x = res_x / 2;
  circle.y = res_y / 2;
  app.stage.addChild(circle);
}
for (let i of out.laser_beams) {
  if (i.length < 0) {
    console.log("negative length");
    break;
  }

  let line = new PIXI.Graphics();
  line.lineStyle(1, 0xffffff);
  line.moveTo(i.x * SCALER_CONST, -i.y * SCALER_CONST);
  line.lineTo(i.end_x * SCALER_CONST, -i.end_y * SCALER_CONST);
  line.x = res_x / 2;
  line.y = res_y / 2;
  app.stage.addChild(line);
}
console.log(out);
