Strategy for text-based CAPTCHAs and distorted text challenges:

1. **Read the distorted text carefully** from the screenshot. Each character may be warped, rotated, or overlapping.
2. **Record your first reading** in memory_ops (e.g., `{"op":"set","key":"captcha_reading","value":"FNZZSD"}`)
3. **Enter the text** using Fill and click Submit
4. **If submission fails (page doesn't advance), try alternative character readings**:
   - Common confusions: O↔D↔0, S↔5↔$, I↔1↔L↔l, Z↔2, B↔8, G↔6, Q↔O, U↔V
   - Track which readings you've already tried in memory_ops
   - Change ONE character at a time, starting with the least certain
   - After each failed attempt, re-examine the screenshot for the ambiguous characters
5. **If stuck after 3+ failed attempts**: look for a refresh/reload button (circular arrow icon) to get a new CAPTCHA, then try again with fresh text
6. **Use Evaluate to check what's currently in the input field**:
   ```js
   document.title = 'INPUT:' + document.querySelector('input[type=text], input.captcha-input-text, input')?.value;
   ```

Key pitfalls:
- NEVER resubmit the exact same text that already failed. Always change at least one character.
- Distorted text uses visual tricks: overlapping lines, warped letterforms, noise backgrounds.
- The text is case-sensitive in some challenges. Try both upper and lower case if stuck.
- Look for a refresh button to get an easier CAPTCHA if the current one is too ambiguous.
