import init, { simulate } from "chaos_theory";
import * as PIXI from "pixi.js";
import type { Settings } from "@/types/main.d";
import { RefObject } from "react";

// render circles
function calculateNewPoint(
  originalX: number,
  originalY: number,
  angleInDegrees: number,
  distance: number
) {
  // Convert angle from degrees to radians
  let angleInRadians = angleInDegrees * (Math.PI / 180);

  // Calculate the new coordinates
  let newX = distance * Math.cos(angleInRadians) + originalX;
  let newY = distance * Math.sin(angleInRadians) + originalY;

  // Return the new coordinates as an object
  return { x: newX, y: newY };
}

// Renderer
let app: PIXI.Application;
export function initCanvas(canvasRef: RefObject<HTMLCanvasElement>) {
  // attach app to the canvas
  if (canvasRef.current === null) return;
  app = new PIXI.Application({
    view: canvasRef.current,
    width: window.innerWidth,
    height: window.innerHeight,
    antialias: true,
  });
}

export function render(
  { ini_x, ini_y, ini_angle, reflectionsNum, zoom }: Settings,
  setAngle: (angle: number) => void
) {
  // checks if runs on client
  if (typeof window === "undefined") return;

  const SCALER_CONST = zoom;
  let res_x = window.innerWidth;
  let res_y = window.innerHeight;

  // check if canvas is initialized
  if (!app) {
    console.warn("app is not initialized");
    return;
  }
  // clear canvas before the next render
  app.stage.removeChildren();

  init().then(() => {
    const out = JSON.parse(simulate(ini_x, ini_y, ini_angle, reflectionsNum));

    // create circle sprite
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
    // create laser beams
    for (let i of out.laser_beams) {
      // check if the laser beam has not bounced from one circle to another circle
      if (!i.bounces) {
        let line = new PIXI.Graphics();
        line.lineStyle(1, 0xffffff);
        line.moveTo(i.x * SCALER_CONST, -i.y * SCALER_CONST);

        const line_end = calculateNewPoint(i.x, i.y, i.angle, 100);
        line.lineTo(line_end.x * SCALER_CONST, -line_end.y * SCALER_CONST);

        line.x = res_x / 2;
        line.y = res_y / 2;
        app.stage.addChild(line);
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

    setAngle(out.laser_beams.slice(-1)[0].angle);
  });
}
