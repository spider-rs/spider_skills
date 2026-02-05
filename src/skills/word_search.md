Strategy for word search grid challenges:

1. **Extract the full grid and word list** using Evaluate:
   ```js
   const cells = [...document.querySelectorAll('[class*=cell], [class*=letter], [class*=grid] > *')];
   const rects = cells.map(c => c.getBoundingClientRect());
   // Auto-detect grid dimensions from positions
   const uniqueTops = [...new Set(rects.map(r => Math.round(r.top)))].sort((a,b) => a-b);
   const rows = uniqueTops.length;
   const cols = Math.round(cells.length / rows);
   const letters = cells.map(c => c.textContent.trim());
   const grid = [];
   for (let r = 0; r < rows; r++) grid.push(letters.slice(r * cols, (r + 1) * cols).join(''));
   // Get word list
   const words = [...document.querySelectorAll('[class*=word], [class*=clue], li')].map(el => el.textContent.trim()).filter(w => w.length > 1);
   document.title = 'GRID:' + JSON.stringify({ rows, cols, grid, words, cellCount: cells.length });
   ```
2. **Search for each word** in all 8 directions (right, left, down, up, and 4 diagonals)
3. **Deselect any previously selected cells** before selecting new ones:
   ```js
   document.querySelectorAll('[class*=selected], [aria-checked=true]').forEach(el => el.click());
   ```
4. **Select found word cells programmatically** by computing cell indices and clicking them:
   ```js
   const cells = [...document.querySelectorAll('[class*=cell], [class*=letter], [class*=grid] > *')];
   [idx1, idx2, idx3, ...].forEach(i => cells[i]?.click());
   ```
5. **After selecting a word, click Submit/Verify** for that word, then repeat for the next word
6. **If selection doesn't work via click**: try ClickDragPoint from first letter to last letter of the word

Key pitfalls:
- Grid dimensions vary. ALWAYS auto-detect rows/cols from element positions, never hardcode.
- Words can go in ANY direction including backwards and diagonal.
- Some word search UIs require drag-selection (mousedown on first letter, mousemove through, mouseup on last letter) rather than individual cell clicks.
- After finding a word, the grid or word list may update. Re-read state after each word.
- Clicking already-selected cells may deselect them. Track selection state.
