# UX Standard — Frontend / UX Engineer

**Perspective:** First user. Is this usable, accessible, consistent, and performant in the browser?

References: WCAG 2.2 AA, Core Web Vitals (LCP, INP, CLS).

---

## 1. Accessibility (WCAG 2.2 AA minimum)

- [ ] All interactive elements are keyboard-navigable (Tab, Enter, Space, arrow keys)
- [ ] Focus is visible — no `outline: none` without a custom focus indicator
- [ ] Focus management correct: modals trap focus; focus returns to trigger on close
- [ ] Colour contrast ≥ 4.5:1 for normal text, ≥ 3:1 for large text (≥ 18pt or bold ≥ 14pt)
- [ ] Images have meaningful `alt` text; decorative images use `alt=""`
- [ ] ARIA roles used correctly — not adding `role="button"` to a `<button>` (redundant), not omitting roles on custom interactive elements
- [ ] Form fields have associated `<label>` elements (via `for`/`id` or wrapping)
- [ ] Error messages are programmatically associated with fields via `aria-describedby`
- [ ] Dynamic content changes announced via `aria-live` regions

## 2. User Experience

- [ ] Loading states handled — skeleton, spinner, or disabled state during async operations
- [ ] Error states user-friendly: clear message (what went wrong) + recovery action (what to do next)
- [ ] Empty states handled and informative — not a blank screen
- [ ] Destructive actions (delete, overwrite) require confirmation
- [ ] Forms preserve user input on validation failure — don't wipe the form on a bad submission
- [ ] Long operations show progress, not a frozen UI

## 3. Visual Consistency

- [ ] Design system tokens / components used — no ad-hoc hex values or one-off font sizes
- [ ] Spacing, typography, and colour follow the established design system
- [ ] Responsive at relevant breakpoints (mobile 320px+, tablet 768px+, desktop 1024px+)
- [ ] Dark mode / theming not broken (if the project supports it)

## 4. Frontend Performance (Core Web Vitals)

- [ ] No unnecessary re-renders — React: `memo`, `useMemo`, `useCallback` where measured to help
- [ ] Images: use WebP/AVIF, lazy-load below-the-fold images, provide correct `width`/`height` to prevent CLS
- [ ] Bundle size impact assessed — no large library (moment.js, lodash) imported for one utility function
- [ ] Critical rendering path not blocked by synchronous scripts in `<head>`
- [ ] LCP (Largest Contentful Paint) target ≤ 2.5s; CLS (Cumulative Layout Shift) ≤ 0.1; INP ≤ 200ms

## 5. Security (Frontend-specific)

- [ ] User-generated content is sanitised before rendering as HTML (no `dangerouslySetInnerHTML` with unsanitised strings)
- [ ] Sensitive data (tokens, user details) not stored in `localStorage` / `sessionStorage`
- [ ] Content Security Policy not weakened by this change
- [ ] Auth tokens in `HttpOnly` cookies, not JS-accessible storage

## 6. Internationalisation (if project supports i18n)

- [ ] All user-visible strings extracted to i18n keys — no hardcoded English in JSX/templates
- [ ] Date, time, and number formatting uses locale-aware functions (`Intl.DateTimeFormat`, `toLocaleString`)
- [ ] RTL layout not broken if the project targets RTL languages

## Output Format

```
[BLOCKER] components/Modal.tsx:34 — focus is not trapped inside the modal.
Screen reader users and keyboard users can tab to elements behind the overlay.
Use a focus trap library or implement manually with the inert attribute on the
background content.

[MAJOR] The new "Delete Account" button triggers immediately on click with no
confirmation step. This is irreversible data loss. Add a confirmation dialog.

[SUGGESTION] The loading spinner uses an inline SVG with no aria-label or
aria-busy="true" on the container. Screen readers have no indication that
content is loading. Add role="status" aria-label="Loading..." to the container.
```
