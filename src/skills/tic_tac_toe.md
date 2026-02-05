Strategy for tic-tac-toe (XOXO) challenges:

1. **ALWAYS read the board state from DOM before every move** using Evaluate:
   ```js
   document.title = 'BOARD:' + JSON.stringify(
     [...document.querySelectorAll('[class*=square], [class*=cell], [class*=tile], td')].map((el,i) => ({
       i, text: el.textContent.trim(), cls: el.className,
       hasX: el.textContent.includes('X') || el.querySelector('[class*=x]') !== null,
       hasO: el.textContent.includes('O') || el.querySelector('[class*=o]') !== null,
       empty: el.textContent.trim() === '' && !el.querySelector('[class*=x],[class*=o]')
     }))
   )
   ```
2. **Determine your piece** (X or O) by counting existing pieces. If equal counts, you're X (go first). If X has one more, you're O.
3. **Play optimal tic-tac-toe strategy**:
   - **Win**: If you can win in one move, take it immediately
   - **Block**: If opponent can win in one move, block it
   - **Center**: Take center (index 4) if available
   - **Corners**: Take corners (0, 2, 6, 8) over edges
   - **Fork**: Create two-way win threats when possible
4. **Click the chosen empty cell** using its selector or coordinates
5. **NEVER click Verify/Submit until you've confirmed a win** via Evaluate:
   ```js
   document.title = 'WIN_CHECK:' + JSON.stringify({
     cells: [...document.querySelectorAll('[class*=square], [class*=cell], td')].map(el => el.textContent.trim()),
     lines: [[0,1,2],[3,4,5],[6,7,8],[0,3,6],[1,4,7],[2,5,8],[0,4,8],[2,4,6]]
   })
   ```
   Then check if any line has three of your piece.
6. **After confirming win, THEN click Verify**

Key pitfalls:
- NEVER assume you've won from the screenshot alone. Always verify via DOM.
- The opponent moves after you. Wait for and read the updated board before your next move.
- If you click Verify without actually winning, the challenge may reset or fail.
- Cells may use images/SVGs instead of text - check class names and child elements.
