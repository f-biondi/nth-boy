import { Emulator } from "nth-boy-wasm";
import { memory } from "nth-boy-wasm/nth_boy_wasm_bg.wasm";

var emulator = null;
var rom_name = null;
var startTime = performance.now();
var frames = 0;

const width = 160;
const height = 144;
const SCALE = 4;
const canvas = document.getElementById("lcd-canvas");
const romSelect = document.getElementById("rom-select");
const rom = document.getElementById("rom");
const fps = document.getElementById("fps");
const ctx = canvas.getContext('2d');
const palette = {
    0x00: 0xFF000000,
    0x55: 0xFF555555,
    0xAA: 0xFFAAAAAA,
    0xFF: 0xFFFFFFFF,
};
canvas.height = SCALE * height;
canvas.width = SCALE * width;

romSelect.addEventListener("click", (e) => {
    romSelect.blur();
    rom.click();
});

rom.addEventListener("change", (e) => {
    if (rom.files.length > 0) {
        let reader = new FileReader();
        reader.readAsArrayBuffer(rom.files[0]);
        reader.onload = function(){
            let arrayBuffer = reader.result
            let bytes = new Uint8Array(arrayBuffer);
            saveData();
            emulator = null;
            setTimeout(() => {
                rom_name = rom.files[0].name;
                let ram = localStorage.getItem(rom_name + ".sav");
                let rtc = localStorage.getItem(rom_name + ".rtc");
                emulator = Emulator.new(
                    bytes,
                    ram != null ? new Uint8Array(JSON.parse(ram)) : new Uint8Array(),
                    rtc != null ? new Uint8Array(JSON.parse(rtc)) : new Uint8Array(),
                );
                requestAnimationFrame(renderLoop);
            }, 100);
        }
    }

});

window.addEventListener("beforeunload", (e) => {
    saveData();
});

document.addEventListener("keydown", (e) => {
  switch(e.key) {
      case "w":
        emulator.set_up();
        break;
      case "s":
        emulator.set_down();
        break;
      case "a":
        emulator.set_left();
        break;
      case "d":
        emulator.set_right();
        break;
      case "j":
        emulator.set_a();
        break;
      case "k":
        emulator.set_b();
        break;
      case "Enter":
        emulator.set_start();
        break;
      case "Backspace":
        emulator.set_select();
        break;
  }
});

document.addEventListener("keyup", (e) => {
  switch(e.key) {
      case "w":
        emulator.unset_up();
        break;
      case "s":
        emulator.unset_down();
        break;
      case "a":
        emulator.unset_left();
        break;
      case "d":
        emulator.unset_right();
        break;
      case "j":
        emulator.unset_a();
        break;
      case "k":
        emulator.unset_b();
        break;
      case "Enter":
        emulator.unset_start();
        break;
      case "Backspace":
        emulator.unset_select();
        break;
  }
});

const saveData = () => {
    if(emulator != null) {
        let ram = emulator.dump_ram();
        let rtc = emulator.dump_rtc();
        if(ram.length > 0) { 
            localStorage.setItem(rom_name + ".sav", JSON.stringify(Array.from(ram)));
        }
        if(rtc.length > 0) { 
            localStorage.setItem(rom_name + ".rtc", JSON.stringify(Array.from(rtc)));
        }
    }
};

const renderLoop = () => {
  if (emulator != null) {
      let startFrame = performance.now();
      emulator.next_frame();

      drawFrame();
      frames++;

      let now = performance.now();

      if ((now-startTime) >= 1000) {
          startTime = performance.now();
          fps.innerHTML = frames + " FPS";
          frames=0;
      }

      requestAnimationFrame(renderLoop);
  }
};

const drawFrame = () => {
  const framePtr = emulator.buffer();
  const pixels = new Uint8Array(memory.buffer);
  const imageData = ctx.createImageData(width*SCALE, height*SCALE);
  const data = new Uint32Array(imageData.data.buffer);

  for(let r=0; r<(height*SCALE); ++r) {
    for(let c=0; c<(width*SCALE); ++c) {
        let i = Math.floor(c/SCALE) + (Math.floor(r/SCALE) * width);
        let color = palette[pixels[framePtr + (i * 4)]];
        data[(r*width*SCALE) + c] = color;
    }
  }
  
  ctx.putImageData(imageData, 0, 0);
};
