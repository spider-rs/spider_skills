Strategy for tic-tac-toe (XOXO) challenges:

**GOAL: Play optimally with 1 Evaluate + 1 Click per move. Read board, decide, act - all in one round.**

Each round:
1. **Read board + decide + click in one round**. Use Evaluate to read board state:
   ```js
   const cells = [...document.querySelectorAll('[class*=square], [class*=cell], [class*=tile], td')];
   const board = cells.map(el => el.textContent.trim() || (el.querySelector('[class*=x]') ? 'X' : el.querySelector('[class*=o]') ? 'O' : ''));
   document.title = 'BOARD:' + board.join(',');
   ```
2. **In the SAME round**, click the best empty cell immediately. Don't waste a round just reading.
3. **Optimal play** (priority order): Win > Block > Center > Corner > Edge
4. **After you win** (3 in a row), click Verify/Submit immediately in the same round.
5. **After each move**, the opponent may respond. Next round: read updated board + click again.

Key rules:
- 1 round = 1 Evaluate + 1 Click. Never spend a round only gathering data.
- Win lines: [0,1,2], [3,4,5], [6,7,8], [0,3,6], [1,4,7], [2,5,8], [0,4,8], [2,4,6]
- Center=4, Corners=0,2,6,8
