Strategy for image grid selection challenges (e.g., "select all stop signs"):

1. **Read the prompt carefully** - identify exactly what to select (stop signs, vehicles, crosswalks, etc.)
2. **Use Evaluate to inspect grid state** before clicking:
   ```js
   document.title = JSON.stringify([...document.querySelectorAll('[class*=grid] > *, [class*=item]')].map((el,i) => ({
     i, cls: el.className, sel: el.classList.contains('selected') || el.getAttribute('aria-checked')==='true',
     alt: el.querySelector('img')?.alt || '', src: el.querySelector('img')?.src?.slice(-30) || ''
   })))
   ```
3. **First deselect any already-selected items** that don't match, then select matching ones
4. **Click items by their CSS selector or coordinates** - prefer selectors when available
5. **After selecting, use Evaluate to verify selection state** before clicking Verify/Submit
6. **If Verify fails**: read the grid state again - items may have toggled. Deselect wrong ones, select correct ones, then retry.

Key pitfalls:
- Clicking a selected item DESELECTS it. Never click an already-correct selection.
- The grid may shuffle after a failed verify attempt.
- Use the screenshot + alt text + class names to identify which tiles match.
