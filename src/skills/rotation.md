Strategy for rotation/orientation challenges:

1. **Identify the rotation mechanism** using Evaluate:
   ```js
   document.title = JSON.stringify({
     sliders: [...document.querySelectorAll('input[type=range], [class*=slider], [class*=knob]')].map(el => ({
       tag: el.tagName, cls: el.className, val: el.value, min: el.min, max: el.max
     })),
     rotated: [...document.querySelectorAll('[style*=transform], [style*=rotate]')].map(el => ({
       cls: el.className, style: el.style.cssText.slice(0,100)
     }))
   })
   ```
2. **Determine current rotation** from transform styles or slider value
3. **Calculate target**: The image should be upright (0° or 360°). If current rotation is X°, you need to adjust by -X° (or 360-X°)
4. **Interact with the slider/control**:
   - For range inputs: `{ "Fill": { "selector": "input[type=range]", "value": "TARGET" } }` then trigger change event via Evaluate
   - For drag controls: Use `ClickDragPoint` to drag the handle to the correct position
   - For click-to-rotate buttons: Click the rotation button the correct number of times
5. **After adjusting, verify visually** from the screenshot that the image looks upright
6. **Then click Verify/Submit**

Key pitfalls:
- Range input `Fill` may not fire change events. Follow with: `el.dispatchEvent(new Event('input', {bubbles:true})); el.dispatchEvent(new Event('change', {bubbles:true}))`
- Slider position may map non-linearly to degrees. Check min/max/step attributes.
- Some rotation controls use click-and-drag rather than slider inputs.
