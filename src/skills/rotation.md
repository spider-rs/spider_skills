Strategy for rotation/orientation challenges:

**GOAL: Solve in 1-2 rounds. Read rotation state, set correct value, submit.**

Round 1 - Read and fix in one shot:
1. **Use Evaluate to read current rotation AND set the correct value**:
   ```js
   const slider = document.querySelector('input[type=range], [class*=slider], [role=slider]');
   const rotated = document.querySelector('[style*=rotate]');
   const match = rotated?.style.cssText.match(/rotate\((-?\d+\.?\d*)deg\)/);
   const currentDeg = match ? parseFloat(match[1]) : 0;
   const targetSlider = slider ? Math.round((360 - (currentDeg % 360 + 360) % 360) % 360 / 360 * (slider.max - slider.min) + Number(slider.min)) : 0;
   if (slider) { slider.value = targetSlider; slider.dispatchEvent(new Event('input',{bubbles:true})); slider.dispatchEvent(new Event('change',{bubbles:true})); }
   document.title = 'ROT:' + currentDeg + ' TARGET_SLIDER:' + targetSlider;
   ```
2. **Click Verify/Submit** in the same round.
3. If the image doesn't look upright, use the screenshot to estimate degrees off and adjust.

Key rules:
- Read the rotation from `transform: rotate(Xdeg)` style.
- For range sliders: set value programmatically via Evaluate with events.
- For drag handles: use ClickDragPoint.
- Don't spend rounds just reading - read and act together.
