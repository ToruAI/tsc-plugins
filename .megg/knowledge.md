---
created: 2026-01-15T09:42:22.948Z
updated: 2026-01-15T09:42:22.951Z
type: knowledge
---

# Knowledge


---

## 2026-01-15 - TSC plugins inherit styles from host
**Type:** pattern
**Topics:** tsc-plugins, frontend, styling, tailwind

TSC plugin frontends render inside the host DOM (`PluginView.tsx`), so they **inherit all CSS**:

- All `:root` CSS variables from TSC's `index.css`
- Tailwind utility classes (TSC has Tailwind loaded)
- Dark mode (`.dark` class on `<html>`)
- Montserrat font

**No need to bundle CSS in plugins.** Just use Tailwind classes directly:
```tsx
<div className="bg-background text-foreground">
  <button className="bg-primary">Works!</button>
</div>
```

For components, use shadcn MCP to look up patterns and ensure consistency with TSC.
