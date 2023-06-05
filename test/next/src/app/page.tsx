"use client";

import { useCallback, useEffect, useRef, useState } from "react";
import init, { manny_circle_set } from "chaos_theory";
import { cn } from "@/lib/utils";
import { render, initCanvas } from "@/lib/renderer";
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "../../components/ui/accordion";
import { Button } from "../../components/ui/button";
import { Input } from "../../components/ui/input";
import type { Settings, MassSetCircles } from "@/types/main.d";
export default function Home() {
  const [angleCalculated, setAngleCalculated] = useState<number>(0);
  const simCanvas = useRef<HTMLCanvasElement>(null);

  // simulation settings
  const [settings, setSettings] = useState<Settings | null>(null);

  const [massSetCircles, setMassSetCircles] = useState<MassSetCircles | null>(
    null
  );

  // make canvas full screen
  useEffect(() => {
    function resizer() {
      const canvas = simCanvas.current;
      if (!canvas) return;
      canvas.width = window.innerWidth;
      canvas.height = window.innerHeight;

      if (!settings) return;
      render(settings, setAngleCalculated);
    }
    window.addEventListener("resize", resizer);

    return () => {
      window.removeEventListener("resize", resizer);
    };
  }, [settings, simCanvas]);

  // init canvas
  useEffect(() => {
    initCanvas(simCanvas);
    setMassSetCircles({
      circleAmountX: 2,
      circleAmountY: 2,
      circleRadius: 1,
      circleSpacing: 3,
    });
  }, []);

  // create circles and before render
  useEffect(() => {
    if (massSetCircles === null) return;
    init().then(() => {
      manny_circle_set(
        massSetCircles.circleAmountX,
        massSetCircles.circleAmountY,
        massSetCircles.circleSpacing,
        massSetCircles.circleRadius
      );
    });
  }, [massSetCircles]);

  useEffect(() => {
    if (!settings) return;
    render(settings, setAngleCalculated);
  }, [settings]);

  return (
    <main>
      <div className={cn("fixed text-white m-4")}>
        <p>Angle: {angleCalculated}</p>
        <p>Random Number: {angleCalculated / 360}</p>
      </div>
      <form
        action=""
        className={cn("fixed right-0 text-white m-4")}
        onSubmit={(e) => {
          e.preventDefault();

          // get data from form
          // for mass set circles
          let circleAmountX: number | undefined = Number(
            (e.target as HTMLFormElement).circleAmountX?.value
          );
          if (circleAmountX === undefined) {
            if (!massSetCircles?.circleAmountX) {
              circleAmountX = 0;
            } else {
              circleAmountX = massSetCircles.circleAmountX;
            }
          }
          let circleAmountY: number | undefined = Number(
            (e.target as HTMLFormElement).circleAmountY?.value
          );
          if (circleAmountY === undefined) {
            if (!massSetCircles?.circleAmountY) {
              circleAmountY = 0;
            } else {
              circleAmountY = massSetCircles.circleAmountY;
            }
          }
          let circleRadius: number | undefined = Number(
            (e.target as HTMLFormElement).circleRadius?.value
          );
          if (circleRadius === undefined) {
            if (!massSetCircles?.circleRadius) {
              circleRadius = 0;
            } else {
              circleRadius = massSetCircles.circleRadius;
            }
          }
          let circleSpacing = Number(
            (e.target as HTMLFormElement).circleSpacing?.value
          );
          if (circleSpacing === undefined) {
            if (!massSetCircles?.circleSpacing) {
              circleSpacing = 0;
            } else {
              circleSpacing = massSetCircles.circleSpacing;
            }
          }
          // for settings
          let zoom = Number((e.target as HTMLFormElement).zoom?.value);
          let ini_x = Number((e.target as HTMLFormElement).ini_x?.value);
          let ini_y = Number((e.target as HTMLFormElement).ini_y?.value);
          let ini_angle = Number(
            (e.target as HTMLFormElement).ini_angle?.value
          );
          let reflectionsNum: number | bigint | undefined = Number(
            (e.target as HTMLFormElement).reflectionsNum?.value
          );
          if (reflectionsNum === undefined || isNaN(reflectionsNum)) {
            if (!settings?.reflectionsNum) {
              reflectionsNum = 20n;
            } else {
              reflectionsNum = settings.reflectionsNum;
            }
          } else {
            reflectionsNum = BigInt(reflectionsNum);
          }

          setMassSetCircles({
            circleAmountX,
            circleAmountY,
            circleRadius,
            circleSpacing,
          });
          setSettings({
            zoom,
            ini_x,
            ini_y,
            ini_angle,
            reflectionsNum,
          });
        }}
      >
        <h1 className="p-0 pb-2">Set Circles</h1>

        <div className="space-y-1 mb-2">
          <label htmlFor="circleAmountX">Amount of circles in X</label>
          <Input
            type="number"
            name="circleAmountX"
            id="circleAmountX"
            placeholder="Circle Amount X"
            defaultValue={massSetCircles?.circleAmountX || ""}
          ></Input>
        </div>
        <div className="space-y-1 mb-2">
          <label htmlFor="circleAmountY">Amount of circles in Y</label>
          <Input
            type="number"
            name="circleAmountY"
            id="circleAmountY"
            placeholder="Circle Amount Y"
            defaultValue={massSetCircles?.circleAmountY || ""}
          ></Input>
        </div>
        <div className="space-y-1 mb-2">
          <label htmlFor="circleRadius">Radius of the circles</label>
          <Input
            type="number"
            name="circleRadius"
            id="circleRadius"
            placeholder="Circles radius"
            defaultValue={massSetCircles?.circleRadius || ""}
            step={0.01}
          ></Input>
        </div>
        <div className="space-y-1">
          <label htmlFor="circleRadius">Spacing between circles</label>
          <Input
            type="number"
            name="circleSpacing"
            id="circleSpacing"
            placeholder="Circles spacing"
            defaultValue={massSetCircles?.circleSpacing || ""}
            step={0.01}
          ></Input>
        </div>

        <h1 className="p-0 pb-2">Lasser settings</h1>

        <div className="space-y-1 mb-2">
          <label htmlFor="ini_angle">Set angle of the lasser</label>
          <Input
            type="number"
            name="ini_angle"
            id="ini_angle"
            placeholder="Angle"
            defaultValue={settings?.ini_angle || ""}
            step={0.00000001}
          ></Input>
        </div>
        <div className="space-y-1 mb-2">
          <label htmlFor="ini_x">Innitial X of the laser</label>
          <Input
            type="number"
            name="ini_x"
            id="ini_x"
            placeholder="Innitial X"
            defaultValue={settings?.ini_x || ""}
            step={0.01}
          ></Input>
        </div>
        <div className="space-y-1 mb-2">
          <label htmlFor="ini_y">Innitial Y of the laser</label>
          <Input
            type="number"
            name="ini_y"
            id="ini_y"
            placeholder="Innitial Y"
            defaultValue={settings?.ini_y || ""}
            step={0.01}
          ></Input>
        </div>

        <h1 className="p-0 pb-2">Misc.</h1>

        <div className="space-y-1 mb-2">
          <label htmlFor="reflectionsNum">
            Maxiumum amounth of reflections
          </label>
          <Input
            type="number"
            name="reflectionsNum"
            id="reflectionsNum"
            placeholder="Maxiumum amounth of reflections"
            defaultValue={Number(settings?.reflectionsNum || 20) || ""}
            step={1}
          ></Input>
        </div>
        <div className="space-y-1 mb-2">
          <label htmlFor="zoom">Zoom</label>
          <Input
            type="number"
            name="zoom"
            id="zoom"
            placeholder="Zoom"
            defaultValue={settings?.zoom || ""}
            step={0.01}
          ></Input>
        </div>

        <div className="flex w-full justify-end">
          <Button className="mt-4" variant="outline">
            Render
          </Button>
        </div>
      </form>
      <canvas ref={simCanvas} />
    </main>
  );
}
