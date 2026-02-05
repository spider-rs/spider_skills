Strategy for word search grid challenges:

**GOAL: Extract grid + find ALL words in one Evaluate, then select each word quickly.**

Round 1 - Extract and solve:
1. **Use a single Evaluate to extract grid, find all words, and report positions**:
   ```js
   const cells = [...document.querySelectorAll('[class*=cell], [class*=letter], [class*=grid] > *')];
   const rects = cells.map(c => c.getBoundingClientRect());
   const uniqueTops = [...new Set(rects.map(r => Math.round(r.top)))].sort((a,b) => a-b);
   const rows = uniqueTops.length, cols = Math.round(cells.length / rows);
   const letters = cells.map(c => c.textContent.trim().toUpperCase());
   const grid = []; for (let r = 0; r < rows; r++) grid.push(letters.slice(r*cols,(r+1)*cols));
   const words = [...document.querySelectorAll('[class*=word], [class*=clue], li')].map(el => el.textContent.trim().toUpperCase()).filter(w => w.length > 1);
   const dirs = [[0,1],[0,-1],[1,0],[-1,0],[1,1],[1,-1],[-1,1],[-1,-1]];
   const found = {};
   words.forEach(w => { for(let r=0;r<rows;r++) for(let c=0;c<cols;c++) for(const[dr,dc] of dirs) {
     let ok=true; const idxs=[]; for(let k=0;k<w.length;k++) { const nr=r+dr*k,nc=c+dc*k;
       if(nr<0||nr>=rows||nc<0||nc>=cols||grid[nr][nc]!==w[k]){ok=false;break;} idxs.push(nr*cols+nc); }
     if(ok){found[w]=idxs;return;}
   }});
   document.title = 'FOUND:' + JSON.stringify(found);
   ```
2. **Select and submit each word** using cell indices from the result. For each word:
   ```js
   const cells = [...document.querySelectorAll('[class*=cell], [class*=letter], [class*=grid] > *')];
   [i1, i2, i3, ...].forEach(i => cells[i]?.click());
   ```
   Then click Submit. If drag-selection is needed, use ClickDragPoint from first to last cell.

Key rules:
- Solve the word search algorithmically in JS, don't search visually.
- Words go in 8 directions including diagonal and backwards.
- Select one word at a time, submit, then next word.
