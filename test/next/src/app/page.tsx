"use client";

import { useEffect, useLayoutEffect, useRef, useState } from "react";
import init, { manny_circle_set } from "chaos_theory";
import { render, initCanvas } from "@/renderer";
import type { Settings, MassSetCircles } from "@/types/main.d";

export default function Home() {
  const [angleCalculated, setAngleCalculated] = useState<number>(0);
  const simCanvas = useRef<HTMLCanvasElement>(null);

  // make canvas full screen
  useEffect(() => {
    function resizer() {
      const canvas = simCanvas.current;
      if (!canvas) return;
      canvas.width = window.innerWidth;
      canvas.height = window.innerHeight;
    }

    window.addEventListener("resize", resizer);

    return () => {
      window.removeEventListener("resize", resizer);
    };
  }, [simCanvas]);

  // simulation settings
  const [settings, setSettings] = useState<Settings | null>(null);

  const [massSetCircles, setMassSetCircles] = useState<MassSetCircles | null>(
    null
  );

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

      setSettings({
        zoom: 20,
        ini_x: 0,
        ini_y: 0,
        ini_angle: 0,
        reflectionsNum: 20n,
      });
    });
  }, [massSetCircles]);

  useEffect(() => {
    if (settings === null) return;
    render(settings, setAngleCalculated);
  }, [settings]);

  return (
    <main>
      <div className="fixed text-white">
        <p>Angle: {angleCalculated}</p>
        <p>Random Number: {angleCalculated / 360}</p>
      </div>
      <form action="">
        <label htmlFor="circleAmountX">Circle Amount X</label>
        <input type="number"></input>
        <label htmlFor="circleAmountY">Circle Amount Y</label>
        <input type="number"></input>
      </form>
      <canvas ref={simCanvas} />
    </main>
  );
}
