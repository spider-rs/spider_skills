Strategy for text-based CAPTCHAs and distorted text challenges:

**CRITICAL RULE: After 2 failed submissions, STOP trying to read the current text. Click the refresh/reload button (circular arrow ↻ icon near the CAPTCHA image) to get a completely new CAPTCHA. Then read the new text fresh.**

1. **First attempt**: Read the distorted text carefully from the screenshot. Record your reading in memory_ops: `{"op":"set","key":"captcha_attempt_count","value":1}`
2. **Enter and submit**: Fill the input and click Submit
3. **If it fails (same page, same level)**: Increment attempt count in memory. Try ONE alternative reading changing the most ambiguous character. Common confusions: O↔D↔0, S↔5, I↔1↔L, Z↔2, B↔8, G↔6
4. **If that also fails (attempt_count >= 2)**: DO NOT keep guessing. Instead:
   - Use Evaluate to find and click the refresh button:
     ```js
     const refresh = document.querySelector('[class*=refresh], [class*=reload], button svg, .captcha-refresh, img[alt*=refresh]');
     if (refresh) refresh.click();
     else document.querySelectorAll('button, [onclick]').forEach(el => { if (el.textContent.includes('↻') || el.innerHTML.includes('refresh')) el.click(); });
     document.title = 'REFRESHED_CAPTCHA';
     ```
   - Or click the circular arrow icon visible near the CAPTCHA image using ClickPoint
   - Wait 1 second for the new CAPTCHA to load
   - Reset attempt_count to 0 and read the NEW text
5. **Never submit the same text twice**. Check memory for previous attempts before submitting.

Key pitfalls:
- You WILL misread distorted text. That's expected. The refresh button is your escape hatch.
- NEVER keep trying variations of the same misread for more than 2 attempts total.
- After refreshing, you get completely different text. Read it fresh, don't carry over old readings.
- The circular arrow (↻) icon near the CAPTCHA image is the refresh button.
